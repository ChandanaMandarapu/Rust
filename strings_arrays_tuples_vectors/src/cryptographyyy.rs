use std::num::Wrapping;

struct AES {
    round_keys: Vec<Vec<u8>>,
    nr: usize,
}

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

impl AES {
    fn new(key: &[u8]) -> Self {
        let nk = key.len() / 4;
        let nr = match nk {
            4 => 10,
            6 => 12,
            8 => 14,
            _ => panic!("Invalid key size"),
        };

        let mut aes = AES {
            round_keys: Vec::new(),
            nr,
        };

        aes.key_expansion(key, nk);
        aes
    }

    fn key_expansion(&mut self, key: &[u8], nk: usize) {
        let mut w = vec![0u8; 4 * (self.nr + 1) * 4];
        
        for i in 0..nk {
            w[4*i] = key[4*i];
            w[4*i+1] = key[4*i+1];
            w[4*i+2] = key[4*i+2];
            w[4*i+3] = key[4*i+3];
        }

        for i in nk..(4 * (self.nr + 1)) {
            let mut temp = [w[4*(i-1)], w[4*(i-1)+1], w[4*(i-1)+2], w[4*(i-1)+3]];

            if i % nk == 0 {
                temp = Self::rot_word(temp);
                temp = Self::sub_word(temp);
                temp[0] ^= RCON[i / nk - 1];
            } else if nk > 6 && i % nk == 4 {
                temp = Self::sub_word(temp);
            }

            w[4*i] = w[4*(i-nk)] ^ temp[0];
            w[4*i+1] = w[4*(i-nk)+1] ^ temp[1];
            w[4*i+2] = w[4*(i-nk)+2] ^ temp[2];
            w[4*i+3] = w[4*(i-nk)+3] ^ temp[3];
        }

        for i in 0..=self.nr {
            self.round_keys.push(w[16*i..16*(i+1)].to_vec());
        }
    }

    fn rot_word(word: [u8; 4]) -> [u8; 4] {
        [word[1], word[2], word[3], word[0]]
    }

    fn sub_word(word: [u8; 4]) -> [u8; 4] {
        [SBOX[word[0] as usize], SBOX[word[1] as usize],
         SBOX[word[2] as usize], SBOX[word[3] as usize]]
    }

    fn sub_bytes(state: &mut [u8; 16]) {
        for byte in state.iter_mut() {
            *byte = SBOX[*byte as usize];
        }
    }

    fn inv_sub_bytes(state: &mut [u8; 16]) {
        for byte in state.iter_mut() {
            *byte = INV_SBOX[*byte as usize];
        }
    }

    fn shift_rows(state: &mut [u8; 16]) {
        let temp = *state;
        state[1] = temp[5];
        state[5] = temp[9];
        state[9] = temp[13];
        state[13] = temp[1];
        
        state[2] = temp[10];
        state[6] = temp[14];
        state[10] = temp[2];
        state[14] = temp[6];
        
        state[3] = temp[15];
        state[7] = temp[3];
        state[11] = temp[7];
        state[15] = temp[11];
    }

    fn inv_shift_rows(state: &mut [u8; 16]) {
        let temp = *state;
        state[1] = temp[13];
        state[5] = temp[1];
        state[9] = temp[5];
        state[13] = temp[9];
        
        state[2] = temp[10];
        state[6] = temp[14];
        state[10] = temp[2];
        state[14] = temp[6];
        
        state[3] = temp[7];
        state[7] = temp[11];
        state[11] = temp[15];
        state[15] = temp[3];
    }

