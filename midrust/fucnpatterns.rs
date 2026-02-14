// File 14: Functional Patterns — Closures, Iterators, and Composition
// Lazy evaluation, custom adaptors, function composition, monadic patterns

use std::collections::HashMap;
use std::fmt;

// ─── Function Composition ─────────────────────────────────────────────────────

fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn pipe<A, B>(value: A, f: impl Fn(A) -> B) -> B {
    f(value)
}

// A chainable pipeline value
struct Pipeline<T>(T);

impl<T> Pipeline<T> {
    fn new(v: T) -> Self { Pipeline(v) }

    fn map<U, F: Fn(T) -> U>(self, f: F) -> Pipeline<U> {
        Pipeline(f(self.0))
    }

    fn tap<F: Fn(&T)>(self, f: F) -> Self {
        f(&self.0);
        self
    }

    fn finish(self) -> T { self.0 }
}

// ─── Custom Iterator Adaptors ─────────────────────────────────────────────────

// Scan with early-exit (like scan but stops when condition is met)
struct TakeWhileScan<I, S, F> {
    iter: I,
    state: S,
    f: F,
    done: bool,
}

impl<I, S, B, F> Iterator for TakeWhileScan<I, S, F>
where
    I: Iterator,
    F: FnMut(&mut S, I::Item) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        if self.done { return None; }
        let item = self.iter.next()?;
        match (self.f)(&mut self.state, item) {
            Some(b) => Some(b),
            None => { self.done = true; None }
        }
    }
}

// Running total adaptor
struct RunningTotal<I: Iterator<Item = f64>> {
    iter: I,
    total: f64,
}

impl<I: Iterator<Item = f64>> Iterator for RunningTotal<I> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let n = self.iter.next()?;
        self.total += n;
        Some(self.total)
    }
}

trait RunningTotalExt: Iterator<Item = f64> + Sized {
    fn running_total(self) -> RunningTotal<Self> {
        RunningTotal { iter: self, total: 0.0 }
    }
}
impl<I: Iterator<Item = f64>> RunningTotalExt for I {}

// Windowed iterator — yields overlapping slices of size N
struct Windows<I: Iterator> {
    iter: I,
    window: std::collections::VecDeque<I::Item>,
    size: usize,
}

impl<I: Iterator> Windows<I> where I::Item: Clone {
    fn new(iter: I, size: usize) -> Self {
        Windows { iter, window: std::collections::VecDeque::new(), size }
    }
}

impl<I: Iterator> Iterator for Windows<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.window.len() < self.size {
            self.window.push_back(self.iter.next()?);
        }
        let result: Vec<_> = self.window.iter().cloned().collect();
        self.window.pop_front();
        Some(result)
    }
}

trait WindowsExt: Iterator + Sized {
    fn windows_of(self, size: usize) -> Windows<Self> where Self::Item: Clone {
        Windows::new(self, size)
    }
}
impl<I: Iterator> WindowsExt for I {}

// ─── Lazy Evaluation ─────────────────────────────────────────────────────────

struct Lazy<T, F: Fn() -> T> {
    f: F,
    cached: Option<T>,
}

impl<T: Clone, F: Fn() -> T> Lazy<T, F> {
    fn new(f: F) -> Self { Lazy { f, cached: None } }

    fn get(&mut self) -> &T {
        if self.cached.is_none() {
            println!("  [lazy: computing...]");
            self.cached = Some((self.f)());
        }
        self.cached.as_ref().unwrap()
    }
}

// ─── Monad-like Result chaining ───────────────────────────────────────────────

type Fallible<T> = Result<T, String>;

fn parse_int(s: &str) -> Fallible<i64> {
    s.trim().parse::<i64>().map_err(|e| format!("parse error: {}", e))
}

fn ensure_positive(n: i64) -> Fallible<i64> {
    if n > 0 { Ok(n) } else { Err(format!("{} is not positive", n)) }
}

fn ensure_less_than(limit: i64) -> impl Fn(i64) -> Fallible<i64> {
    move |n| if n < limit { Ok(n) } else { Err(format!("{} >= limit {}", n, limit)) }
}

fn process_input(s: &str) -> Fallible<String> {
    parse_int(s)
        .and_then(ensure_positive)
        .and_then(ensure_less_than(1000))
        .map(|n| format!("valid number: {}", n))
}

// ─── Higher-Order Functions ───────────────────────────────────────────────────

