// Built a full custom wire format from scratch including LEB128 varint encoding with zigzag, zero copy ByteReader cursor, ByteWriter with patching and alignment, and a recursive TLV serializer supporting ints floats strings lists maps and null. Added message framing with magic header versioning payload length CRC32 checksum validation and error handling. Implemented codec pipeline with XOR and run length encoding stages. Main demonstrates varints zigzag decoding TLV roundtrips frame corruption detection and multi stage encode decode flow.
use std::fmt;
use std::collections::HashMap;
use std::io::{self, Read, Write, Cursor};

// ─── Varint Encoding (LEB128) ─────────────────────────────────────────────────
// Used in protobuf, DWARF, WebAssembly — compact variable-length integers

fn encode_varint(mut n: u64) -> Vec<u8> {
    let mut out = vec![];
    loop {
        let byte = (n & 0x7F) as u8;
        n >>= 7;
        if n == 0 { out.push(byte); break; }
        else       { out.push(byte | 0x80); }
    }
    out
}

fn decode_varint(data: &[u8]) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift  = 0;
    for (i, &byte) in data.iter().enumerate() {
        if shift >= 64 { return None; }
        result |= ((byte & 0x7F) as u64) << shift;
        shift  += 7;
        if byte & 0x80 == 0 { return Some((result, i + 1)); }
    }
    None
}

fn encode_zigzag(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

fn decode_zigzag(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

// ─── Byte Cursor (zero-copy reader) ──────────────────────────────────────────

struct ByteReader<'a> {
    data: &'a [u8],
    pos:  usize,
}

impl<'a> ByteReader<'a> {
    fn new(data: &'a [u8]) -> Self { ByteReader { data, pos: 0 } }
    fn remaining(&self) -> usize   { self.data.len() - self.pos }
    fn is_empty(&self)  -> bool    { self.pos >= self.data.len() }
    fn position(&self)  -> usize   { self.pos }

    fn peek_byte(&self) -> Option<u8> { self.data.get(self.pos).copied() }

    fn read_byte(&mut self) -> Option<u8> {
        let b = self.data.get(self.pos).copied();
        if b.is_some() { self.pos += 1; }
        b
    }

    fn read_bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.pos + n > self.data.len() { return None; }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Some(s)
    }

    fn read_u8(&mut self)  -> Option<u8>  { self.read_byte() }
    fn read_u16_be(&mut self) -> Option<u16> {
        let b = self.read_bytes(2)?;
        Some(u16::from_be_bytes([b[0], b[1]]))
    }
    fn read_u32_be(&mut self) -> Option<u32> {
        let b = self.read_bytes(4)?;
        Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn read_u64_be(&mut self) -> Option<u64> {
        let b = self.read_bytes(8)?;
        Some(u64::from_be_bytes(b.try_into().ok()?))
    }
    fn read_i64_be(&mut self) -> Option<i64> {
        let b = self.read_bytes(8)?;
        Some(i64::from_be_bytes(b.try_into().ok()?))
    }
    fn read_f64_be(&mut self) -> Option<f64> {
        let b = self.read_bytes(8)?;
        Some(f64::from_be_bytes(b.try_into().ok()?))
    }

    fn read_varint(&mut self) -> Option<u64> {
        let (val, n) = decode_varint(&self.data[self.pos..])?;
        self.pos += n;
        Some(val)
    }

    fn read_length_prefixed(&mut self) -> Option<&'a [u8]> {
        let len = self.read_varint()? as usize;
        self.read_bytes(len)
    }

    fn read_cstr(&mut self) -> Option<&'a [u8]> {
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != 0 { self.pos += 1; }
        if self.pos >= self.data.len() { return None; }
        let s = &self.data[start..self.pos];
        self.pos += 1; // skip null byte
        Some(s)
    }

    fn skip(&mut self, n: usize) -> bool {
        if self.pos + n > self.data.len() { return false; }
        self.pos += n;
        true
    }

    fn align_to(&mut self, align: usize) {
        let rem = self.pos % align;
        if rem != 0 { self.pos += align - rem; }
    }
}

// ─── ByteWriter ───────────────────────────────────────────────────────────────

struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    fn new() -> Self { ByteWriter { buf: vec![] } }
    fn with_capacity(n: usize) -> Self { ByteWriter { buf: Vec::with_capacity(n) } }

    fn write_u8 (&mut self, v: u8)  { self.buf.push(v); }
    fn write_u16_be(&mut self, v: u16) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn write_u32_be(&mut self, v: u32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn write_u64_be(&mut self, v: u64) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn write_i64_be(&mut self, v: i64) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn write_f64_be(&mut self, v: f64) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    fn write_bytes(&mut self, v: &[u8]) { self.buf.extend_from_slice(v); }

    fn write_varint(&mut self, n: u64) { self.buf.extend(encode_varint(n)); }
    fn write_length_prefixed(&mut self, data: &[u8]) {
        self.write_varint(data.len() as u64);
        self.write_bytes(data);
    }

    fn write_cstr(&mut self, s: &str) {
        self.write_bytes(s.as_bytes());
        self.buf.push(0);
    }

    fn patch_u32_be(&mut self, pos: usize, v: u32) {
        let b = v.to_be_bytes();
        self.buf[pos..pos+4].copy_from_slice(&b);
    }

    fn len(&self) -> usize { self.buf.len() }
    fn as_bytes(&self) -> &[u8] { &self.buf }
    fn finish(self) -> Vec<u8> { self.buf }

    // Reserve space and return position for later patching
    fn reserve_u32(&mut self) -> usize {
        let pos = self.buf.len();
        self.buf.extend_from_slice(&[0u8; 4]);
        pos
    }

    fn align_to(&mut self, align: usize) {
        let rem = self.buf.len() % align;
        if rem != 0 { self.buf.extend(std::iter::repeat(0).take(align - rem)); }
    }
}

// ─── TLV (Type-Length-Value) Protocol ────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum TlvValue {
    Integer(i64),
    Float(f64),
    Bytes(Vec<u8>),
    Str(String),
    List(Vec<TlvValue>),
    Map(Vec<(u16, TlvValue)>),
    Bool(bool),
    Null,
}

const TAG_INT:  u8 = 0x01;
const TAG_F64:  u8 = 0x02;
const TAG_BYTES:u8 = 0x03;
const TAG_STR:  u8 = 0x04;
const TAG_LIST: u8 = 0x05;
const TAG_MAP:  u8 = 0x06;
const TAG_BOOL: u8 = 0x07;
const TAG_NULL: u8 = 0x00;

impl TlvValue {
    fn encode(&self, w: &mut ByteWriter) {
        match self {
            TlvValue::Null       => { w.write_u8(TAG_NULL); w.write_varint(0); }
            TlvValue::Bool(b)    => { w.write_u8(TAG_BOOL); w.write_varint(1); w.write_u8(*b as u8); }
            TlvValue::Integer(n) => {
                let enc = encode_varint(encode_zigzag(*n));
                w.write_u8(TAG_INT); w.write_varint(enc.len() as u64); w.write_bytes(&enc);
            }
            TlvValue::Float(f)   => { w.write_u8(TAG_F64); w.write_varint(8); w.write_f64_be(*f); }
            TlvValue::Bytes(b)   => { w.write_u8(TAG_BYTES); w.write_length_prefixed(b); }
            TlvValue::Str(s)     => {
                w.write_u8(TAG_STR);
                let b = s.as_bytes();
                w.write_varint(b.len() as u64);
                w.write_bytes(b);
            }
            TlvValue::List(items) => {
                w.write_u8(TAG_LIST);
                let mut sub = ByteWriter::new();
                sub.write_varint(items.len() as u64);
                for item in items { item.encode(&mut sub); }
                let bytes = sub.finish();
                w.write_varint(bytes.len() as u64);
                w.write_bytes(&bytes);
            }
            TlvValue::Map(pairs) => {
                w.write_u8(TAG_MAP);
                let mut sub = ByteWriter::new();
                sub.write_varint(pairs.len() as u64);
                for (k, v) in pairs { sub.write_u16_be(*k); v.encode(&mut sub); }
                let bytes = sub.finish();
                w.write_varint(bytes.len() as u64);
                w.write_bytes(&bytes);
            }
        }
    }

