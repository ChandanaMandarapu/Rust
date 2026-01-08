use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Header {
    magic: [u8; 4],
    version: u32,
    flags: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Payload {
    id: u128,
    tags: Vec<String>,
    data: HashMap<String, Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Envelope {
    header: Header,
    payload: Payload,
}

impl Envelope {
    fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("series".into(), (0..1024).map(|i| i as f64 * 1.25).collect());
        Self {
            header: Header {
                magic: *b"RUST",
                version: 1,
                flags: 0xDEADBEEF,
            },
            payload: Payload {
                id: 0xC0FFEE_BAD_BEEF,
                tags: vec!["rust".into(), "serde".into(), "bincode".into()],
                data,
            },
        }
    }
    fn to_bin(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn from_bin(slice: &[u8]) -> Self {
        bincode::deserialize(slice).unwrap()
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

fn main() {
    let env = Envelope::new();
    let bin = env.to_bin();
    let back = Envelope::from_bin(&bin);
    let json = env.to_json();
    let _back2 = Envelope::from_json(&json);
    io::stdout().write_all(format!("{} {}\n", bin.len(), json.len()).as_bytes()).unwrap();
}