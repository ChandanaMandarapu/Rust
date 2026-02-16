// File 19: Type System Wizardry
// Const generics, GATs (Generic Associated Types), type-level state machines,
// zero-cost abstractions, newtype stacks, sealed traits, and type-driven design

#![allow(unused)]

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Index};

// ─── Const Generics ───────────────────────────────────────────────────────────

// A fixed-size vector backed by a stack array — no heap allocation
#[derive(Debug, Clone, Copy)]
struct FixedVec<T, const N: usize> {
    data: [T; N],
    len:  usize,
}

impl<T: Copy + Default, const N: usize> FixedVec<T, N> {
    fn new() -> Self {
        FixedVec { data: [T::default(); N], len: 0 }
    }

    fn push(&mut self, val: T) -> bool {
        if self.len >= N { return false; }
        self.data[self.len] = val;
        self.len += 1;
        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        Some(self.data[self.len])
    }

    fn get(&self, i: usize) -> Option<&T> {
        if i < self.len { Some(&self.data[i]) } else { None }
    }

    fn len(&self) -> usize { self.len }
    fn is_full(&self) -> bool { self.len == N }
    fn capacity(&self) -> usize { N }

    fn as_slice(&self) -> &[T] { &self.data[..self.len] }
}

impl<T: Copy + Default + fmt::Display, const N: usize> fmt::Display for FixedVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FixedVec<{}>", N)?;
        write!(f, "[")?;
        for i in 0..self.len {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", self.data[i])?;
        }
        write!(f, "]")
    }
}

// Compile-time matrix with dimensions in the type
#[derive(Debug, Clone)]
struct Matrix<T, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

impl<T: Copy + Default + fmt::Display, const R: usize, const C: usize> Matrix<T, R, C> {
    fn new() -> Self { Matrix { data: [[T::default(); C]; R] } }

    fn set(&mut self, row: usize, col: usize, val: T) {
        self.data[row][col] = val;
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    fn rows(&self) -> usize { R }
    fn cols(&self) -> usize { C }

    fn print(&self) {
        for row in &self.data {
            for (i, v) in row.iter().enumerate() {
                if i > 0 { print!(", "); }
                print!("{:6.2}", v);
            }
            println!();
        }
    }
}

// Matrix multiplication — type system ensures dimensions match at compile time
// A is R×K, B is K×C, result is R×C
impl<const R: usize, const K: usize, const C: usize> Matrix<f64, R, K> {
    fn multiply(&self, other: &Matrix<f64, K, C>) -> Matrix<f64, R, C> {
        let mut result = Matrix::new();
        for r in 0..R {
            for c in 0..C {
                let mut sum = 0.0;
                for k in 0..K { sum += self.data[r][k] * other.data[k][c]; }
                result.data[r][c] = sum;
            }
        }
        result
    }
}

// Const generic function — works with any array size
fn sum_array<T, const N: usize>(arr: &[T; N]) -> T
where
    T: Default + Add<Output = T> + Copy,
{
    arr.iter().copied().fold(T::default(), |acc, x| acc + x)
}

fn assert_same_length<const N: usize>(a: &[u8; N], b: &[u8; N]) -> bool {
    a == b  // can only be called with same-sized arrays!
}

// ─── Generic Associated Types (GATs) ─────────────────────────────────────────

// A GAT-based lending iterator (borrow from self)
trait LendingIterator {
    // Item has a lifetime tied to &'this self — this is the GAT
    type Item<'this> where Self: 'this;
    fn next<'this>(&'this mut self) -> Option<Self::Item<'this>>;
}

// Lending iterator over a slice — yields references to original data
struct SliceIter<'a, T> {
    slice: &'a [T],
    pos:   usize,
}

impl<'a, T> SliceIter<'a, T> {
    fn new(slice: &'a [T]) -> Self { SliceIter { slice, pos: 0 } }
}

impl<'a, T> LendingIterator for SliceIter<'a, T> {
    type Item<'this> = &'this T where Self: 'this;

    fn next<'this>(&'this mut self) -> Option<Self::Item<'this>> {
        if self.pos >= self.slice.len() { return None; }
        let item = &self.slice[self.pos];
        self.pos += 1;
        Some(item)
    }
}

