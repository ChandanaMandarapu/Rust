// ============================================================================
// AES-256-GCM — COMPLETE FROM-SCRATCH IMPLEMENTATION
// No crates. No lookup tables from the internet. Every constant derived here.
//
// AES: FIPS 197
// GCM: NIST SP 800-38D
//
// This file implements:
//   - GF(2^8) arithmetic (the mathematical foundation)
//   - AES S-Box generation at runtime from GF(2^8) inverses + affine transform
//   - AES key schedule (256-bit)
//   - AES encryption/decryption (ECB block)
//   - CTR mode streaming
//   - GHASH (GCM authentication)
//   - Full AES-256-GCM AEAD with constant-time tag verification
// ============================================================================

use core::ptr;

// ---------------------------------------------------------------------------
// GF(2^8) — Galois Field arithmetic with irreducible polynomial x^8+x^4+x^3+x+1
// AES uses this field for ALL its operations
// ---------------------------------------------------------------------------

/// Multiply two elements in GF(2^8) using the AES irreducible polynomial.
/// This is the fundamental building block of literally everything in AES.
/// Uses the "Russian peasant multiplication" algorithm — no lookup tables.
#[inline(always)]
pub const fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    let mut i = 0u8;
    while i < 8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1B; // x^8 + x^4 + x^3 + x + 1 reduced (0x11B & 0xFF = 0x1B)
        }
        b >>= 1;
        i += 1;
    }
    p
}

/// Compute GF(2^8) multiplicative inverse via Fermat's little theorem.
/// In GF(2^8), a^(2^8 - 1) = 1, so a^(-1) = a^(2^8 - 2) = a^254.
/// We compute this via repeated squaring.
#[inline(always)]
pub const fn gf_inv(a: u8) -> u8 {
    if a == 0 {
        return 0; // 0 has no inverse; AES defines inv(0) = 0
    }
    // a^254 via square-and-multiply
    let a2   = gf_mul(a, a);       // a^2
    let a4   = gf_mul(a2, a2);     // a^4
    let a8   = gf_mul(a4, a4);     // a^8
    let a16  = gf_mul(a8, a8);     // a^16
    let a32  = gf_mul(a16, a16);   // a^32
    let a64  = gf_mul(a32, a32);   // a^64
    let a128 = gf_mul(a64, a64);   // a^128
    // 254 = 128 + 64 + 32 + 16 + 8 + 4 + 2
    let t = gf_mul(a128, a64);
    let t = gf_mul(t, a32);
    let t = gf_mul(t, a16);
    let t = gf_mul(t, a8);
    let t = gf_mul(t, a4);
    gf_mul(t, a2)
}

/// AES affine transformation applied after GF inverse to produce S-Box value.
/// This is a bit matrix multiply in GF(2) + constant 0x63.
/// The matrix is circular shifts of 10001111:
///   b_i = a_i ^ a_{i+4} ^ a_{i+5} ^ a_{i+6} ^ a_{i+7} (mod 8) + c_i
#[inline(always)]
const fn affine(x: u8) -> u8 {
    let x = x as u32;
    // 8 circular shifts XORed together
    let y = x
        ^ x.rotate_left(1) & 0xFF
        ^ x.rotate_left(2) & 0xFF
        ^ x.rotate_left(3) & 0xFF
        ^ x.rotate_left(4) & 0xFF;
    ((y ^ 0x63) & 0xFF) as u8
}

// ---------------------------------------------------------------------------
// S-Box and Inverse S-Box — generated at compile time (const fn)
// ---------------------------------------------------------------------------

/// Generate the full 256-entry AES SubBytes S-Box.
/// Each entry: S[x] = affine(gf_inv(x))
const fn make_sbox() -> [u8; 256] {
    let mut s = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        s[i] = affine(gf_inv(i as u8));
        i += 1;
    }
    s
}

/// Generate the inverse S-Box for AES decryption.
/// InvS[S[x]] = x for all x
const fn make_inv_sbox(sbox: &[u8; 256]) -> [u8; 256] {
    let mut inv = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        inv[sbox[i] as usize] = i as u8;
        i += 1;
    }
    inv
}

pub const SBOX:     [u8; 256] = make_sbox();
pub const INV_SBOX: [u8; 256] = make_inv_sbox(&SBOX);

// ---------------------------------------------------------------------------
// Round constants (RCON) — powers of 2 in GF(2^8)
// Used in the key schedule
// ---------------------------------------------------------------------------
const fn make_rcon() -> [u32; 15] {
    let mut rcon = [0u32; 15];
    let mut x: u8 = 1;
    let mut i = 0;
    while i < 15 {
        rcon[i] = (x as u32) << 24;
        x = gf_mul(x, 2);
        i += 1;
    }
    rcon
}
pub const RCON: [u32; 15] = make_rcon();

