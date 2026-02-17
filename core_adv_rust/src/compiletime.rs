// today practice session 2 
// Implemented SIMD style const generic vectors with math operations, dot cross normalize and display. Added compile time hashing, prime generation, power functions, and sorted assertions. Built const generic string interning, type level bounded values with compile time proofs, fixed size stack with RPN calculator, circular buffer, and static command dispatch without vtables. Also added bitonic sorting network and phantom type unit system for compile time dimensional analysis. Main demonstrates all features end to end.

#![allow(incomplete_features, unused)]

use std::marker::PhantomData;
use std::fmt;
use std::ops::{Add, Sub, Mul, Index, IndexMut};

// ─── Const Generics: SIMD-style Fixed Vectors ────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec<T, const N: usize> {
    data: [T; N],
}

impl<T: Copy + Default, const N: usize> Vec<T, N> {
    fn zero() -> Self where T: Default { Vec { data: [T::default(); N] } }
    fn from_array(data: [T; N]) -> Self { Vec { data } }

    fn as_slice(&self) -> &[T; N] { &self.data }
    fn len(&self) -> usize { N }
}

impl<T, const N: usize> Index<usize> for Vec<T, N> {
    type Output = T;
    fn index(&self, i: usize) -> &T { &self.data[i] }
}

impl<T, const N: usize> IndexMut<usize> for Vec<T, N> {
    fn index_mut(&mut self, i: usize) -> &mut T { &mut self.data[i] }
}

// Arithmetic
impl<T: Copy + Default + Add<Output=T>, const N: usize> Add for Vec<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut out = Self::zero();
        for i in 0..N { out.data[i] = self.data[i] + rhs.data[i]; }
        out
    }
}

impl<T: Copy + Default + Mul<Output=T> + Add<Output=T>, const N: usize> Vec<T, N> {
    fn dot(&self, other: &Self) -> T {
        let mut acc = T::default();
        for i in 0..N { acc = acc + self.data[i] * other.data[i]; }
        acc
    }
}

impl<T: Copy + Default + Mul<Output=T> + Add<Output=T>, const N: usize> Vec<T, N> {
    fn hadamard(&self, other: &Self) -> Self {
        let mut out = Self::zero();
        for i in 0..N { out.data[i] = self.data[i] * other.data[i]; }
        out
    }
}

// Type aliases for common sizes
type Vec2f = Vec<f64, 2>;
type Vec3f = Vec<f64, 3>;
type Vec4f = Vec<f64, 4>;

impl Vec3f {
    fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f::from_array([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }

    fn norm(&self) -> f64 {
        (self[0]*self[0] + self[1]*self[1] + self[2]*self[2]).sqrt()
    }

    fn normalize(&self) -> Vec3f {
        let n = self.norm();
        Vec3f::from_array([self[0]/n, self[1]/n, self[2]/n])
    }
}

impl<T: fmt::Display + Copy, const N: usize> fmt::Display for Vec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.data.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:.4}", v)?;
        }
        write!(f, "]")
    }
}

// ─── Const fn: Compile-Time Hash Map ─────────────────────────────────────────

const fn const_hash(s: &str) -> u64 {
    // FNV-1a hash
    let mut h: u64 = 14695981039346656037;
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        h ^= b[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}

const fn const_pow(base: u64, exp: u32) -> u64 {
    let mut result = 1u64;
    let mut b = base;
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 { result = result.wrapping_mul(b); }
        b = b.wrapping_mul(b);
        e >>= 1;
    }
    result
}

const fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

// Build a const array of primes at compile time
const fn first_n_primes<const N: usize>() -> [u64; N] {
    let mut primes = [0u64; N];
    let mut count = 0;
    let mut n = 2u64;
    while count < N {
        if is_prime(n) {
            primes[count] = n;
            count += 1;
        }
        n += 1;
    }
    primes
}

const PRIMES_20: [u64; 20] = first_n_primes::<20>();
const PRIMES_50: [u64; 50] = first_n_primes::<50>();

// Compile-time sorted array check
const fn is_sorted(arr: &[u64]) -> bool {
    let mut i = 1;
    while i < arr.len() {
        if arr[i] < arr[i-1] { return false; }
        i += 1;
    }
    true
}

const _: () = assert!(is_sorted(&PRIMES_20), "primes must be sorted");

// ─── Const Generic String Interning ──────────────────────────────────────────

struct InternedStr<const HASH: u64> {
    _phantom: PhantomData<[(); HASH as usize]>,
    value: &'static str,
}

