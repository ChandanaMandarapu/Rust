// File 20: Advanced Iterators, Lazy Evaluation & State Machines
// Custom adaptors, pull-based streaming, coroutine-style generators,
// stateful machines, parser combinators, and infinite sequences

use std::collections::HashMap;
use std::fmt;

// ─── Infinite Lazy Sequences ──────────────────────────────────────────────────

struct Iterate<T, F> {
    state: T,
    f:     F,
}

impl<T: Clone, F: FnMut(T) -> T> Iterate<T, F> {
    fn new(seed: T, f: F) -> Self { Iterate { state: seed, f } }
}

impl<T: Clone, F: FnMut(T) -> T> Iterator for Iterate<T, F> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let current = self.state.clone();
        self.state = (self.f)(self.state.clone());
        Some(current)
    }
}

fn iterate<T: Clone, F: FnMut(T) -> T>(seed: T, f: F) -> Iterate<T, F> {
    Iterate::new(seed, f)
}

// Powers of 2
struct Unfold<S, T, F> {
    state: S,
    f:     F,
    _t:    std::marker::PhantomData<T>,
}

impl<S, T, F: FnMut(&mut S) -> Option<T>> Unfold<S, T, F> {
    fn new(state: S, f: F) -> Self {
        Unfold { state, f, _t: std::marker::PhantomData }
    }
}

impl<S, T, F: FnMut(&mut S) -> Option<T>> Iterator for Unfold<S, T, F> {
    type Item = T;
    fn next(&mut self) -> Option<T> { (self.f)(&mut self.state) }
}

fn unfold<S, T, F: FnMut(&mut S) -> Option<T>>(state: S, f: F) -> Unfold<S, T, F> {
    Unfold::new(state, f)
}

// ─── Stateful Iterator Adaptors ───────────────────────────────────────────────

struct Tap<I: Iterator, F> {
    iter: I,
    f:    F,
}

impl<I: Iterator, F: FnMut(&I::Item)> Iterator for Tap<I, F> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        let item = self.iter.next()?;
        (self.f)(&item);
        Some(item)
    }
}

struct ScanState<I: Iterator, S, B, F> {
    iter:  I,
    state: S,
    f:     F,
    _b:    std::marker::PhantomData<B>,
}

impl<I: Iterator, S, B, F: FnMut(&mut S, I::Item) -> B> Iterator for ScanState<I, S, B, F> {
    type Item = B;
    fn next(&mut self) -> Option<B> {
        let item = self.iter.next()?;
        Some((self.f)(&mut self.state, item))
    }
}

// Deduplicate consecutive equal elements
struct Dedup<I: Iterator> {
    iter: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Iterator for Dedup<I>
where I::Item: PartialEq + Clone
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let item = self.iter.next()?;
            if self.last.as_ref().map_or(true, |l| l != &item) {
                self.last = Some(item.clone());
                return Some(item);
            }
        }
    }
}

// Group consecutive elements by a key
struct GroupBy<I: Iterator, K, F> {
    iter: std::iter::Peekable<I>,
    key_fn: F,
    _k: std::marker::PhantomData<K>,
}