// ---------------------------------------------------------------------------
// MixColumns precomputed tables — the heart of AES diffusion
// MixColumns multiplies each column by the fixed polynomial
// {03}x^3 + {01}x^2 + {01}x + {02} in GF(2^8)[x]/(x^4+1)
// We precompute the 4 possible XTIMEs to avoid runtime GF mul in the hot path
// ---------------------------------------------------------------------------

const fn make_xtime2() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = gf_mul(i as u8, 2);
        i += 1;
    }
    t
}

const fn make_xtime3() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = gf_mul(i as u8, 3);
        i += 1;
    }
    t
}

const fn make_xtime9() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 { t[i] = gf_mul(i as u8, 9); i += 1; }
    t
}
const fn make_xtime11() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 { t[i] = gf_mul(i as u8, 11); i += 1; }
    t
}
const fn make_xtime13() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 { t[i] = gf_mul(i as u8, 13); i += 1; }
    t
}
const fn make_xtime14() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 { t[i] = gf_mul(i as u8, 14); i += 1; }
    t
}

pub const MUL2:  [u8; 256] = make_xtime2();
pub const MUL3:  [u8; 256] = make_xtime3();
pub const MUL9:  [u8; 256] = make_xtime9();
pub const MUL11: [u8; 256] = make_xtime11();
pub const MUL13: [u8; 256] = make_xtime13();
pub const MUL14: [u8; 256] = make_xtime14();

// ---------------------------------------------------------------------------
// AES State — 4x4 byte matrix, stored column-major (AES standard)
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct AesState {
    pub s: [u8; 16], // s[row + 4*col]
}

impl AesState {
    #[inline(always)]
    pub fn from_block(block: &[u8; 16]) -> Self {
        AesState { s: *block }
    }

    #[inline(always)]
    pub fn to_block(&self) -> [u8; 16] {
        self.s
    }