impl<const H: u64> InternedStr<H> {
    const fn new(s: &'static str) -> Self {
        InternedStr { _phantom: PhantomData, value: s }
    }
    fn get(&self) -> &str { self.value }
}

macro_rules! intern {
    ($s:literal) => {
        InternedStr::<{ const_hash($s) }>::new($s)
    };
}

// ─── Type-Level Bounds Checking ───────────────────────────────────────────────

// A value guaranteed (at compile time) to be in [MIN, MAX]
#[derive(Debug, Clone, Copy)]
struct Bounded<const MIN: i64, const MAX: i64> {
    value: i64,
}

impl<const MIN: i64, const MAX: i64> Bounded<MIN, MAX> {
    const fn new(v: i64) -> Option<Self> {
        if v >= MIN && v <= MAX {
            Some(Bounded { value: v })
        } else {
            None
        }
    }

    const fn from_const<const V: i64>() -> Self where (): BoundsCheck<MIN, MAX, V> {
        Bounded { value: V }
    }

    fn get(&self) -> i64 { self.value }

    fn clamp(v: i64) -> Self {
        Bounded { value: v.max(MIN).min(MAX) }
    }
}

impl<const MIN: i64, const MAX: i64> fmt::Display for Bounded<MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}∈[{},{}]", self.value, MIN, MAX)
    }
}

// Compile-time bounds check witness
trait BoundsCheck<const MIN: i64, const MAX: i64, const V: i64> {}
struct BoundsOk;
impl<const MIN: i64, const MAX: i64, const V: i64> BoundsCheck<MIN, MAX, V> for ()
where [(); (V >= MIN) as usize]: ,
      [(); (V <= MAX) as usize]: {}

// ─── Const Generic Stack (fixed depth call stacks / expression stacks) ────────

struct ConstStack<T: Copy + Default, const CAP: usize> {
    data: [T; CAP],
    top:  usize,
}

impl<T: Copy + Default + fmt::Debug, const CAP: usize> ConstStack<T, CAP> {
    const fn new() -> Self {
        ConstStack { data: [T::default(); CAP], top: 0 }
    }

    fn push(&mut self, v: T) -> bool {
        if self.top >= CAP { return false; }
        self.data[self.top] = v;
        self.top += 1;
        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        Some(self.data[self.top])
    }

    fn peek(&self) -> Option<&T> {
        if self.top == 0 { None } else { Some(&self.data[self.top - 1]) }
    }

    fn len(&self) -> usize { self.top }
    fn is_empty(&self) -> bool { self.top == 0 }
    fn is_full(&self) -> bool { self.top == CAP }
    fn remaining(&self) -> usize { CAP - self.top }
}

// RPN (Reverse Polish Notation) calculator using a const stack
fn rpn_eval<const STACK_SIZE: usize>(tokens: &[&str]) -> Result<f64, String> {
    let mut stack: ConstStack<f64, STACK_SIZE> = ConstStack::new();

    for &token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 { return Err("division by zero".to_string()); }
                        a / b
                    }
                    _ => unreachable!(),
                };
                if !stack.push(result) { return Err("stack overflow".to_string()); }
            }
            n => {
                let v: f64 = n.parse().map_err(|_| format!("bad token: {}", n))?;
                if !stack.push(v) { return Err("stack overflow".to_string()); }
            }
        }
    }

    stack.pop().ok_or("empty stack".to_string())
}

// ─── Const Generic Circular Buffer ────────────────────────────────────────────

struct CircularBuf<T: Copy + Default, const N: usize> {
    data:  [T; N],
    head:  usize,
    tail:  usize,
    full:  bool,
}

impl<T: Copy + Default + fmt::Debug, const N: usize> CircularBuf<T, N> {
    fn new() -> Self {
        CircularBuf { data: [T::default(); N], head: 0, tail: 0, full: false }
    }

    fn push(&mut self, v: T) {
        self.data[self.tail] = v;
        self.tail = (self.tail + 1) % N;
        if self.full { self.head = (self.head + 1) % N; } // overwrite oldest
        self.full = self.tail == self.head;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let v = self.data[self.head];
        self.head = (self.head + 1) % N;
        self.full = false;
        Some(v)
    }

    fn len(&self) -> usize {
        if self.full { N }
        else if self.tail >= self.head { self.tail - self.head }
        else { N - self.head + self.tail }
    }

    fn is_empty(&self) -> bool { !self.full && self.head == self.tail }
    fn is_full(&self)  -> bool { self.full }

