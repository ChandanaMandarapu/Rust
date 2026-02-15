// File 16: Advanced Lifetimes
// Variance, Higher-Ranked Trait Bounds (HRTB), lifetime subtyping,
// self-referential patterns, elision rules, and complex borrow scenarios

use std::fmt;
use std::marker::PhantomData;

// ─── Lifetime Subtyping: 'long: 'short ───────────────────────────────────────
// 'b: 'a means 'b outlives 'a

fn longest_with_announcement<'a, 'b: 'a>(
    x: &'a str,
    y: &'a str,
    ann: &'b str,
) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() >= y.len() { x } else { y }
}

// A context that holds a reference and can lend sub-references
struct Context<'ctx> {
    data: &'ctx str,
}

impl<'ctx> Context<'ctx> {
    fn new(data: &'ctx str) -> Self { Context { data } }

    // Returned ref lives as long as the context itself, not just the method call
    fn extract_word(&self, start: usize, end: usize) -> &'ctx str {
        &self.data[start..end]
    }

    fn first_word(&self) -> &'ctx str {
        self.data
            .find(' ')
            .map(|i| &self.data[..i])
            .unwrap_or(self.data)
    }
}

// ─── Higher-Ranked Trait Bounds (for<'a>) ────────────────────────────────────
// The function must work for *any* lifetime 'a, not just a specific one

fn apply_to_str<F>(f: F, s: &str) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    f(s)
}

// A struct that stores a closure valid for any lifetime
struct StrProcessor<F>
where
    F: for<'a> Fn(&'a str) -> String,
{
    func: F,
}

impl<F> StrProcessor<F>
where
    F: for<'a> Fn(&'a str) -> String,
{
    fn new(func: F) -> Self { StrProcessor { func } }
    fn process<'a>(&self, input: &'a str) -> String { (self.func)(input) }
}

// HRTB with traits — a trait that must be implementable for any lifetime
trait Transform {
    fn transform<'a>(&self, input: &'a str) -> &'a str;
}

struct IdentityTransform;
impl Transform for IdentityTransform {
    fn transform<'a>(&self, input: &'a str) -> &'a str { input }
}

fn apply_transform<T: Transform>(t: &T, s: &str) -> &str {
    t.transform(s)  // lifetime flows through
}

// ─── Variance ─────────────────────────────────────────────────────────────────
// Covariant:     &'a T — shorter lifetime is a subtype
// Contravariant: fn(T) — works with longer lifetimes
// Invariant:     &'a mut T — must match exactly

// PhantomData lets you declare variance without holding the value

struct Covariant<'a, T: 'a> {
    _marker: PhantomData<&'a T>,  // covariant over T and 'a
}

struct Invariant<'a, T: 'a> {
    _marker: PhantomData<&'a mut T>,  // invariant over T
}

struct Contravariant<T> {
    _marker: PhantomData<fn(T)>,  // contravariant over T
}

// ─── Lending Iterator (GAT preview via workaround) ───────────────────────────
// An iterator where the yielded reference borrows from the iterator itself

struct LendingLines<'buf> {
    buffer: &'buf str,
    pos: usize,
}

impl<'buf> LendingLines<'buf> {
    fn new(buffer: &'buf str) -> Self { LendingLines { buffer, pos: 0 } }

    // Borrows from 'buf (the buffer), not from &self
    fn next_line(&mut self) -> Option<&'buf str> {
        if self.pos >= self.buffer.len() { return None; }
        let rest = &self.buffer[self.pos..];
        let end = rest.find('\n').unwrap_or(rest.len());
        let line = &self.buffer[self.pos..self.pos + end];
        self.pos += end + 1;
        if line.is_empty() && self.pos > self.buffer.len() { None } else { Some(line) }
    }
}

// ─── Self-Referential via Split Borrows ──────────────────────────────────────
// Rust prevents self-referential structs directly; here we show
// the safe split-borrow pattern instead

struct SplitSlice<'a, T> {
    left:  &'a [T],
    right: &'a [T],
}

impl<'a, T: fmt::Debug> SplitSlice<'a, T> {
    fn from_slice(slice: &'a [T], mid: usize) -> Self {
        let (l, r) = slice.split_at(mid);
        SplitSlice { left: l, right: r }
    }

    fn print(&self) {
        println!("left:  {:?}", self.left);
        println!("right: {:?}", self.right);
    }
}

// A struct that holds two independent borrows from the same source
struct HeadTail<'h, 't, T> {
    head: &'h T,
    tail: &'t [T],
}

