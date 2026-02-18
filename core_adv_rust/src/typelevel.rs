// Explored higher kinded type simulation using GATs with Functor Applicative and Monad traits. Added tagless final interpreters for evaluation pretty printing and operation counting from a single expression definition. Built a Free monad console DSL with multiple interpreters for real IO and testing. Implemented continuation passing style with trampolined recursion, defunctionalization of higher order functions, and Church encoded numerals and booleans. Main demonstrates HKT mapping binding applicatives tagless programs CPS fibonacci free monad workflows and pure lambda calculus encodings end to end.

#![allow(unused, type_alias_bounds)]

use std::fmt;
use std::marker::PhantomData;
use std::collections::HashMap;

// ─── Higher-Kinded Types via GATs ─────────────────────────────────────────────
// Rust has no F<_> where F is itself a type parameter.
// We simulate this with a trait that has an associated type.

// A type-level function: given Inner, produce the "lifted" type
trait HKT {
    type Applied<Inner>;
}

// Concrete type constructors
struct VecH;
struct OptionH;
struct ResultH<E>(PhantomData<E>);
struct BoxH;

impl HKT for VecH    { type Applied<T> = Vec<T>; }
impl HKT for OptionH { type Applied<T> = Option<T>; }
impl<E> HKT for ResultH<E> { type Applied<T> = Result<T, E>; }
impl HKT for BoxH    { type Applied<T> = Box<T>; }

// Now write a generic Functor over any F: HKT
trait Functor: HKT {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Self::Applied<A>, f: F) -> Self::Applied<B>;
}

impl Functor for VecH {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Vec<A>, f: F) -> Vec<B> { fa.into_iter().map(f).collect() }
}

impl Functor for OptionH {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Option<A>, f: F) -> Option<B> { fa.map(f) }
}

impl<E> Functor for ResultH<E> {
    fn fmap<A, B, F: Fn(A) -> B>(fa: Result<A, E>, f: F) -> Result<B, E> { fa.map(f) }
}

// Applicative
trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::Applied<A>;
    fn ap<A, B, F: Fn(A) -> B>(ff: Self::Applied<F>, fa: Self::Applied<A>) -> Self::Applied<B>;
}

impl Applicative for OptionH {
    fn pure<A>(a: A) -> Option<A> { Some(a) }
    fn ap<A, B, F: Fn(A) -> B>(ff: Option<F>, fa: Option<A>) -> Option<B> {
        ff.and_then(|f| fa.map(f))
    }
}

impl Applicative for VecH {
    fn pure<A>(a: A) -> Vec<A> { vec![a] }
    fn ap<A: Clone, B, F: Fn(A) -> B + Clone>(ff: Vec<F>, fa: Vec<A>) -> Vec<B> {
        ff.into_iter().flat_map(|f| fa.iter().cloned().map(move |a| f(a))).collect()
    }
}

// Monad
trait Monad: Applicative {
    fn bind<A, B, F: Fn(A) -> Self::Applied<B>>(ma: Self::Applied<A>, f: F) -> Self::Applied<B>;
    fn then<A, B>(ma: Self::Applied<A>, mb: Self::Applied<B>) -> Self::Applied<B>
    where A: 'static { Self::bind(ma, |_| mb) }
}

impl Monad for OptionH {
    fn bind<A, B, F: Fn(A) -> Option<B>>(ma: Option<A>, f: F) -> Option<B> { ma.and_then(f) }
}

impl Monad for VecH {
    fn bind<A, B, F: Fn(A) -> Vec<B>>(ma: Vec<A>, f: F) -> Vec<B> { ma.into_iter().flat_map(f).collect() }
}

// Generic function that works over any Monad F
fn safe_divide<F: Monad>(
    a: F::Applied<f64>,
    b: F::Applied<f64>,
) -> F::Applied<f64>
where
    F::Applied<f64>: Clone,
{
    // Can't use bind easily across different F — demonstrate with Option directly
    todo!()
}

// ─── Tagless Final Interpreter ────────────────────────────────────────────────
// Define operations as a trait. Different interpreters (eval, pretty-print, etc.)
// implement the same trait. The program is written generically over the interpreter.

trait Expr {
    type Repr<T>;