    fn gmul(mut a: u8, mut b: u8) -> u8 {
        let mut p = 0u8;
        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }
            let hi_bit_set = a & 0x80 != 0;
            a <<= 1;
            if hi_bit_set {
                a ^= 0x1b;
            }
            b >>= 1;
        }
        p
    }

    fn mix_columns(state: &mut [u8; 16]) {
        for i in 0..4 {
            let s0 = state[i*4];
            let s1 = state[i*4+1];
            let s2 = state[i*4+2];
            let s3 = state[i*4+3];

            state[i*4] = Self::gmul(s0, 2) ^ Self::gmul(s1, 3) ^ s2 ^ s3;
            state[i*4+1] = s0 ^ Self::gmul(s1, 2) ^ Self::gmul(s2, 3) ^ s3;
            state[i*4+2] = s0 ^ s1 ^ Self::gmul(s2, 2) ^ Self::gmul(s3, 3);
            state[i*4+3] = Self::gmul(s0, 3) ^ s1 ^ s2 ^ Self::gmul(s3, 2);
        }
    }

    fn inv_mix_columns(state: &mut [u8; 16]) {
        for i in 0..4 {
            let s0 = state[i*4];
            let s1 = state[i*4+1];
            let s2 = state[i*4+2];
            let s3 = state[i*4+3];

            state[i*4] = Self::gmul(s0, 14) ^ Self::gmul(s1, 11) ^ 
                        Self::gmul(s2, 13) ^ Self::gmul(s3, 9);
            state[i*4+1] = Self::gmul(s0, 9) ^ Self::gmul(s1, 14) ^ 
                          Self::gmul(s2, 11) ^ Self::gmul(s3, 13);
            state[i*4+2] = Self::gmul(s0, 13) ^ Self::gmul(s1, 9) ^ 
                          Self::gmul(s2, 14) ^ Self::gmul(s3, 11);
            state[i*4+3] = Self::gmul(s0, 11) ^ Self::gmul(s1, 13) ^ 
                          Self::gmul(s2, 9) ^ Self::gmul(s3, 14);
        }
    }

    fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
        for i in 0..16 {
            state[i] ^= round_key[i];
        }
    }

    fn encrypt_block(&self, plaintext: &[u8; 16]) -> [u8; 16] {
        let mut state = *plaintext;

        Self::add_round_key(&mut state, &self.round_keys[0]);

        for round in 1..self.nr {
            Self::sub_bytes(&mut state);
            Self::shift_rows(&mut state);
            Self::mix_columns(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round]);
        }

        Self::sub_bytes(&mut state);
        Self::shift_rows(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[self.nr]);

        state
    }

    fn decrypt_block(&self, ciphertext: &[u8; 16]) -> [u8; 16] {
        let mut state = *ciphertext;

        Self::add_round_key(&mut state, &self.round_keys[self.nr]);

        for round in (1..self.nr).rev() {
            Self::inv_shift_rows(&mut state);
            Self::inv_sub_bytes(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round]);
            Self::inv_mix_columns(&mut state);
        }

        Self::inv_shift_rows(&mut state);
        Self::inv_sub_bytes(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[0]);

        state
    }
}

struct ChaCha20 {
    state: [u32; 16],
}

impl ChaCha20 {
    fn new(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> Self {
        let mut state = [0u32; 16];
        
        state[0] = 0x61707865;
        state[1] = 0x3320646e;
        state[2] = 0x79622d32;
        state[3] = 0x6b206574;

        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                key[i*4], key[i*4+1], key[i*4+2], key[i*4+3]
            ]);
        }

        state[12] = counter;
        for i in 0..3 {
            state[13 + i] = u32::from_le_bytes([
                nonce[i*4], nonce[i*4+1], nonce[i*4+2], nonce[i*4+3]
            ]);
        }

        ChaCha20 { state }
    }

    fn quarter_round(a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32) {
        *a = a.wrapping_add(*b); *d ^= *a; *d = d.rotate_left(16);
        *c = c.wrapping_add(*d); *b ^= *c; *b = b.rotate_left(12);
        *a = a.wrapping_add(*b); *d ^= *a; *d = d.rotate_left(8);
        *c = c.wrapping_add(*d); *b ^= *c; *b = b.rotate_left(7);
    }

    fn block(&self) -> [u32; 16] {
        let mut working_state = self.state;

        for _ in 0..10 {
            Self::quarter_round(&mut working_state[0], &mut working_state[4], 
                              &mut working_state[8], &mut working_state[12]);
            Self::quarter_round(&mut working_state[1], &mut working_state[5], 
                              &mut working_state[9], &mut working_state[13]);
            Self::quarter_round(&mut working_state[2], &mut working_state[6], 
                              &mut working_state[10], &mut working_state[14]);
            Self::quarter_round(&mut working_state[3], &mut working_state[7], 
                              &mut working_state[11], &mut working_state[15]);

            Self::quarter_round(&mut working_state[0], &mut working_state[5], 
                              &mut working_state[10], &mut working_state[15]);
            Self::quarter_round(&mut working_state[1], &mut working_state[6], 
                              &mut working_state[11], &mut working_state[12]);
            Self::quarter_round(&mut working_state[2], &mut working_state[7], 
                              &mut working_state[8], &mut working_state[13]);
            Self::quarter_round(&mut working_state[3], &mut working_state[4], 
                              &mut working_state[9], &mut working_state[14]);
        }

        for i in 0..16 {
            working_state[i] = working_state[i].wrapping_add(self.state[i]);
        }

        working_state
    }

    fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::new();
        let mut block_counter = 0;

        for chunk in plaintext.chunks(64) {
            self.state[12] = block_counter;
            let keystream_block = self.block();
            
            let keystream_bytes: Vec<u8> = keystream_block
                .iter()
                .flat_map(|w| w.to_le_bytes().to_vec())
                .collect();

            for (i, &byte) in chunk.iter().enumerate() {
                ciphertext.push(byte ^ keystream_bytes[i]);
            }

            block_counter += 1;
        }

        ciphertext
    }
}