// GAT-based container trait
trait Container {
    type Item;
    type Iter<'a>: Iterator<Item = &'a Self::Item> where Self: 'a;

    fn iter<'a>(&'a self) -> Self::Iter<'a>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
}

struct VecContainer<T>(Vec<T>);

impl<T> Container for VecContainer<T> {
    type Item = T;
    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a;

    fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> { self.0.iter() }
    fn len(&self) -> usize { self.0.len() }
}

// ─── Sealed Trait Pattern ─────────────────────────────────────────────────────

// Prevents external crates from implementing the trait
mod sealed {
    pub trait Sealed {}
}

trait Numeric: sealed::Sealed + Copy + fmt::Display {
    fn to_f64(self) -> f64;
    fn from_f64(v: f64) -> Self;
}

impl sealed::Sealed for i32 {}
impl sealed::Sealed for f64 {}
impl sealed::Sealed for f32 {}

impl Numeric for i32 { fn to_f64(self) -> f64 { self as f64 } fn from_f64(v: f64) -> Self { v as i32 } }
impl Numeric for f64 { fn to_f64(self) -> f64 { self }         fn from_f64(v: f64) -> Self { v } }
impl Numeric for f32 { fn to_f64(self) -> f64 { self as f64 } fn from_f64(v: f64) -> Self { v as f32 } }

fn normalize<T: Numeric>(values: &[T]) -> Vec<T> {
    if values.is_empty() { return vec![]; }
    let min = values.iter().map(|v| v.to_f64()).fold(f64::INFINITY, f64::min);
    let max = values.iter().map(|v| v.to_f64()).fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    if range == 0.0 { return values.to_vec(); }
    values.iter().map(|&v| T::from_f64((v.to_f64() - min) / range)).collect()
}

// ─── Zero-Cost Builder with Type-Level Completion ────────────────────────────

// Compile-time "all fields set" check via phantom types
struct Set<T>(PhantomData<T>);
struct Unset<T>(PhantomData<T>);

struct Name;
struct Age;
struct Email;

struct UserBuilder<N, A, E> {
    name:  Option<String>,
    age:   Option<u32>,
    email: Option<String>,
    _n: PhantomData<N>,
    _a: PhantomData<A>,
    _e: PhantomData<E>,
}

impl UserBuilder<Unset<Name>, Unset<Age>, Unset<Email>> {
    fn new() -> Self {
        UserBuilder { name: None, age: None, email: None,
            _n: PhantomData, _a: PhantomData, _e: PhantomData }
    }
}

impl<A, E> UserBuilder<Unset<Name>, A, E> {
    fn name(self, name: &str) -> UserBuilder<Set<Name>, A, E> {
        UserBuilder { name: Some(name.to_string()), age: self.age, email: self.email,
            _n: PhantomData, _a: PhantomData, _e: PhantomData }
    }
}

impl<N, E> UserBuilder<N, Unset<Age>, E> {
    fn age(self, age: u32) -> UserBuilder<N, Set<Age>, E> {
        UserBuilder { name: self.name, age: Some(age), email: self.email,
            _n: PhantomData, _a: PhantomData, _e: PhantomData }
    }
}

impl<N, A> UserBuilder<N, A, Unset<Email>> {
    fn email(self, email: &str) -> UserBuilder<N, A, Set<Email>> {
        UserBuilder { name: self.name, age: self.age, email: Some(email.to_string()),
            _n: PhantomData, _a: PhantomData, _e: PhantomData }
    }
}

// build() ONLY available when all three are Set — compile error otherwise!
impl UserBuilder<Set<Name>, Set<Age>, Set<Email>> {
    fn build(self) -> User {
        User { name: self.name.unwrap(), age: self.age.unwrap(), email: self.email.unwrap() }
    }
}

#[derive(Debug)]
struct User { name: String, age: u32, email: String }

// ─── Typenum-style Compile-Time Integers ─────────────────────────────────────

// Type-level peano arithmetic
struct Zero;
struct Succ<N>(PhantomData<N>);

type One   = Succ<Zero>;
type Two   = Succ<One>;
type Three = Succ<Two>;
type Four  = Succ<Three>;
type Five  = Succ<Four>;