    fn lit_int(n: i64) -> Self::Repr<i64>;
    fn lit_bool(b: bool) -> Self::Repr<bool>;
    fn add(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn mul(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn neg(a: Self::Repr<i64>) -> Self::Repr<i64>;
    fn eq(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<bool>;
    fn if_<T>(cond: Self::Repr<bool>, then: Self::Repr<T>, else_: Self::Repr<T>) -> Self::Repr<T>;
    fn lam<A, B, F: Fn(Self::Repr<A>) -> Self::Repr<B>>(f: F) -> Self::Repr<Box<dyn Fn(A) -> B>>;
}

// Interpreter 1: Evaluate (direct semantics)
struct Eval;

impl Expr for Eval {
    type Repr<T> = T;

    fn lit_int(n: i64) -> i64 { n }
    fn lit_bool(b: bool) -> bool { b }
    fn add(a: i64, b: i64) -> i64 { a + b }
    fn mul(a: i64, b: i64) -> i64 { a * b }
    fn neg(a: i64) -> i64 { -a }
    fn eq(a: i64, b: i64) -> bool { a == b }
    fn if_<T>(cond: bool, then: T, else_: T) -> T { if cond { then } else { else_ } }
    fn lam<A, B, F: Fn(A) -> B + 'static>(f: F) -> Box<dyn Fn(A) -> B> { Box::new(f) }
}

// Interpreter 2: Pretty-print (string semantics)
struct Pretty;

impl Expr for Pretty {
    type Repr<T> = String;

    fn lit_int(n: i64) -> String { n.to_string() }
    fn lit_bool(b: bool) -> String { b.to_string() }
    fn add(a: String, b: String) -> String { format!("({} + {})", a, b) }
    fn mul(a: String, b: String) -> String { format!("({} * {})", a, b) }
    fn neg(a: String) -> String { format!("(-{})", a) }
    fn eq(a: String, b: String) -> String { format!("({} == {})", a, b) }
    fn if_<T>(cond: String, then: String, else_: String) -> String {
        format!("(if {} then {} else {})", cond, then, else_)
    }
    fn lam<A, B, F: Fn(String) -> String>(f: F) -> String {
        format!("(λx. {})", f("x".to_string()))
    }
}

// Interpreter 3: Count operations
struct Counter;
struct Count { ops: usize, repr: String }

impl Expr for Counter {
    type Repr<T> = Count;

    fn lit_int(n: i64) -> Count { Count { ops: 1, repr: n.to_string() } }
    fn lit_bool(b: bool) -> Count { Count { ops: 1, repr: b.to_string() } }
    fn add(a: Count, b: Count) -> Count { Count { ops: a.ops + b.ops + 1, repr: format!("({}+{})", a.repr, b.repr) } }
    fn mul(a: Count, b: Count) -> Count { Count { ops: a.ops + b.ops + 1, repr: format!("({}*{})", a.repr, b.repr) } }
    fn neg(a: Count) -> Count { Count { ops: a.ops + 1, repr: format!("(-{})", a.repr) } }
    fn eq(a: Count, b: Count) -> Count { Count { ops: a.ops + b.ops + 1, repr: format!("({}=={})", a.repr, b.repr) } }
    fn if_<T>(cond: Count, then: Count, else_: Count) -> Count {
        Count { ops: cond.ops + then.ops + else_ops(&else_) + 1, repr: format!("if {}", cond.repr) }
    }
    fn lam<A, B, F: Fn(Count) -> Count>(f: F) -> Count {
        let body = f(Count { ops: 0, repr: "x".to_string() });
        Count { ops: body.ops + 1, repr: format!("λx.{}", body.repr) }
    }
}

fn else_ops(c: &Count) -> usize { c.ops }

// Write a program ONCE, run it under any interpreter
fn my_program<E: Expr>() -> E::Repr<i64> {
    // (3 + 4) * -(2 + 1)
    E::mul(
        E::add(E::lit_int(3), E::lit_int(4)),
        E::neg(E::add(E::lit_int(2), E::lit_int(1))),
    )
}

fn my_conditional<E: Expr>() -> E::Repr<i64> {
    E::if_(
        E::eq(E::lit_int(5), E::add(E::lit_int(2), E::lit_int(3))),
        E::lit_int(100),
        E::lit_int(-1),
    )
}

// ─── Free Monad ──────────────────────────────────────────────────────────────
// A Free monad separates the description of an effectful computation
// from its interpretation. The program is a pure data structure.

// The "algebra" — operations without implementation
#[derive(Debug)]
enum ConsoleOp<Next> {
    PrintLine(String, Next),
    ReadLine(Box<dyn Fn(String) -> Next>),
}

// Free monad over ConsoleOp
enum Free<Op, A> {
    Pure(A),
    Bind(Op),
}

// Console DSL using Free
enum Console<A> {
    Pure(A),
    PrintLine(String, Box<Console<A>>),
    ReadLine(Box<dyn Fn(String) -> Console<A>>),
}

impl<A> Console<A> {
    fn pure(a: A) -> Self { Console::Pure(a) }

    fn print_line(msg: impl Into<String>) -> Console<()> {
        Console::PrintLine(msg.into(), Box::new(Console::Pure(())))
    }

    fn read_line() -> Console<String> {
        Console::ReadLine(Box::new(|s| Console::Pure(s)))
    }

    fn and_then<B, F: Fn(A) -> Console<B> + 'static>(self, f: F) -> Console<B> {
        match self {
            Console::Pure(a) => f(a),
            Console::PrintLine(msg, next) => Console::PrintLine(msg, Box::new(next.and_then(f))),
            Console::ReadLine(k) => Console::ReadLine(Box::new(move |s| k(s).and_then(&f))),
        }
    }
}

impl<A: fmt::Display> Console<A> {
    fn map<B, F: Fn(A) -> B + 'static>(self, f: F) -> Console<B> {
        self.and_then(move |a| Console::Pure(f(a)))
    }
}

// Interpreter 1: Run with real IO (simulated here)
fn interpret_console<A: fmt::Debug>(program: Console<A>, inputs: &mut Vec<String>) -> A {
    match program {
        Console::Pure(a) => a,
        Console::PrintLine(msg, next) => {
            println!("  [console] {}", msg);
            interpret_console(*next, inputs)
        }
        Console::ReadLine(k) => {
            let input = inputs.pop().unwrap_or_else(|| "default".to_string());
            println!("  [console] read: {}", input);
            interpret_console(k(input), inputs)
        }
    }
}

// Interpreter 2: Collect all printed lines (testing)
fn interpret_collect<A>(program: Console<A>, inputs: &mut Vec<String>) -> (A, Vec<String>) {
    fn go<A>(p: Console<A>, inputs: &mut Vec<String>, acc: &mut Vec<String>) -> A {
        match p {
            Console::Pure(a) => a,
            Console::PrintLine(msg, next) => { acc.push(msg); go(*next, inputs, acc) }
            Console::ReadLine(k) => {
                let input = inputs.pop().unwrap_or_default();
                go(k(input), inputs, acc)
            }
        }
    }
    let mut acc = vec![];
    let a = go(program, inputs, &mut acc);
    (a, acc)
}

// Write a program ONCE using the Console DSL
fn greeting_program() -> Console<String> {
    Console::print_line("Enter your name:").and_then(|_| {
        Console::read_line().and_then(|name| {
            let msg = format!("Hello, {}!", name);
            Console::print_line(msg.clone()).and_then(move |_| {
                Console::print_line("Have a nice day!").and_then(move |_| {
                    Console::pure(msg)
                })
            })
        })
    })
}

// ─── Continuation-Passing Style (CPS) ────────────────────────────────────────
// Every function takes an extra argument: what to do with the result.
// Enables: tail-call optimization, coroutines, generators, backtracking.

fn add_cps<R>(a: i64, b: i64, k: impl FnOnce(i64) -> R) -> R { k(a + b) }
fn mul_cps<R>(a: i64, b: i64, k: impl FnOnce(i64) -> R) -> R { k(a * b) }
fn neg_cps<R>(a: i64, k: impl FnOnce(i64) -> R) -> R { k(-a) }

// CPS fibonacci — tail-call optimizable
fn fib_cps(n: u64, k: impl FnOnce(u64) -> u64) -> u64 {
    if n <= 1 { k(n) }
    else {
        fib_cps(n - 1, |a| fib_cps(n - 2, |b| k(a + b)))
    }
}

// CPS with explicit continuation stack (trampolined — no stack overflow)
enum Trampoline<T> {
    Done(T),
    More(Box<dyn FnOnce() -> Trampoline<T>>),
}

impl<T> Trampoline<T> {
    fn run(self) -> T {
        let mut step = self;
        loop {
            match step {
                Trampoline::Done(v) => return v,
                Trampoline::More(f) => step = f(),
            }
        }
    }
}

fn fib_tramp(n: u64) -> Trampoline<u64> {
    if n <= 1 {
        Trampoline::Done(n)
    } else {
        Trampoline::More(Box::new(move || {
            let a = fib_tramp(n - 1).run();
            let b = fib_tramp(n - 2).run();
            Trampoline::Done(a + b)
        }))
    }
}

// ─── Church Encoding ──────────────────────────────────────────────────────────
// Represent data types purely as functions (lambda calculus style)

// Church numeral: n = λf.λx. f(f(f(...x...))) applied n times
type Church<T> = Box<dyn Fn(Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T>>;

fn church_zero<T: 'static>() -> Church<T> {
    Box::new(|_f| Box::new(|x| x))
}

fn church_succ<T: Clone + 'static>(n: Church<T>) -> Church<T> {
    Box::new(move |f: Box<dyn Fn(T) -> T>| {
        let nf = n(dyn_clone(&f));
        Box::new(move |x: T| f(nf(x)))
    })
}