struct SHA256 {
    state: [u32; 8],
    buffer: Vec<u8>,
    len: u64,
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

impl SHA256 {
    fn new() -> Self {
        SHA256 {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buffer: Vec::new(),
            len: 0,
        }
    }

    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        self.len += data.len() as u64;

        while self.buffer.len() >= 64 {
            let block: [u8; 64] = self.buffer[0..64].try_into().unwrap();
            self.process_block(&block);
            self.buffer.drain(0..64);
        }
    }

    fn process_block(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];

        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i*4], block[i*4+1], block[i*4+2], block[i*4+3]
            ]);
        }

        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }

    fn finalize(&mut self) -> [u8; 32] {
        let bit_len = self.len * 8;
        self.buffer.push(0x80);

        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0x00);
        }

        self.buffer.extend_from_slice(&bit_len.to_be_bytes());

        while !self.buffer.is_empty() {
            let block: [u8; 64] = self.buffer[0..64].try_into().unwrap();
            self.process_block(&block);
            self.buffer.drain(0..64);
        }

        let mut result = [0u8; 32];
        for i in 0..8 {
            result[i*4..(i+1)*4].copy_from_slice(&self.state[i].to_be_bytes());
        }

        result
    }
}

struct RSA {
    n: u64,
    e: u64,
    d: u64,
}

impl RSA {
    fn new(p: u64, q: u64, e: u64) -> Self {
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let d = Self::mod_inverse(e, phi);

        RSA { n, e, d }
    }

    fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        let mut result = 1u64;
        base %= modulus;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result as u128 * base as u128 % modulus as u128) as u64;
            }
            exp >>= 1;
            base = (base as u128 * base as u128 % modulus as u128) as u64;
        }

        result
    }

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }

        let (gcd, x1, y1) = Self::extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;

        (gcd, x, y)
    }

    fn mod_inverse(a: u64, m: u64) -> u64 {
        let (_, x, _) = Self::extended_gcd(a as i64, m as i64);
        ((x % m as i64 + m as i64) % m as i64) as u64
    }

    fn encrypt(&self, plaintext: u64) -> u64 {
        Self::mod_pow(plaintext, self.e, self.n)
    }

    fn decrypt(&self, ciphertext: u64) -> u64 {
        Self::mod_pow(ciphertext, self.d, self.n)
    }
}

struct ECDSA {
    a: i64,
    b: i64,
    p: i64,
    g: (i64, i64),
    n: i64,
}

impl ECDSA {
    fn new() -> Self {
        ECDSA {
            a: 0,
            b: 7,
            p: 23,
            g: (1, 1),
            n: 29,
        }
    }

    fn mod_inverse(a: i64, m: i64) -> i64 {
        let (_, x, _) = RSA::extended_gcd(a, m);
        ((x % m + m) % m)
    }

    fn point_add(&self, p1: (i64, i64), p2: (i64, i64)) -> (i64, i64) {
        if p1 == p2 {
            return self.point_double(p1);
        }

        let (x1, y1) = p1;
        let (x2, y2) = p2;

        let slope = ((y2 - y1) * Self::mod_inverse(x2 - x1, self.p)) % self.p;
        let x3 = (slope * slope - x1 - x2) % self.p;
        let y3 = (slope * (x1 - x3) - y1) % self.p;

        ((x3 + self.p) % self.p, (y3 + self.p) % self.p)
    }

    fn point_double(&self, p: (i64, i64)) -> (i64, i64) {
        let (x, y) = p;

        let slope = ((3 * x * x + self.a) * Self::mod_inverse(2 * y, self.p)) % self.p;
        let x3 = (slope * slope - 2 * x) % self.p;
        let y3 = (slope * (x - x3) - y) % self.p;

        ((x3 + self.p) % self.p, (y3 + self.p) % self.p)
    }

    fn scalar_mult(&self, k: i64, p: (i64, i64)) -> (i64, i64) {
        let mut result = p;
        let mut k = k - 1;

        while k > 0 {
            result = self.point_add(result, p);
            k -= 1;
        }

        result
    }
}