trait ToUsize { const VALUE: usize; }
impl ToUsize for Zero        { const VALUE: usize = 0; }
impl<N: ToUsize> ToUsize for Succ<N> { const VALUE: usize = N::VALUE + 1; }

// A strongly typed index that can't exceed N
struct BoundedIndex<N: ToUsize> {
    value: usize,
    _n: PhantomData<N>,
}

impl<N: ToUsize> BoundedIndex<N> {
    fn new(value: usize) -> Option<Self> {
        if value < N::VALUE {
            Some(BoundedIndex { value, _n: PhantomData })
        } else {
            None
        }
    }

    fn get(&self) -> usize { self.value }
}

// ─── Type-Level State Machine ─────────────────────────────────────────────────

// States
struct Disconnected;
struct Connecting;
struct Connected;
struct Authenticated;

struct Socket<State> {
    address: String,
    _state: PhantomData<State>,
}

impl Socket<Disconnected> {
    fn new(address: &str) -> Self {
        println!("  Socket created for {}", address);
        Socket { address: address.to_string(), _state: PhantomData }
    }

    fn connect(self) -> Socket<Connecting> {
        println!("  Connecting to {}...", self.address);
        Socket { address: self.address, _state: PhantomData }
    }
}

impl Socket<Connecting> {
    fn established(self) -> Socket<Connected> {
        println!("  Connection established");
        Socket { address: self.address, _state: PhantomData }
    }

    fn failed(self) -> Socket<Disconnected> {
        println!("  Connection failed, back to disconnected");
        Socket { address: self.address, _state: PhantomData }
    }
}

impl Socket<Connected> {
    fn authenticate(self, token: &str) -> Socket<Authenticated> {
        println!("  Authenticating with token: {}...", &token[..6.min(token.len())]);
        Socket { address: self.address, _state: PhantomData }
    }

    fn disconnect(self) -> Socket<Disconnected> {
        println!("  Disconnecting");
        Socket { address: self.address, _state: PhantomData }
    }
}

impl Socket<Authenticated> {
    fn send(&self, data: &str) {
        println!("  Sending '{}' to {}", data, self.address);
    }

    fn recv(&self) -> String {
        println!("  Receiving from {}", self.address);
        "ack".to_string()
    }

    fn logout(self) -> Socket<Connected> {
        println!("  Logged out");
        Socket { address: self.address, _state: PhantomData }
    }
}

// ─── Heterogeneous List (HList) via Recursion ─────────────────────────────────

struct HNil;
struct HCons<H, T>(H, T);

trait HList {
    fn len(&self) -> usize;
}

impl HList for HNil {
    fn len(&self) -> usize { 0 }
}

impl<H, T: HList> HList for HCons<H, T> {
    fn len(&self) -> usize { 1 + self.1.len() }
}

trait HHead {
    type Head;
    fn head(&self) -> &Self::Head;
}

impl<H, T> HHead for HCons<H, T> {
    type Head = H;
    fn head(&self) -> &H { &self.0 }
}

macro_rules! hlist {
    () => { HNil };
    ($head:expr $(, $tail:expr)*) => {
        HCons($head, hlist!($($tail),*))
    };
}

// ─── Const Evaluations ───────────────────────────────────────────────────────

const fn fibonacci_const(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            let mut i = 2;
            while i <= n {
                let tmp = a + b;
                a = b;
                b = tmp;
                i += 1;
            }
            b
        }
    }
}

const FIB_20: u64 = fibonacci_const(20);
const FIB_30: u64 = fibonacci_const(30);

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

// Compile-time reduced fraction
struct Fraction<const N: u64, const D: u64>;

impl<const N: u64, const D: u64> Fraction<N, D> {
    const GCD: u64 = gcd(N, D);
    const NUMERATOR: u64 = N / Self::GCD;
    const DENOMINATOR: u64 = D / Self::GCD;

    fn value() -> f64 { N as f64 / D as f64 }
    fn reduced() -> String { format!("{}/{}", Self::NUMERATOR, Self::DENOMINATOR) }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Type System Wizardry ===\n");