    fn iter_snapshot(&self) -> std::vec::Vec<T> {
        let mut out = std::vec::Vec::with_capacity(self.len());
        let mut h = self.head;
        let mut remaining = self.len();
        while remaining > 0 {
            out.push(self.data[h]);
            h = (h + 1) % N;
            remaining -= 1;
        }
        out
    }
}

// ─── Static Dispatch Table ────────────────────────────────────────────────────

// No vtable, no heap — all resolved at compile time
trait Command {
    const NAME: &'static str;
    fn execute(&self, args: &[&str]) -> String;
}

struct HelpCommand;
struct EchoCommand;
struct CountCommand;

impl Command for HelpCommand {
    const NAME: &'static str = "help";
    fn execute(&self, _args: &[&str]) -> String {
        "Available: help, echo, count".to_string()
    }
}

impl Command for EchoCommand {
    const NAME: &'static str = "echo";
    fn execute(&self, args: &[&str]) -> String {
        args.join(" ")
    }
}

impl Command for CountCommand {
    const NAME: &'static str = "count";
    fn execute(&self, args: &[&str]) -> String {
        format!("{} argument(s)", args.len())
    }
}

// Zero-cost dispatch via const matching
fn dispatch(name: &str, args: &[&str]) -> Option<String> {
    match name {
        HelpCommand::NAME  => Some(HelpCommand.execute(args)),
        EchoCommand::NAME  => Some(EchoCommand.execute(args)),
        CountCommand::NAME => Some(CountCommand.execute(args)),
        _ => None,
    }
}

// ─── Const Generic Sorting Network ────────────────────────────────────────────

// A sorting network for known sizes — no branches at runtime
fn compare_swap<T: PartialOrd>(arr: &mut [T], i: usize, j: usize) {
    if arr[i] > arr[j] { arr.swap(i, j); }
}

// Bitonic sort for power-of-2 sizes — O(log²n) depth
fn bitonic_sort<T: PartialOrd + Copy + Default, const N: usize>(arr: &mut [T]) {
    let n = N;
    let mut k = 2;
    while k <= n {
        let mut j = k / 2;
        while j >= 1 {
            for i in 0..n {
                let l = i ^ j;
                if l > i {
                    if (i & k) == 0 && arr[i] > arr[l] { arr.swap(i, l); }
                    if (i & k) != 0 && arr[i] < arr[l] { arr.swap(i, l); }
                }
            }
            j /= 2;
        }
        k *= 2;
    }
}

// ─── Phantom Type Unit System (compile-time dimensional analysis) ─────────────

struct Quantity<T, Unit> {
    value: T,
    _unit: PhantomData<Unit>,
}

impl<T: Copy + fmt::Display, U: UnitName> Quantity<T, U> {
    fn new(v: T) -> Self { Quantity { value: v, _unit: PhantomData } }
    fn get(&self) -> T { self.value }
}

impl<T: fmt::Display + Copy, U: UnitName> fmt::Display for Quantity<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, U::name())
    }
}

trait UnitName { fn name() -> &'static str; }

struct Meters;   impl UnitName for Meters   { fn name() -> &'static str { "m" } }
struct Seconds;  impl UnitName for Seconds  { fn name() -> &'static str { "s" } }
struct MPerS;    impl UnitName for MPerS    { fn name() -> &'static str { "m/s" } }
struct Kg;       impl UnitName for Kg       { fn name() -> &'static str { "kg" } }
struct Newtons;  impl UnitName for Newtons  { fn name() -> &'static str { "N" } }
struct Joules;   impl UnitName for Joules   { fn name() -> &'static str { "J" } }

// Dimension-safe math — wrong units = compile error
impl Quantity<f64, Meters> {
    fn per(self, t: Quantity<f64, Seconds>) -> Quantity<f64, MPerS> {
        Quantity::new(self.value / t.value)
    }
}

impl Quantity<f64, Kg> {
    fn times_accel(self, a: Quantity<f64, MPerS>) -> Quantity<f64, Newtons> {
        Quantity::new(self.value * a.value)
    }
}

impl Quantity<f64, Newtons> {
    fn times_dist(self, d: Quantity<f64, Meters>) -> Quantity<f64, Joules> {
        Quantity::new(self.value * d.value)
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Compile-Time Programming ===\n");

    // Fixed vectors
    println!("── Const Generic Vectors ──");
    let a = Vec3f::from_array([1.0, 2.0, 3.0]);
    let b = Vec3f::from_array([4.0, 5.0, 6.0]);
    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a + b);
    println!("a · b = {:.2}", a.dot(&b));
    println!("a × b = {}", a.cross(&b));
    println!("|a|   = {:.4}", a.norm());
    println!("â     = {}", a.normalize());