    fn decode(r: &mut ByteReader) -> Option<Self> {
        let tag = r.read_u8()?;
        let len = r.read_varint()? as usize;

        match tag {
            TAG_NULL => Some(TlvValue::Null),
            TAG_BOOL => {
                let b = r.read_u8()?;
                Some(TlvValue::Bool(b != 0))
            }
            TAG_INT => {
                let bytes = r.read_bytes(len)?;
                let (z, _) = decode_varint(bytes)?;
                Some(TlvValue::Integer(decode_zigzag(z)))
            }
            TAG_F64 => Some(TlvValue::Float(r.read_f64_be()?)),
            TAG_BYTES => {
                let b = r.read_bytes(len)?.to_vec();
                Some(TlvValue::Bytes(b))
            }
            TAG_STR => {
                let b = r.read_bytes(len)?;
                Some(TlvValue::Str(String::from_utf8_lossy(b).into_owned()))
            }
            TAG_LIST => {
                let bytes = r.read_bytes(len)?;
                let mut sub = ByteReader::new(bytes);
                let count = sub.read_varint()? as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count { items.push(Self::decode(&mut sub)?); }
                Some(TlvValue::List(items))
            }
            TAG_MAP => {
                let bytes = r.read_bytes(len)?;
                let mut sub = ByteReader::new(bytes);
                let count = sub.read_varint()? as usize;
                let mut pairs = Vec::with_capacity(count);
                for _ in 0..count {
                    let k = sub.read_u16_be()?;
                    let v = Self::decode(&mut sub)?;
                    pairs.push((k, v));
                }
                Some(TlvValue::Map(pairs))
            }
            _ => None,
        }
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        let mut w = ByteWriter::new();
        self.encode(&mut w);
        w.finish()
    }

    fn decode_from_slice(data: &[u8]) -> Option<Self> {
        let mut r = ByteReader::new(data);
        Self::decode(&mut r)
    }
}

impl fmt::Display for TlvValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TlvValue::Null       => write!(f, "null"),
            TlvValue::Bool(b)    => write!(f, "{}", b),
            TlvValue::Integer(n) => write!(f, "{}", n),
            TlvValue::Float(v)   => write!(f, "{}", v),
            TlvValue::Bytes(b)   => write!(f, "<{} bytes>", b.len()),
            TlvValue::Str(s)     => write!(f, "\"{}\"", s),
            TlvValue::List(l)    => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() { if i > 0 { write!(f, ", ")?; } write!(f, "{}", v)?; }
                write!(f, "]")
            }
            TlvValue::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() { if i > 0 { write!(f, ", ")?; } write!(f, "{}:{}", k, v)?; }
                write!(f, "}}")
            }
        }
    }
}

// ─── Message Framing Protocol ─────────────────────────────────────────────────
//
// Frame format:
//   [4 bytes magic] [1 byte version] [1 byte msg_type] [4 bytes payload_len]
//   [4 bytes checksum] [N bytes payload]

const MAGIC: u32 = 0xCAFE_BABE;
const VERSION: u8 = 1;

fn crc32(data: &[u8]) -> u32 {
    // Simple CRC-32 (Castagnoli) — no external crate
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            if crc & 1 != 0 { crc = (crc >> 1) ^ 0xEDB8_8320; }
            else             { crc >>= 1; }
        }
    }
    !crc
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum MsgType { Data = 0x01, Ack = 0x02, Error = 0x03, Ping = 0x04, Pong = 0x05 }

#[derive(Debug, Clone)]
struct Frame { msg_type: MsgType, payload: Vec<u8> }

impl Frame {
    fn new(msg_type: MsgType, payload: Vec<u8>) -> Self { Frame { msg_type, payload } }

    fn encode(&self) -> Vec<u8> {
        let mut w = ByteWriter::with_capacity(14 + self.payload.len());
        w.write_u32_be(MAGIC);
        w.write_u8(VERSION);
        w.write_u8(self.msg_type as u8);
        w.write_u32_be(self.payload.len() as u32);
        w.write_u32_be(crc32(&self.payload));
        w.write_bytes(&self.payload);
        w.finish()
    }

