use std::fmt;
use std::ops::Deref;

struct StrPair<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> StrPair<'a> {
    fn new(first: &'a str, second: &'a str) -> Self {
        Self { first, second }
    }
    fn longest(&self) -> &'a str {
        if self.first.len() >= self.second.len() {
            self.first
        } else {
            self.second
        }
    }
    fn both<F>(&self, f: F) -> (&'a str, &'a str)
    where
        F: Fn(&str) -> bool,
    {
        (self.first, self.second)
    }
}

struct Buffer<'a> {
    data: &'a mut [u8],
    pos: usize,
}

impl<'a> Buffer<'a> {
    fn new(slice: &'a mut [u8]) -> Self {
        Self { data: slice, pos: 0 }
    }
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let left = self.data.len().saturating_sub(self.pos);
        let take = buf.len().min(left);
        self.data[self.pos..self.pos + take].copy_from_slice(&buf[..take]);
        self.pos += take;
        Ok(take)
    }
    fn written(&self) -> &[u8] {
        &self.data[..self.pos]
    }
}

struct Table<'k, 'v> {
    inner: Vec<(&'k str, &'v str)>,
}

impl<'k, 'v> Table<'k, 'v> {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }
    fn insert(&mut self, k: &'k str, v: &'v str) {
        self.inner.push((k, v))
    }
    fn get(&self, k: &str) -> Option<&'v str> {
        self.inner.iter().find(|(key, _)| *key == k).map(|(_, v)| *v)
    }
    fn keys(&self) -> impl Iterator<Item = &'k str> + '_ {
        self.inner.iter().map(|(k, _)| *k)
    }
    fn values(&self) -> impl Iterator<Item = &'v str> + '_ {
        self.inner.iter().map(|(_, v)| *v)
    }
}

struct Borrowed<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T> Borrowed<'a, T> {
    fn new(x: &'a T) -> Self {
        Self { value: x }
    }
}

impl<'a, T> Deref for Borrowed<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for Borrowed<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

fn combine<'a>(a: &'a str, b: &'a str, c: &'a str) -> &'a str {
    if a.len() + b.len() > c.len() {
        a
    } else {
        b
    }
}

fn main() {
    let mut arena = [0u8; 2048];
    let mut buf = Buffer::new(&mut arena);
    buf.write(b"lifetime").unwrap();
    buf.write(b"practice").unwrap();
    let pair = StrPair::new("left", "right");
    let longest = pair.longest();
    let mut tab = Table::new();
    tab.insert("k", "v");
    let b = Borrowed::new(&42u32);
    let out = combine(longest, std::str::from_utf8(buf.written()).unwrap(), "fallback");
    println!("{}", out);
}