impl<I: Iterator, K: PartialEq, F: FnMut(&I::Item) -> K> Iterator for GroupBy<I, K, F>
where
    I::Item: Clone,
{
    type Item = (K, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let key   = (self.key_fn)(&first);
        let mut group = vec![first];

        while let Some(next) = self.iter.peek() {
            if (self.key_fn)(next) == key {
                group.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        Some((key, group))
    }
}

// Extension trait to hang all adaptors off .iter()
trait IterExt: Iterator + Sized {
    fn tap<F: FnMut(&Self::Item)>(self, f: F) -> Tap<Self, F> {
        Tap { iter: self, f }
    }

    fn scan_state<S, B, F: FnMut(&mut S, Self::Item) -> B>(
        self, init: S, f: F
    ) -> ScanState<Self, S, B, F> {
        ScanState { iter: self, state: init, f, _b: std::marker::PhantomData }
    }

    fn dedup(self) -> Dedup<Self> where Self::Item: PartialEq + Clone {
        Dedup { iter: self, last: None }
    }

    fn group_by<K: PartialEq, F: FnMut(&Self::Item) -> K>(
        self, f: F
    ) -> GroupBy<Self, K, F> where Self::Item: Clone {
        GroupBy { iter: self.peekable(), key_fn: f, _k: std::marker::PhantomData }
    }

    fn sum_by<F, T>(self, f: F) -> T
    where
        F: Fn(Self::Item) -> T,
        T: Default + std::ops::Add<Output = T>,
    {
        self.map(f).fold(T::default(), |a, b| a + b)
    }

    fn min_max(self) -> Option<(Self::Item, Self::Item)>
    where Self::Item: Ord + Clone
    {
        self.fold(None::<(Self::Item, Self::Item)>, |acc, x| {
            Some(match acc {
                None => (x.clone(), x),
                Some((mn, mx)) => (
                    if x < mn { x.clone() } else { mn },
                    if x > mx { x } else { mx },
                ),
            })
        })
    }
}

impl<I: Iterator> IterExt for I {}

// ─── Parser Combinator ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    Unexpected(String),
    UnexpectedEof,
    ExpectedChar(char),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Unexpected(s)    => write!(f, "unexpected: {}", s),
            ParseError::UnexpectedEof    => write!(f, "unexpected end of input"),
            ParseError::ExpectedChar(c)  => write!(f, "expected '{}'", c),
        }
    }
}

type ParseResult<'a, T> = Result<(T, &'a str), ParseError>;

// A parser is a function: &str -> ParseResult<T>
struct Parser<T>(Box<dyn Fn(&str) -> ParseResult<T>>);

impl<T: 'static> Parser<T> {
    fn new<F: Fn(&str) -> ParseResult<T> + 'static>(f: F) -> Self {
        Parser(Box::new(f))
    }

    fn parse<'a>(&self, input: &'a str) -> ParseResult<'a, T> {
        (self.0)(input)
    }

    fn map<U: 'static, F: Fn(T) -> U + 'static>(self, f: F) -> Parser<U> {
        Parser::new(move |input| {
            let (val, rest) = self.parse(input)?;
            Ok((f(val), rest))
        })
    }

    fn and_then<U: 'static, F: Fn(T) -> Parser<U> + 'static>(self, f: F) -> Parser<U> {
        Parser::new(move |input| {
            let (val, rest) = self.parse(input)?;
            f(val).parse(rest)
        })
    }

    fn or(self, other: Parser<T>) -> Parser<T> {
        Parser::new(move |input| {
            self.parse(input).or_else(|_| other.parse(input))
        })
    }

    fn many(self) -> Parser<Vec<T>> {
        Parser::new(move |input| {
            let mut results = vec![];
            let mut remaining = input;
            while let Ok((val, rest)) = self.parse(remaining) {
                results.push(val);
                remaining = rest;
            }
            Ok((results, remaining))
        })
    }
}

fn char_parser(c: char) -> Parser<char> {
    Parser::new(move |input: &str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(ch) if ch == c => Ok((c, chars.as_str())),
            Some(ch) => Err(ParseError::Unexpected(ch.to_string())),
            None => Err(ParseError::UnexpectedEof),
        }
    })
}

fn digit() -> Parser<char> {
    Parser::new(|input: &str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c.is_ascii_digit() => Ok((c, chars.as_str())),
            Some(c) => Err(ParseError::Unexpected(c.to_string())),
            None => Err(ParseError::UnexpectedEof),
        }
    })
}

fn alpha() -> Parser<char> {
    Parser::new(|input: &str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c.is_alphabetic() => Ok((c, chars.as_str())),
            Some(c) => Err(ParseError::Unexpected(c.to_string())),
            None => Err(ParseError::UnexpectedEof),
        }
    })
}

fn integer() -> Parser<i64> {
    let digits = digit().many();
    digits.map(|ds| {
        ds.iter().fold(0i64, |acc, &c| acc * 10 + (c as i64 - '0' as i64))
    })
}

fn identifier() -> Parser<String> {
    let first = alpha();
    let rest  = alpha().or(digit()).many();
    first.and_then(|c| rest.map(move |cs| {
        let mut s = String::new();
        s.push(c);
        s.extend(cs.iter());
        s
    }))
}

// ─── Coroutine-Style Generator (via enum state machine) ──────────────────────

#[derive(Debug)]
enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