fn memoize<A, B, F>(mut f: F) -> impl FnMut(A) -> B
where
    A: std::hash::Hash + Eq + Clone,
    B: Clone,
    F: FnMut(A) -> B,
{
    let mut cache: HashMap<A, B> = HashMap::new();
    move |arg: A| {
        if let Some(cached) = cache.get(&arg) {
            cached.clone()
        } else {
            let result = f(arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}

fn retry<T, E: fmt::Display, F: FnMut() -> Result<T, E>>(
    mut f: F,
    attempts: usize,
) -> Result<T, E> {
    let mut last_err = None;
    for attempt in 1..=attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                println!("  Attempt {} failed: {}", attempt, e);
                last_err = Some(e);
            }
        }
    }
    Err(last_err.unwrap())
}

// ─── Data Transformation Pipeline ────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Record {
    name: String,
    score: f64,
    tags: Vec<String>,
}

impl Record {
    fn new(name: &str, score: f64, tags: Vec<&str>) -> Self {
        Record {
            name: name.to_string(),
            score,
            tags: tags.into_iter().map(|t| t.to_string()).collect(),
        }
    }
}

fn process_records(records: &[Record]) -> HashMap<String, Vec<&Record>> {
    // Group records by first tag, keeping only high-scorers
    let mut grouped: HashMap<String, Vec<&Record>> = HashMap::new();

    records
        .iter()
        .filter(|r| r.score >= 70.0)
        .for_each(|r| {
            let tag = r.tags.first().cloned().unwrap_or_else(|| "untagged".to_string());
            grouped.entry(tag).or_default().push(r);
        });

    // Sort each group by score descending
    for group in grouped.values_mut() {
        group.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }

    grouped
}

// ─── Iterator Recipes ─────────────────────────────────────────────────────────

