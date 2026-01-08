use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher, BuildHasherDefault};
use std::io::{self, Write};

#[derive(Default)]
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Hasher for XorShift64 {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state ^= b as u64;
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
        }
    }
}

type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<XorShift64>>;

fn build_reverse_index(words: &[&str]) -> FastMap<String, Vec<usize>> {
    let mut map = FastMap::default();
    for (idx, &w) in words.iter().enumerate() {
        for len in 1..=w.len() {
            let key = w[..len].to_string();
            map.entry(key).or_default().push(idx);
        }
    }
    map
}

fn count_bigrams(text: &str) -> FastMap<(char, char), usize> {
    let mut counts = FastMap::default();
    let mut prev: Option<char> = None;
    for c in text.chars().filter(|c| c.is_alphabetic()) {
        if let Some(p) = prev {
            *counts.entry((p.to_ascii_lowercase(), c.to_ascii_lowercase())).or_insert(0) += 1;
        }
        prev = Some(c);
    }
    counts
}

fn bfs_shortest_path(
    adj: &FastMap<usize, Vec<usize>>,
    start: usize,
    goal: usize,
) -> Option<Vec<usize>> {
    let mut queue = VecDeque::new();
    let mut parent = FastMap::default();
    queue.push_back(start);
    parent.insert(start, None);
    while let Some(cur) = queue.pop_front() {
        if cur == goal {
            let mut path = vec![goal];
            let mut p = goal;
            while let Some(&Some(prev)) = parent.get(&p) {
                path.push(prev);
                p = prev;
            }
            path.reverse();
            return Some(path);
        }
        if let Some(neigh) = adj.get(&cur) {
            for &n in neigh {
                if !parent.contains_key(&n) {
                    parent.insert(n, Some(cur));
                    queue.push_back(n);
                }
            }
        }
    }
    None
}

fn main() {
    let corpus = include_str!("02_hashmap.rs");
    let words: Vec<_> = corpus.split_whitespace().collect();
    let idx = build_reverse_index(&words);
    let big = count_bigrams(corpus);
    let mut adj = FastMap::default();
    for window in words.windows(2) {
        let a = window[0].len();
        let b = window[1].len();
        adj.entry(a).or_default().push(b);
    }
    let path = bfs_shortest_path(&adj, 1, 20).unwrap_or_default();
    io::stdout().write_all(format!("{} {} {}\n", idx.len(), big.len(), path.len()).as_bytes()).unwrap();
}