    #[inline(always)]
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.s[row + 4 * col]
    }

    #[inline(always)]
    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.s[row + 4 * col] = val;
    }

    /// SubBytes: replace each byte with its S-Box value
    pub fn sub_bytes(&mut self) {
        for b in &mut self.s {
            *b = SBOX[*b as usize];
        }
    }

    /// InvSubBytes: reverse SubBytes using inverse S-Box
    pub fn inv_sub_bytes(&mut self) {
        for b in &mut self.s {
            *b = INV_SBOX[*b as usize];
        }
    }

    /// ShiftRows: cyclically shift row i left by i positions
    pub fn shift_rows(&mut self) {
        // Row 1: shift left by 1
        let t = self.s[1];
        self.s[1]  = self.s[5];
        self.s[5]  = self.s[9];
        self.s[9]  = self.s[13];
        self.s[13] = t;
        // Row 2: shift left by 2
        self.s.swap(2, 10);
        self.s.swap(6, 14);
        // Row 3: shift left by 3 (= shift right by 1)
        let t = self.s[15];
        self.s[15] = self.s[11];
        self.s[11] = self.s[7];
        self.s[7]  = self.s[3];
        self.s[3]  = t;
    }

    /// InvShiftRows: undo ShiftRows
    pub fn inv_shift_rows(&mut self) {
        // Row 1: shift right by 1
        let t = self.s[13];
        self.s[13] = self.s[9];
        self.s[9]  = self.s[5];
        self.s[5]  = self.s[1];
        self.s[1]  = t;
        // Row 2: shift right by 2
        self.s.swap(2, 10);
        self.s.swap(6, 14);
        // Row 3: shift right by 3 (= shift left by 1)
        let t = self.s[3];
        self.s[3]  = self.s[7];
        self.s[7]  = self.s[11];
        self.s[11] = self.s[15];
        self.s[15] = t;
    }

    /// MixColumns: multiply each column by the MDS matrix in GF(2^8)
    /// The matrix multiplication:
    /// |2 3 1 1|   |s0|
    /// |1 2 3 1| x |s1|
    /// |1 1 2 3|   |s2|
    /// |3 1 1 2|   |s3|
    pub fn mix_columns(&mut self) {
        for col in 0..4 {
            let s0 = self.get(0, col);
            let s1 = self.get(1, col);
            let s2 = self.get(2, col);
            let s3 = self.get(3, col);

            self.set(0, col, MUL2[s0 as usize] ^ MUL3[s1 as usize] ^ s2 ^ s3);
            self.set(1, col, s0 ^ MUL2[s1 as usize] ^ MUL3[s2 as usize] ^ s3);
            self.set(2, col, s0 ^ s1 ^ MUL2[s2 as usize] ^ MUL3[s3 as usize]);
            self.set(3, col, MUL3[s0 as usize] ^ s1 ^ s2 ^ MUL2[s3 as usize]);
        }
    }

    /// InvMixColumns: inverse MDS matrix multiplication
    /// The inverse matrix:
    /// |14 11 13  9|
    /// | 9 14 11 13|
    /// |13  9 14 11|
    /// |11 13  9 14|
    pub fn inv_mix_columns(&mut self) {
        for col in 0..4 {
            let s0 = self.get(0, col);
            let s1 = self.get(1, col);
            let s2 = self.get(2, col);
            let s3 = self.get(3, col);

            self.set(0, col, MUL14[s0 as usize] ^ MUL11[s1 as usize] ^ MUL13[s2 as usize] ^ MUL9[s3 as usize]);
            self.set(1, col, MUL9[s0 as usize]  ^ MUL14[s1 as usize] ^ MUL11[s2 as usize] ^ MUL13[s3 as usize]);
            self.set(2, col, MUL13[s0 as usize] ^ MUL9[s1 as usize]  ^ MUL14[s2 as usize] ^ MUL11[s3 as usize]);
            self.set(3, col, MUL11[s0 as usize] ^ MUL13[s1 as usize] ^ MUL9[s2 as usize]  ^ MUL14[s3 as usize]);
        }
    }

    /// AddRoundKey: XOR state with round key (column-major)
    pub fn add_round_key(&mut self, rk: &[u32; 4]) {
        for col in 0..4 {
            let k = rk[col].to_be_bytes();
            for row in 0..4 {
                self.s[row + 4 * col] ^= k[row];
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AES-256 Key Schedule
// AES-256: 14 rounds, 15 round keys (each 4 words = 128 bits), 60 words total
// ---------------------------------------------------------------------------

pub struct Aes256Key {
    pub round_keys: [[u32; 4]; 15], // 15 round keys of 4 words each
}

impl Aes256Key {
    /// Expand a 32-byte (256-bit) key into the full key schedule.
    /// AES-256 key schedule is more complex than AES-128/192:
    /// every other word uses SubWord (S-Box on all 4 bytes of the word).
    pub fn expand(key: &[u8; 32]) -> Self {
        // Parse key into 8 initial words
        let mut w = [0u32; 60];
        for i in 0..8 {
            w[i] = u32::from_be_bytes([
                key[4 * i],
                key[4 * i + 1],
                key[4 * i + 2],
                key[4 * i + 3],
            ]);
        }

        // Expand remaining 52 words
        for i in 8..60 {
            let mut temp = w[i - 1];
            if i % 8 == 0 {
                // RotWord: rotate left by 8 bits
                temp = temp.rotate_left(8);
                // SubWord: apply S-Box to each byte
                temp = sub_word(temp);
                // XOR with round constant
                temp ^= RCON[i / 8 - 1];
            } else if i % 8 == 4 {
                // AES-256 specific: every 4th word after the rotation also gets SubWord
                temp = sub_word(temp);
            }
            w[i] = w[i - 8] ^ temp;
        }

        // Pack into round keys
        let mut round_keys = [[0u32; 4]; 15];
        for rk in 0..15 {
            round_keys[rk] = [w[4*rk], w[4*rk+1], w[4*rk+2], w[4*rk+3]];
        }

        Aes256Key { round_keys }
    }
}

/// SubWord: apply S-Box to each of the 4 bytes of a u32
#[inline(always)]
fn sub_word(w: u32) -> u32 {
    let b = w.to_be_bytes();
    u32::from_be_bytes([
        SBOX[b[0] as usize],
        SBOX[b[1] as usize],
        SBOX[b[2] as usize],
        SBOX[b[3] as usize],
    ])
}

// ---------------------------------------------------------------------------
// AES-256 Block Cipher — Encryption and Decryption
// ---------------------------------------------------------------------------

pub struct Aes256 {
    key: Aes256Key,
}

impl Aes256 {
    pub fn new(key: &[u8; 32]) -> Self {
        Aes256 { key: Aes256Key::expand(key) }
    }

    /// Encrypt a single 16-byte block using AES-256.
    /// 14 rounds: initial AddRoundKey, 13 full rounds, 1 final round (no MixColumns)
    pub fn encrypt_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state = AesState::from_block(block);
        let rk = &self.key.round_keys;

        // Initial round key addition
        state.add_round_key(&rk[0]);

        // Rounds 1-13: SubBytes + ShiftRows + MixColumns + AddRoundKey
        for round in 1..14 {
            state.sub_bytes();
            state.shift_rows();
            state.mix_columns();
            state.add_round_key(&rk[round]);
        }

        // Final round: no MixColumns
        state.sub_bytes();
        state.shift_rows();
        state.add_round_key(&rk[14]);

        state.to_block()
    }