    fn decode(data: &[u8]) -> Result<(Frame, usize), String> {
        let mut r = ByteReader::new(data);

        let magic = r.read_u32_be().ok_or("truncated: magic")?;
        if magic != MAGIC { return Err(format!("bad magic: 0x{:08X}", magic)); }

        let version = r.read_u8().ok_or("truncated: version")?;
        if version != VERSION { return Err(format!("unsupported version: {}", version)); }

        let msg_type = match r.read_u8().ok_or("truncated: type")? {
            0x01 => MsgType::Data,  0x02 => MsgType::Ack,
            0x03 => MsgType::Error, 0x04 => MsgType::Ping,
            0x05 => MsgType::Pong,
            t => return Err(format!("unknown msg_type: 0x{:02X}", t)),
        };

        let payload_len = r.read_u32_be().ok_or("truncated: len")? as usize;
        let checksum    = r.read_u32_be().ok_or("truncated: checksum")?;
        let payload     = r.read_bytes(payload_len).ok_or("truncated: payload")?.to_vec();

        let computed = crc32(&payload);
        if computed != checksum {
            return Err(format!("checksum mismatch: got 0x{:08X}, expected 0x{:08X}", computed, checksum));
        }

        Ok((Frame { msg_type, payload }, r.position()))
    }
}

// ─── Codec Pipeline ───────────────────────────────────────────────────────────

trait Codec {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Vec<u8>;
    fn name(&self)  -> &str;
}

struct XorCodec { key: u8 }
struct RunLengthCodec;
struct Passthrough;

impl Codec for XorCodec {
    fn name(&self) -> &str { "XOR" }
    fn encode(&self, data: &[u8]) -> Vec<u8> { data.iter().map(|&b| b ^ self.key).collect() }
    fn decode(&self, data: &[u8]) -> Vec<u8> { self.encode(data) }
}

impl Codec for RunLengthCodec {
    fn name(&self) -> &str { "RLE" }
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut out = vec![];
        let mut i = 0;
        while i < data.len() {
            let byte = data[i];
            let mut count = 1usize;
            while i + count < data.len() && data[i + count] == byte && count < 255 { count += 1; }
            out.push(count as u8);
            out.push(byte);
            i += count;
        }
        out
    }
    fn decode(&self, data: &[u8]) -> Vec<u8> {
        let mut out = vec![];
        let mut i = 0;
        while i + 1 < data.len() {
            let count = data[i] as usize;
            let byte  = data[i + 1];
            out.extend(std::iter::repeat(byte).take(count));
            i += 2;
        }
        out
    }
}

impl Codec for Passthrough {
    fn name(&self) -> &str { "Passthrough" }
    fn encode(&self, data: &[u8]) -> Vec<u8> { data.to_vec() }
    fn decode(&self, data: &[u8]) -> Vec<u8> { data.to_vec() }
}

struct CodecPipeline { stages: Vec<Box<dyn Codec>> }

impl CodecPipeline {
    fn new() -> Self { CodecPipeline { stages: vec![] } }
    fn add(mut self, c: Box<dyn Codec>) -> Self { self.stages.push(c); self }