impl<'h, 't, T: fmt::Debug> HeadTail<'h, 't, T> {
    fn from_slice(slice: &'h [T]) -> Option<HeadTail<'h, 'h, T>> {
        let (first, rest) = slice.split_first()?;
        Some(HeadTail { head: first, tail: rest })
    }

    fn head(&self) -> &T { self.head }
    fn tail(&self) -> &[T] { self.tail }
}

// ─── Lifetime Elision Deep Dive ───────────────────────────────────────────────
// Three elision rules — showing what the compiler infers

// Rule 1: each input ref gets its own lifetime
// fn foo(x: &str, y: &str)  →  fn foo<'a,'b>(x: &'a str, y: &'b str)
fn first_char(s: &str) -> Option<char> { s.chars().next() }

// Rule 2: if exactly one input lifetime, it's assigned to all outputs
// fn first_word(s: &str) -> &str  →  fn first_word<'a>(s: &'a str) -> &'a str
fn trim_left(s: &str) -> &str { s.trim_start() }

// Rule 3: if &self or &mut self, its lifetime is assigned to all outputs
struct Parser<'a> { input: &'a str, pos: usize }
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self { Parser { input, pos: 0 } }

    // Output lifetime tied to 'a (the struct), NOT to &self
    fn peek(&self) -> &'a str { &self.input[self.pos..] }

    fn consume(&mut self, n: usize) -> &'a str {
        let start = self.pos;
        self.pos = (self.pos + n).min(self.input.len());
        &self.input[start..self.pos]
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, pred: F) -> &'a str {
        let start = self.pos;
        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next().unwrap();
            if !pred(ch) { break; }
            self.pos += ch.len_utf8();
        }
        &self.input[start..self.pos]
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn is_done(&self) -> bool { self.pos >= self.input.len() }
}

// ─── Reborrowing and Two-Phase Borrows ───────────────────────────────────────

fn demonstrate_reborrow() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Two-phase borrow: reserve borrows mutably but doesn't conflict
    // because the actual mutation happens after the immutable access
    let len = data.len();
    data.push(len as i32 + 1);
    println!("After push: {:?}", data);

    // Reborrowing: &mut T can be reborrowed as &mut T (shorter lifetime)
    fn add_one(v: &mut i32) { *v += 1; }
    let r = &mut data[0];
    add_one(r);       // r is reborrowed here, not moved
    add_one(r);       // still valid
    println!("After add_one x2 on [0]: {:?}", data);
}

// ─── Lifetime in Closures ─────────────────────────────────────────────────────

// Closure that captures a reference — the closure's lifetime is tied to it
fn make_greeter<'a>(name: &'a str) -> impl Fn() -> String + 'a {
    move || format!("Hello, {}!", name)
}

// Closure returning a reference — must annotate
fn find_first<'a, T, F>(slice: &'a [T], pred: F) -> Option<&'a T>
where
    F: Fn(&T) -> bool,
{
    slice.iter().find(|x| pred(x))
}

// ─── Arena Allocator Pattern ──────────────────────────────────────────────────
// All items get the arena's lifetime — safe and fast

struct Arena {
    chunks: Vec<Vec<u8>>,
    chunk_size: usize,
}

impl Arena {
    fn new(chunk_size: usize) -> Self {
        Arena { chunks: vec![Vec::with_capacity(chunk_size)], chunk_size }
    }

    // All strings allocated here live as long as the Arena
    fn alloc_str<'a>(&'a mut self, s: &str) -> &'a str {
        let bytes = s.as_bytes();
        let last = self.chunks.last_mut().unwrap();
        if last.len() + bytes.len() > self.chunk_size {
            self.chunks.push(Vec::with_capacity(self.chunk_size.max(bytes.len())));
        }
        let chunk = self.chunks.last_mut().unwrap();
        let start = chunk.len();
        chunk.extend_from_slice(bytes);
        // SAFETY: We own the memory and it's valid UTF-8 (came from &str)
        unsafe {
            std::str::from_utf8_unchecked(&chunk[start..start + bytes.len()])
        }
    }

    fn stats(&self) -> (usize, usize) {
        let total: usize = self.chunks.iter().map(|c| c.capacity()).sum();
        let used:  usize = self.chunks.iter().map(|c| c.len()).sum();
        (used, total)
    }
}

// ─── Complex Struct with Multiple Lifetimes ───────────────────────────────────

struct Query<'conn, 'params> {
    sql:    &'conn str,
    params: &'params [&'params str],
    limit:  usize,
}