// A range generator that yields values one at a time
struct RangeGen {
    current: i64,
    end:     i64,
    step:    i64,
}

impl RangeGen {
    fn new(start: i64, end: i64, step: i64) -> Self {
        RangeGen { current: start, end, step }
    }

    fn resume(&mut self) -> GeneratorState<i64, i64> {
        if self.current >= self.end {
            GeneratorState::Complete(self.current)
        } else {
            let val = self.current;
            self.current += self.step;
            GeneratorState::Yielded(val)
        }
    }
}

// A more complex generator: produces (n, fib(n)) pairs
struct FibGen {
    n: u64,
    a: u64,
    b: u64,
    max: u64,
}

impl FibGen {
    fn new(max: u64) -> Self { FibGen { n: 0, a: 0, b: 1, max } }

    fn resume(&mut self) -> GeneratorState<(u64, u64), ()> {
        if self.n >= self.max {
            return GeneratorState::Complete(());
        }
        let result = (self.n, self.a);
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        self.n += 1;
        GeneratorState::Yielded(result)
    }
}

// ─── Data Flow Graph ──────────────────────────────────────────────────────────

trait Transform<In, Out>: fmt::Debug {
    fn apply(&self, input: In) -> Out;
    fn name(&self) -> &str;
}

#[derive(Debug)]
struct MapStep<F>(String, F);
#[derive(Debug)]
struct FilterStep<F>(String, F);

impl<F: Fn(i32) -> i32 + fmt::Debug> Transform<Vec<i32>, Vec<i32>> for MapStep<F> {
    fn apply(&self, input: Vec<i32>) -> Vec<i32> { input.into_iter().map(&self.1).collect() }
    fn name(&self) -> &str { &self.0 }
}

impl<F: Fn(&i32) -> bool + fmt::Debug> Transform<Vec<i32>, Vec<i32>> for FilterStep<F> {
    fn apply(&self, input: Vec<i32>) -> Vec<i32> { input.into_iter().filter(&self.1).collect() }
    fn name(&self) -> &str { &self.0 }
}

struct DataPipeline {
    steps: Vec<Box<dyn Transform<Vec<i32>, Vec<i32>>>>,
}

impl DataPipeline {
    fn new() -> Self { DataPipeline { steps: vec![] } }

    fn add_step<T: Transform<Vec<i32>, Vec<i32>> + 'static>(&mut self, step: T) -> &mut Self {
        self.steps.push(Box::new(step));
        self
    }

    fn run(&self, input: Vec<i32>) -> Vec<i32> {
        let mut data = input;
        for step in &self.steps {
            println!("  [{}] {:?}", step.name(), &data);
            data = step.apply(data);
        }
        data
    }
}

// ─── Event-Driven State Machine ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum TrafficLight { Red, Yellow, Green }

#[derive(Debug, Clone)]
enum LightEvent { Timer, Emergency, AllClear }

struct LightMachine {
    state: TrafficLight,
    transitions: HashMap<String, TrafficLight>,
}

impl LightMachine {
    fn new() -> Self {
        let mut t = HashMap::new();
        t.insert("Red:Timer".to_string(), TrafficLight::Green);
        t.insert("Green:Timer".to_string(), TrafficLight::Yellow);
        t.insert("Yellow:Timer".to_string(), TrafficLight::Red);
        t.insert("Red:Emergency".to_string(), TrafficLight::Red);
        t.insert("Green:Emergency".to_string(), TrafficLight::Red);
        t.insert("Yellow:Emergency".to_string(), TrafficLight::Red);
        LightMachine { state: TrafficLight::Red, transitions: t }
    }

    fn transition(&mut self, event: &LightEvent) {
        let key = format!("{:?}:{:?}", self.state, event);
        if let Some(next) = self.transitions.get(&key) {
            println!("  {:?} + {:?} → {:?}", self.state, event, next);
            self.state = next.clone();
        } else {
            println!("  {:?} + {:?} → (no transition)", self.state, event);
        }
    }

    fn current(&self) -> &TrafficLight { &self.state }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Advanced Iterators & State Machines ===\n");

    // Infinite sequences
    println!("── Infinite Sequences ──");
    let powers_of_2: Vec<u64> = iterate(1u64, |x| x * 2).take(10).collect();
    println!("Powers of 2: {:?}", powers_of_2);