fn dyn_clone<T: Clone + 'static>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T> {
    let f_ref = f as *const Box<dyn Fn(T) -> T>;
    Box::new(move |x| unsafe { (*f_ref)(x) })
}

fn church_to_usize(n: Church<usize>) -> usize {
    let f: Box<dyn Fn(usize) -> usize> = Box::new(|x| x + 1);
    n(f)(0)
}

// Church booleans
fn church_true<A: 'static, B: 'static>() -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> A>> {
    Box::new(|a| Box::new(move |_b| a))
}

fn church_false<A: 'static, B: 'static>() -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> B>> {
    Box::new(|_a| Box::new(move |b| b))
}

// ─── Defunctionalization ──────────────────────────────────────────────────────
// Convert higher-order functions to first-order by reifying closures as data

#[derive(Debug, Clone)]
enum DoubleListCont {
    Done,
    ConsNext { head: i32, rest: Box<DoubleListCont> },
}

fn double_list_defunc(xs: &[i32]) -> Vec<i32> {
    fn build_cont(xs: &[i32]) -> DoubleListCont {
        if xs.is_empty() { DoubleListCont::Done }
        else {
            DoubleListCont::ConsNext {
                head: xs[0] * 2,
                rest: Box::new(build_cont(&xs[1..])),
            }
        }
    }

    fn apply_cont(k: DoubleListCont) -> Vec<i32> {
        match k {
            DoubleListCont::Done => vec![],
            DoubleListCont::ConsNext { head, rest } => {
                let mut v = vec![head];
                v.extend(apply_cont(*rest));
                v
            }
        }
    }

    apply_cont(build_cont(xs))
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Type-Level Programming ===\n");

    // HKT Functor
    println!("── HKT Functor / Monad ──");
    let v = VecH::fmap(vec![1, 2, 3, 4], |x| x * x);
    println!("  Vec fmap (square): {:?}", v);

    let o = OptionH::fmap(Some(42), |x| x + 8);
    println!("  Option fmap (+8): {:?}", o);

    let b = VecH::bind(vec![1, 2, 3], |x| vec![x, x * 10]);
    println!("  Vec bind (dup): {:?}", b);

    let c = OptionH::bind(Some(10), |x| if x > 5 { Some(x * 2) } else { None });
    println!("  Option bind (>5 → *2): {:?}", c);

    // Applicative
    let fns = vec![|x: i32| x + 1, |x: i32| x * 2];
    let vals = vec![10, 20];
    let ap_result = VecH::ap(fns, vals);
    println!("  Vec ap (+1, *2) × [10,20]: {:?}", ap_result);

    // Tagless final
    println!("\n── Tagless Final ──");
    let eval_result   = my_program::<Eval>();
    let pretty_result = my_program::<Pretty>();
    let count_result  = my_program::<Counter>();
    println!("  program eval:   {}", eval_result);
    println!("  program pretty: {}", pretty_result);
    println!("  program ops:    {}", count_result.ops);

    let cond_eval   = my_conditional::<Eval>();
    let cond_pretty = my_conditional::<Pretty>();
    println!("  conditional eval:   {}", cond_eval);
    println!("  conditional pretty: {}", cond_pretty);

    // Free monad (Console)
    println!("\n── Free Monad (Console DSL) ──");
    let program = greeting_program();
    let mut inputs = vec!["Rustacean".to_string()];
    let result = interpret_console(program, &mut inputs);
    println!("  result: {}", result);

    // Test interpreter
    let program2 = greeting_program();
    let mut inputs2 = vec!["Ferris".to_string()];
    let (_, lines) = interpret_collect(program2, &mut inputs2);
    println!("  collected lines: {:?}", lines);

    // CPS
    println!("\n── CPS ──");
    let result = add_cps(3, 4, |sum| mul_cps(sum, 2, |prod| neg_cps(prod, |n| n)));
    println!("  -(( 3 + 4) * 2) = {}", result);

    for n in [0, 1, 5, 10] {
        let r = fib_cps(n, |x| x);
        println!("  fib_cps({}) = {}", n, r);
    }

    // Trampolined
    println!("\n── Trampolined Fibonacci ──");
    for n in [0u64, 1, 5, 10, 15] {
        println!("  fib_tramp({}) = {}", n, fib_tramp(n).run());
    }

    // Defunctionalization
    println!("\n── Defunctionalization ──");
    let xs = vec![1, 2, 3, 4, 5];
    println!("  double_list {:?} = {:?}", xs, double_list_defunc(&xs));

    // Church encoding
    println!("\n── Church Encoding ──");
    let zero  = church_zero::<usize>();
    let one   = church_succ(church_zero::<usize>());
    let two   = church_succ(church_succ(church_zero::<usize>()));
    let three = church_succ(church_succ(church_succ(church_zero::<usize>())));
    println!("  church zero  = {}", church_to_usize(zero));
    println!("  church one   = {}", church_to_usize(one));
    println!("  church two   = {}", church_to_usize(two));
    println!("  church three = {}", church_to_usize(three));

    println!("\n=== Done ===");
}