impl<'conn, 'params> Query<'conn, 'params> {
    fn new(sql: &'conn str, params: &'params [&'params str]) -> Self {
        Query { sql, params, limit: 100 }
    }

    fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    fn describe(&self) -> String {
        format!(
            "SQL: '{}' | params: {:?} | limit: {}",
            self.sql, self.params, self.limit
        )
    }

    // Returns a reference tied to 'conn (the SQL string lifetime)
    fn sql(&self) -> &'conn str { self.sql }
}

// ─── Lifetime Bounds on Generics ─────────────────────────────────────────────

// T: 'a means "T must outlive 'a" — T may contain references, all >= 'a
struct Holder<'a, T: 'a> {
    value: &'a T,
}

impl<'a, T: fmt::Display + 'a> Holder<'a, T> {
    fn new(value: &'a T) -> Self { Holder { value } }
    fn show(&self) { println!("Holding: {}", self.value); }
}

// 'static bound: T contains no non-static references
fn store_forever<T: 'static + fmt::Debug>(value: T) -> Box<T> {
    println!("Storing forever: {:?}", value);
    Box::new(value)
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Advanced Lifetimes ===\n");

    // Lifetime subtyping
    println!("── Subtyping ──");
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xy");
        result = longest_with_announcement(s1.as_str(), s2.as_str(), "comparing lengths");
        println!("longest: {}", result);
    }

    // Context with inner borrows
    println!("\n── Context borrows ──");
    let text = String::from("hello world from rust");
    let ctx  = Context::new(&text);
    let word = ctx.extract_word(0, 5);
    let fw   = ctx.first_word();
    println!("extracted: {}, first: {}", word, fw);

    // HRTB
    println!("\n── HRTB ──");
    let result = apply_to_str(|s| s.len(), "hello");
    println!("len via HRTB: {}", result);

    let proc = StrProcessor::new(|s: &str| s.to_uppercase());
    println!("processed: {}", proc.process("hello lifetimes"));

    // Lending lines
    println!("\n── LendingLines ──");
    let text = "line one\nline two\nline three\n";
    let mut lines = LendingLines::new(text);
    while let Some(line) = lines.next_line() {
        println!("  '{}'", line);
    }

    // Split borrows
    println!("\n── Split borrows ──");
    let nums = vec![1, 2, 3, 4, 5, 6];
    let split = SplitSlice::from_slice(&nums, 3);
    split.print();

    let ht = HeadTail::from_slice(&nums).unwrap();
    println!("head: {:?}, tail: {:?}", ht.head(), ht.tail());

    // Parser with lifetime-bearing output
    println!("\n── Parser ──");
    let source = "   let x = 42   rest";
    let mut parser = Parser::new(source);
    parser.skip_whitespace();
    let keyword = parser.consume_while(|c| c.is_alphabetic());
    parser.skip_whitespace();
    let rest = parser.peek();
    println!("keyword: '{}', rest: '{}'", keyword, rest);

    // Reborrow
    println!("\n── Reborrow ──");
    demonstrate_reborrow();

    // Closures with lifetimes
    println!("\n── Closures ──");
    let name = String::from("Rustacean");
    let greet = make_greeter(&name);
    println!("{}", greet());
    println!("{}", greet()); // closure borrows name, still valid

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let first_even = find_first(&numbers, |&&x| x % 2 == 0);
    println!("first even: {:?}", first_even);

    // Arena
    println!("\n── Arena Allocator ──");
    let mut arena = Arena::new(64);
    let a = arena.alloc_str("hello");
    let b = arena.alloc_str(" world");
    let c = arena.alloc_str(" from the arena allocator");
    println!("a='{}' b='{}' c='{}'", a, b, c);
    let (used, cap) = arena.stats();
    println!("arena: {} bytes used / {} capacity", used, cap);

    // Query with two lifetimes
    println!("\n── Multi-lifetime Query ──");
    let sql    = String::from("SELECT * FROM users WHERE id = ? AND role = ?");
    let params = vec!["42", "admin"];
    let query  = Query::new(&sql, &params).with_limit(10);
    println!("{}", query.describe());
    println!("SQL ref: '{}'", query.sql());

    // Generic lifetime bounds
    println!("\n── Generic lifetime bounds ──");
    let val = 42i32;
    let holder = Holder::new(&val);
    holder.show();

    let owned = store_forever(vec![1, 2, 3]);
    println!("stored: {:?}", owned);

    // 'static strings are fine
    let s: &'static str = "I live forever";
    let _b = store_forever(s);

    println!("\n=== Done ===");
}