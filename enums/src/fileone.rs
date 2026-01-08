use std::collections::VecDeque;
use std::io::{self, Read, Write};

enum Packet {
    Ping(u64),
    Pong(u64),
    Data(Vec<u8>),
    Disconnect,
    Fragment { id: u16, index: u8, total: u8, payload: Vec<u8> },
}

impl Packet {
    fn size_hint(&self) -> usize {
        match self {
            Packet::Ping(_) | Packet::Pong(_) => 8,
            Packet::Data(v) => v.len(),
            Packet::Disconnect => 0,
            Packet::Fragment { payload, .. } => payload.len() + 4,
        }
    }
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            Packet::Ping(n) => { buf.push(1); buf.extend_from_slice(&n.to_le_bytes()); }
            Packet::Pong(n) => { buf.push(2); buf.extend_from_slice(&n.to_le_bytes()); }
            Packet::Data(v) => { buf.push(3); buf.extend_from_slice(&(v.len() as u32).to_le_bytes()); buf.extend_from_slice(v); }
            Packet::Disconnect => buf.push(4),
            Packet::Fragment { id, index, total, payload } => {
                buf.push(5);
                buf.extend_from_slice(&id.to_le_bytes());
                buf.push(*index);
                buf.push(*total);
                buf.extend_from_slice(payload);
            }
        }
    }
    fn decode(buf: &[u8]) -> Option<(Packet, usize)> {
        if buf.is_empty() { return None; }
        match buf[0] {
            1 if buf.len() >= 9 => Some((Packet::Ping(u64::from_le_bytes(buf[1..9].try_into().unwrap())), 9)),
            2 if buf.len() >= 9 => Some((Packet::Pong(u64::from_le_bytes(buf[1..9].try_into().unwrap())), 9)),
            3 if buf.len() >= 5 => {
                let len = u32::from_le_bytes(buf[1..5].try_into().unwrap()) as usize;
                if buf.len() < 5 + len { return None; }
                Some((Packet::Data(buf[5..5 + len].to_vec()), 5 + len))
            }
            4 => Some((Packet::Disconnect, 1)),
            5 if buf.len() >= 5 => {
                let id = u16::from_le_bytes(buf[1..3].try_into().unwrap());
                let index = buf[3];
                let total = buf[4];
                let payload_len = buf.len().saturating_sub(5);
                if index >= total { return None; }
                Some((Packet::Fragment { id, index, total, payload: buf[5..].to_vec() }, 5 + payload_len))
            }
            _ => None,
        }
    }
}

fn main() {
    let mut pool = VecDeque::new();
    for i in 0..2000 {
        pool.push_back(Packet::Ping(i));
        pool.push_back(Packet::Data(vec![i as u8; 1024]));
        pool.push_back(Packet::Fragment { id: (i % 256) as u16, index: 0, total: 3, payload: vec![0; 512] });
    }
    let mut encoded = Vec::with_capacity(1 << 20);
    while let Some(p) = pool.pop_front() {
        p.encode(&mut encoded);
    }
    let mut cursor = 0;
    let mut decoded = Vec::new();
    while cursor < encoded.len() {
        if let Some((pkt, adv)) = Packet::decode(&encoded[cursor..]) {
            decoded.push(pkt);
            cursor += adv;
        } else {
            break;
        }
    }
    io::stdout().write_all(format!("{}\n", decoded.len()).as_bytes()).unwrap();
}