fn demonstrate_iterator_recipes() {
    println!("\n── Iterator Recipes ──");

    let nums: Vec<i32> = (1..=20).collect();

    // Partition into two vecs in one pass
    let (evens, odds): (Vec<_>, Vec<_>) = nums.iter().partition(|&&x| x % 2 == 0);
    println!("evens: {:?}", evens);
    println!("odds:  {:?}", odds);

    // group consecutive by predicate
    let items = vec![1, 1, 2, 2, 2, 1, 3, 3];
    let mut grouped_consecutive: Vec<(i32, usize)> = Vec::new();
    for &item in &items {
        if grouped_consecutive.last().map_or(false, |(k, _)| *k == item) {
            grouped_consecutive.last_mut().unwrap().1 += 1;
        } else {
            grouped_consecutive.push((item, 1));
        }
    }
    println!("Run-length: {:?}", grouped_consecutive);

    // Flat-map example
    let sentences = vec!["hello world", "rust is great"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("Words: {:?}", words);

    // scan — running sum
    let running: Vec<i32> = (1..=5).scan(0, |acc, x| { *acc += x; Some(*acc) }).collect();
    println!("Running sum: {:?}", running);

    // unzip
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (numbers, letters): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    println!("Unzipped: {:?} {:?}", numbers, letters);

    // step_by + cycle
    let cycled: Vec<i32> = vec![1, 2, 3].into_iter().cycle().take(9).collect();
    println!("Cycled:   {:?}", cycled);

    // peekable
    let mut peekable = [10, 20, 30].iter().peekable();
    println!("Peek: {:?}", peekable.peek());
    println!("Next: {:?}", peekable.next());
    println!("Peek: {:?}", peekable.peek());

    // by_ref — consuming part of an iterator
    let mut iter = (0..10).into_iter();
    let first_three: Vec<_> = iter.by_ref().take(3).collect();
    let rest: Vec<_> = iter.collect();
    println!("first 3: {:?}", first_three);
    println!("rest:    {:?}", rest);
}

// ─── Closures as State Machines ───────────────────────────────────────────────

fn make_counter(step: i32) -> impl FnMut() -> i32 {
    let mut n = 0;
    move || {
        n += step;
        n
    }
}

fn make_accumulator() -> impl FnMut(i32) -> i32 {
    let mut total = 0;
    move |x| { total += x; total }
}

fn make_rate_limiter(limit: usize) -> impl FnMut() -> bool {
    let mut count = 0;
    move || {
        if count < limit { count += 1; true } else { false }
    }
}

// ─── Currying and Partial Application ────────────────────────────────────────

fn add(a: i32) -> impl Fn(i32) -> i32 { move |b| a + b }

fn multiply(a: i32) -> impl Fn(i32) -> i32 { move |b| a * b }

fn apply_twice<T, F: Fn(T) -> T>(f: F, x: T) -> T { f(f(x)) }

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Functional Patterns ===\n");

    // Compose
    println!("── Function Composition ──");
    let double     = |x: i32| x * 2;
    let add_three  = |x: i32| x + 3;
    let to_string  = |x: i32| format!("result={}", x);
    let pipeline   = compose(compose(double, add_three), to_string);
    println!("compose(double, add3, toString)(5) = {}", pipeline(5));

    // Pipeline
    let result = Pipeline::new(10)
        .tap(|v| println!("start: {}", v))
        .map(|x| x * 3)
        .tap(|v| println!("after *3: {}", v))
        .map(|x| x + 5)
        .tap(|v| println!("after +5: {}", v))
        .finish();
    println!("pipeline result: {}", result);

    // Currying
    println!("\n── Currying ──");
    let add5    = add(5);
    let double2 = multiply(2);
    println!("add5(10) = {}", add5(10));
    println!("apply_twice(add5, 0) = {}", apply_twice(add5, 0));
    println!("apply_twice(double, 3) = {}", apply_twice(double2, 3));

    // Closures as state
    println!("\n── Closures as State ──");
    let mut count_by_3 = make_counter(3);
    println!("counter: {} {} {}", count_by_3(), count_by_3(), count_by_3());

    let mut acc = make_accumulator();
    for x in [10, 20, 5, 15] { println!("acc({}): {}", x, acc(x)); }

    let mut limiter = make_rate_limiter(3);
    for i in 1..=5 {
        println!("request {}: {}", i, if limiter() { "OK" } else { "DENIED" });
    }

    // Running total
    println!("\n── Running Total ──");
    let vals: Vec<f64> = vec![1.5, 2.0, 0.5, 3.0, 1.0];
    let totals: Vec<f64> = vals.into_iter().running_total().collect();
    println!("running totals: {:?}", totals);

    // Windowed
    println!("\n── Windows ──");
    let data = vec![1, 2, 3, 4, 5, 6];
    let windows: Vec<_> = data.into_iter().windows_of(3).collect();
    for w in &windows {
        println!("  {:?}  avg={:.2}", w, w.iter().sum::<i32>() as f64 / w.len() as f64);
    }

    // Memoize
    println!("\n── Memoization ──");
    let mut fib_memo = memoize(|n: u64| {
        println!("  computing fib({})...", n);
        // Simple non-recursive for demo
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..n { let tmp = b; b = a + b; a = tmp; }
        a
    });
    println!("fib(10) = {}", fib_memo(10));
    println!("fib(10) = {} (cached)", fib_memo(10)); // no recompute
    println!("fib(15) = {}", fib_memo(15));

    // Retry
    println!("\n── Retry ──");
    let mut attempt = 0;
    let result = retry(|| {
        attempt += 1;
        if attempt < 3 { Err("not ready yet") } else { Ok("success!") }
    }, 5);
    println!("Result: {:?}", result);

    // Lazy
    println!("\n── Lazy Evaluation ──");
    let mut lazy_val = Lazy::new(|| {
        (1..=1000).sum::<u64>()
    });
    println!("Before get — nothing computed yet");
    println!("lazy val = {}", lazy_val.get());
    println!("again   = {} (cached)", lazy_val.get());

    // Result chaining (monad-like)
    println!("\n── Monadic Result Chaining ──");
    for input in ["42", "-5", "5000", "not_a_number", "100"] {
        match process_input(input) {
            Ok(msg) => println!("  {:12} → {}", input, msg),
            Err(e)  => println!("  {:12} → Error: {}", input, e),
        }
    }

    // Data pipeline
    println!("\n── Record Processing Pipeline ──");
    let records = vec![
        Record::new("Alice",   95.0, vec!["rust", "systems"]),
        Record::new("Bob",     60.0, vec!["rust", "web"]),
        Record::new("Charlie", 88.0, vec!["python", "ml"]),
        Record::new("Diana",   72.0, vec!["rust", "embedded"]),
        Record::new("Eve",     55.0, vec!["python", "data"]),
        Record::new("Frank",   91.0, vec!["python", "ml"]),
    ];

    let grouped = process_records(&records);
    let mut keys: Vec<_> = grouped.keys().collect();
    keys.sort();
    for tag in keys {
        println!("  [{}]", tag);
        for r in &grouped[tag] {
            println!("    {} ({:.0})", r.name, r.score);
        }
    }

    demonstrate_iterator_recipes();

    println!("\n=== Done ===");
}