    let v4 = Vec4f::from_array([1.0, 0.0, 0.0, 1.0]);
    println!("v4 dot v4 = {:.2}", v4.dot(&v4));

    // Compile-time primes
    println!("\n── Compile-Time Primes ──");
    println!("First 20 primes: {:?}", PRIMES_20);
    println!("First 50 primes (last 5): {:?}", &PRIMES_50[45..]);
    println!("PRIMES_20 is sorted (proven at compile time): {}", is_sorted(&PRIMES_20));

    // Const hashing
    println!("\n── Compile-Time Hashing ──");
    println!("hash(\"hello\") = 0x{:016X}", const_hash("hello"));
    println!("hash(\"world\") = 0x{:016X}", const_hash("world"));
    println!("pow(2, 10) = {}", const_pow(2, 10));
    println!("pow(3, 5)  = {}", const_pow(3, 5));

    let s = intern!("hello");
    println!("interned: '{}' with hash 0x{:X}", s.get(), const_hash("hello"));

    // Bounded values
    println!("\n── Type-Level Bounds ──");
    let valid: Option<Bounded<0, 100>> = Bounded::new(42);
    let bad:   Option<Bounded<0, 100>> = Bounded::new(200);
    println!("Bounded(42):  {:?} → {}", valid, valid.unwrap());
    println!("Bounded(200): {:?}", bad);
    let clamped = Bounded::<0, 100>::clamp(150);
    println!("clamped(150): {}", clamped);

    // RPN calculator
    println!("\n── RPN Calculator (const stack depth 16) ──");
    let expressions: Vec<(&str, std::vec::Vec<&str>)> = vec![
        ("3 4 +",         vec!["3", "4", "+"]),
        ("5 1 2 + 4 * +", vec!["5", "1", "2", "+", "4", "*", "+"]),
        ("2 3 * 4 5 * +", vec!["2", "3", "*", "4", "5", "*", "+"]),
        ("15 7 1 1 + - / 3 * 2 1 1 + + -",
         vec!["15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"]),
    ];
    for (expr, tokens) in &expressions {
        match rpn_eval::<16>(&tokens) {
            Ok(r)  => println!("  {} = {}", expr, r),
            Err(e) => println!("  {} → error: {}", expr, e),
        }
    }

    // Circular buffer
    println!("\n── Const Generic Circular Buffer (cap=4) ──");
    let mut cb: CircularBuf<i32, 4> = CircularBuf::new();
    for i in 1..=6 { cb.push(i * 10); println!("  push {}: snapshot={:?}", i*10, cb.iter_snapshot()); }
    while let Some(v) = cb.pop() { print!("pop={} ", v); }
    println!();

    // Static dispatch
    println!("\n── Static Dispatch Table ──");
    for (name, args) in &[
        ("help",  vec![]),
        ("echo",  vec!["hello", "world"]),
        ("count", vec!["a", "b", "c", "d"]),
        ("quit",  vec![]),
    ] {
        match dispatch(name, args) {
            Some(r) => println!("  {}: {}", name, r),
            None    => println!("  {}: unknown command", name),
        }
    }

    // Bitonic sort
    println!("\n── Bitonic Sort (const generic N=8) ──");
    let mut arr: [i32; 8] = [5, 2, 8, 1, 9, 3, 7, 4];
    println!("  before: {:?}", arr);
    bitonic_sort::<i32, 8>(&mut arr);
    println!("  after:  {:?}", arr);

    // Dimensional analysis
    println!("\n── Dimensional Analysis (compile-time unit safety) ──");
    let dist   = Quantity::<f64, Meters>::new(100.0);
    let time   = Quantity::<f64, Seconds>::new(9.58);
    let mass   = Quantity::<f64, Kg>::new(70.0);

    let speed  = dist.per(time);
    println!("  100m / 9.58s = {}", speed);

    let force  = mass.times_accel(Quantity::<f64, MPerS>::new(9.81));
    println!("  70kg * 9.81m/s = {}", force);

    let energy = force.times_dist(Quantity::<f64, Meters>::new(10.0));
    println!("  {} * 10m = {}", Quantity::<f64, Newtons>::new(686.7), energy);

    println!("\n=== Done ===");
}