    fn encode(&self, data: &[u8]) -> Vec<u8> {
        self.stages.iter().fold(data.to_vec(), |d, c| c.encode(&d))
    }
    fn decode(&self, data: &[u8]) -> Vec<u8> {
        self.stages.iter().rev().fold(data.to_vec(), |d, c| c.decode(&d))
    }
    fn describe(&self) -> String {
        self.stages.iter().map(|c| c.name()).collect::<Vec<_>>().join(" → ")
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Binary Protocols & Serialization ===\n");

    // Varint
    println!("── Varint (LEB128) ──");
    for n in [0u64, 127, 128, 300, 16383, 16384, 1_000_000, u64::MAX / 2] {
        let enc = encode_varint(n);
        let (dec, consumed) = decode_varint(&enc).unwrap();
        println!("  {:12} → {:?} ({} bytes) → {}", n, enc, consumed, dec);
    }

    // Zigzag
    println!("\n── ZigZag Encoding ──");
    for n in [0i64, -1, 1, -2, 2, i64::MIN, i64::MAX] {
        let z = encode_zigzag(n);
        let b = decode_zigzag(z);
        println!("  {:20} → {} → {}", n, z, b);
    }

    // ByteReader
    println!("\n── ByteReader ──");
    let raw = b"\x00\x01\x02\x03\xDE\xAD\xBE\xEF\x41\x42\x43\x00rest";
    let mut r = ByteReader::new(raw);
    println!("  u8:    {}", r.read_u8().unwrap());
    println!("  u16be: {}", r.read_u16_be().unwrap());
    println!("  u32be: 0x{:08X}", r.read_u32_be().unwrap());
    println!("  cstr:  {:?}", std::str::from_utf8(r.read_cstr().unwrap()).unwrap());
    println!("  remaining: {}", r.remaining());

    // TLV serialization
    println!("\n── TLV Serialization ──");
    let original = TlvValue::Map(vec![
        (1, TlvValue::Str("Alice".to_string())),
        (2, TlvValue::Integer(30)),
        (3, TlvValue::Float(98.6)),
        (4, TlvValue::Bool(true)),
        (5, TlvValue::List(vec![
            TlvValue::Integer(10),
            TlvValue::Integer(20),
            TlvValue::Integer(30),
        ])),
        (6, TlvValue::Null),
    ]);

    let encoded = original.encode_to_vec();
    println!("  original: {}", original);
    println!("  encoded:  {} bytes: {:?}", encoded.len(), &encoded[..encoded.len().min(20)]);

    let decoded = TlvValue::decode_from_slice(&encoded).unwrap();
    println!("  decoded:  {}", decoded);
    println!("  roundtrip ok: {}", original == decoded);

    // Nested
    let nested = TlvValue::List(vec![
        TlvValue::Integer(-99),
        TlvValue::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        TlvValue::Str("hello world".to_string()),
    ]);
    let ne = nested.encode_to_vec();
    let nd = TlvValue::decode_from_slice(&ne).unwrap();
    println!("  nested roundtrip: {} → {}", nested, nd);

    // Framing
    println!("\n── Message Framing ──");
    let payloads: Vec<(&str, MsgType, &[u8])> = vec![
        ("Data", MsgType::Data, b"Hello, network!"),
        ("Ping", MsgType::Ping, b""),
        ("Ack",  MsgType::Ack,  &[0x00, 0x00, 0x00, 0x01]),
    ];

    for (label, mtype, payload) in &payloads {
        let frame   = Frame::new(*mtype, payload.to_vec());
        let encoded = frame.encode();
        let checksum = crc32(payload);

        match Frame::decode(&encoded) {
            Ok((f, consumed)) => {
                println!("  {} | type={:?} len={} crc=0x{:08X} consumed={}",
                    label, f.msg_type, f.payload.len(), checksum, consumed);
            }
            Err(e) => println!("  {} | decode error: {}", label, e),
        }
    }

    // Corrupted frame
    let mut bad = Frame::new(MsgType::Data, b"test data".to_vec()).encode();
    bad[13] ^= 0xFF; // flip some bits in payload
    match Frame::decode(&bad) {
        Ok(_) => println!("  (should not happen)"),
        Err(e) => println!("  corrupted frame: {}", e),
    }

    // Codec pipeline
    println!("\n── Codec Pipeline ──");
    let input = b"AAAABBBCCCCDDDDEEEEEFF";
    println!("  input:  {:?} ({} bytes)", std::str::from_utf8(input).unwrap(), input.len());

    let pipeline = CodecPipeline::new()
        .add(Box::new(RunLengthCodec))
        .add(Box::new(XorCodec { key: 0x5A }));
    println!("  pipeline: {}", pipeline.describe());

    let enc = pipeline.encode(input);
    println!("  encoded: {:?} ({} bytes)", &enc, enc.len());

    let dec = pipeline.decode(&enc);
    println!("  decoded: {:?}", std::str::from_utf8(&dec).unwrap());
    println!("  roundtrip ok: {}", dec == input);

    println!("\n=== Done ===");
}