    let collatz = unfold(27u64, |n| {
        if *n == 1 { return None; }
        let v = *n;
        *n = if v % 2 == 0 { v / 2 } else { 3 * v + 1 };
        Some(v)
    });
    let seq: Vec<_> = collatz.collect();
    println!("Collatz(27): len={}, starts={:?}...", seq.len(), &seq[..8]);

    // Custom adaptors
    println!("\n── Custom Adaptors ──");
    let data = vec![1, 1, 2, 2, 2, 3, 1, 1];
    let deduped: Vec<_> = data.iter().copied().dedup().collect();
    println!("dedup {:?} → {:?}", data, deduped);

    let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let grouped: Vec<_> = items.iter().copied()
        .group_by(|&x| if x % 2 == 0 { "even" } else { "odd" })
        .collect();
    println!("grouped by parity:");
    for (k, v) in &grouped { println!("  {}: {:?}", k, v); }

    let with_running_sum: Vec<_> = (1..=6)
        .scan_state(0i32, |acc, x| { *acc += x; *acc })
        .collect();
    println!("running sum: {:?}", with_running_sum);

    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (mn, mx) = nums.iter().copied().min_max().unwrap();
    println!("min={}, max={}", mn, mx);

    // Parser combinators
    println!("\n── Parser Combinators ──");
    let int_parser = integer();
    let id_parser  = identifier();

    let cases: Vec<(&str, &str)> = vec![
        ("12345 rest", "integer"),
        ("hello123 world", "identifier"),
        ("42abc", "integer"),
    ];

    for (input, kind) in &cases {
        if *kind == "integer" {
            match int_parser.parse(input) {
                Ok((n, rest)) => println!("  int: {} | rest='{}'", n, rest),
                Err(e) => println!("  err: {}", e),
            }
        } else {
            match id_parser.parse(input) {
                Ok((s, rest)) => println!("  id: {} | rest='{}'", s, rest),
                Err(e) => println!("  err: {}", e),
            }
        }
    }

    // Comma-separated integers
    let comma_ints = integer().map(|n| vec![n]).and_then(|first| {
        char_parser(',').and_then(|_| integer())
            .many()
            .map(move |rest| {
                let mut v = first.clone();
                v.extend(rest);
                v
            })
    });
    match comma_ints.parse("1,2,3,4,5 done") {
        Ok((nums, rest)) => println!("  parsed ints: {:?} | rest='{}'", nums, rest),
        Err(e) => println!("  err: {}", e),
    }

    // Generator
    println!("\n── Generators ──");
    let mut gen = RangeGen::new(0, 10, 3);
    print!("range(0..10, step=3): ");
    loop {
        match gen.resume() {
            GeneratorState::Yielded(v)   => print!("{} ", v),
            GeneratorState::Complete(v)  => { println!("(done at {})", v); break; }
        }
    }

    let mut fib = FibGen::new(8);
    println!("fib pairs:");
    loop {
        match fib.resume() {
            GeneratorState::Yielded((n, f)) => println!("  fib({}) = {}", n, f),
            GeneratorState::Complete(())    => break,
        }
    }

    // Data pipeline
    println!("\n── Data Pipeline ──");
    let mut pipeline = DataPipeline::new();
    pipeline
        .add_step(FilterStep("keep_positive".to_string(), |&x: &i32| x > 0))
        .add_step(MapStep("double".to_string(), |x| x * 2))
        .add_step(FilterStep("lt_20".to_string(), |&x: &i32| x < 20))
        .add_step(MapStep("add_one".to_string(), |x| x + 1));
    let input = vec![-3, 1, 4, -1, 5, 9, 2, -6, 7];
    println!("  input: {:?}", input);
    let output = pipeline.run(input);
    println!("  output: {:?}", output);

    // State machine
    println!("\n── Traffic Light State Machine ──");
    let mut light = LightMachine::new();
    println!("  initial: {:?}", light.current());
    let events = vec![
        LightEvent::Timer,
        LightEvent::Timer,
        LightEvent::Emergency,
        LightEvent::AllClear,
        LightEvent::Timer,
    ];
    for e in &events { light.transition(e); }
    println!("  final: {:?}", light.current());

    println!("\n=== Done ===");
}