    // Const generics: FixedVec
    println!("── FixedVec<i32, 4> ──");
    let mut fv = FixedVec::<i32, 4>::new();
    fv.push(10); fv.push(20); fv.push(30);
    println!("{}", fv);
    println!("len={}, cap={}", fv.len(), fv.capacity());
    let overflow = fv.push(40);
    let would_fail = fv.push(50);
    println!("4th push: {}, 5th push (overflow): {}", overflow, would_fail);

    // Const generic matrix multiply
    println!("\n── Matrix<f64, 2, 3> × Matrix<f64, 3, 2> ──");
    let mut a: Matrix<f64, 2, 3> = Matrix::new();
    a.set(0,0,1.0); a.set(0,1,2.0); a.set(0,2,3.0);
    a.set(1,0,4.0); a.set(1,1,5.0); a.set(1,2,6.0);
    let mut b: Matrix<f64, 3, 2> = Matrix::new();
    b.set(0,0,7.0); b.set(0,1,8.0);
    b.set(1,0,9.0); b.set(1,1,10.0);
    b.set(2,0,11.0); b.set(2,1,12.0);
    println!("A:"); a.print();
    println!("B:"); b.print();
    let c = a.multiply(&b);
    println!("A×B:"); c.print();

    // sum_array works with any size
    println!("\n── sum_array (const generic) ──");
    println!("sum [1,2,3,4,5]: {}", sum_array(&[1,2,3,4,5]));
    println!("sum [10,20,30]:  {}", sum_array(&[10,20,30]));

    // GAT Container
    println!("\n── GAT Container ──");
    let c = VecContainer(vec![10, 20, 30, 40]);
    let sum: i32 = c.iter().sum();
    println!("sum via GAT iterator: {}", sum);

    // Sealed + normalize
    println!("\n── Sealed trait + normalize ──");
    let floats = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
    println!("normalized: {:?}", normalize(&floats));
    let ints = vec![0i32, 50, 100];
    println!("normalized: {:?}", normalize(&ints));

    // Type-safe builder
    println!("\n── Zero-Cost Type-State Builder ──");
    let user = UserBuilder::new()
        .name("Alice")
        .age(30)
        .email("alice@example.com")
        .build();  // only compiles if all fields are Set
    println!("{:?}", user);

    // Type-level integers
    println!("\n── Type-Level Integers ──");
    println!("Zero::VALUE    = {}", Zero::VALUE);
    println!("Three::VALUE   = {}", Three::VALUE);
    println!("Five::VALUE    = {}", Five::VALUE);
    let idx = BoundedIndex::<Five>::new(3);
    let bad = BoundedIndex::<Five>::new(10);
    println!("BoundedIndex(3) in [0..5): {:?}", idx.map(|i| i.get()));
    println!("BoundedIndex(10) in [0..5): {}", bad.is_some());

    // Type-level state machine
    println!("\n── Type-Level Socket ──");
    let socket = Socket::<Disconnected>::new("api.example.com:443");
    let connecting = socket.connect();
    let connected  = connecting.established();
    let authed     = connected.authenticate("Bearer secret_token_here");
    authed.send("{ \"query\": \"hello\" }");
    let resp = authed.recv();
    println!("  response: {}", resp);
    // can't call authed.send after this — it's moved
    // authed.logout().disconnect();  // compiles fine

    // HList
    println!("\n── HList ──");
    let list = hlist!(42, "hello", 3.14, true);
    println!("HList length: {}", list.len());
    println!("HList head: {}", list.head());

    // Const evaluation
    println!("\n── Const Evaluation ──");
    println!("fib(20) at compile time: {}", FIB_20);
    println!("fib(30) at compile time: {}", FIB_30);
    println!("gcd(48, 18) = {}", gcd(48, 18));

    type Half    = Fraction<1, 2>;
    type Quarter = Fraction<1, 4>;
    type SixTwelths = Fraction<6, 12>;
    println!("1/2     = {:.4}, reduced: {}", Half::value(), Half::reduced());
    println!("6/12    = {:.4}, reduced: {}", SixTwelths::value(), SixTwelths::reduced());
    println!("1/4     = {:.4}, reduced: {}", Quarter::value(), Quarter::reduced());

    println!("\n=== Done ===");
}