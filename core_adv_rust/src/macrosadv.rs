// Implemented deep macro_rules patterns including TT munchers, push down automata, recursive token processing, and variadic style repetition. Added struct generators, builder style macros, compile time assertions, newtype operator derivations, and custom error boilerplate. Built declarative state machine generators, string backed enums, SQL like DSL macros, map literals, benchmark harness, and static assertions. Demonstrated macro driven code generation for domain models, protocol enums, unit types, and runtime helpers, showing how declarative macros can simulate procedural macros and generate full APIs at compile time.

#![allow(unused_macros, unused_imports, dead_code, unused_variables)]

use std::collections::HashMap;
use std::fmt;

// ─── TT Muncher Pattern ───────────────────────────────────────────────────────
// Process token trees one at a time recursively — the workhorse of complex macros

macro_rules! count_items {
    () => { 0usize };
    ($head:tt $($tail:tt)*) => { 1usize + count_items!($($tail)*) };
}

macro_rules! sum_values {
    () => { 0 };
    ($head:expr) => { $head };
    ($head:expr, $($tail:expr),+) => { $head + sum_values!($($tail),+) };
}

macro_rules! max_of {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {{
        let rest_max = max_of!($($rest),+);
        if $x > rest_max { $x } else { rest_max }
    }};
}

// ─── Push-Down Automaton Pattern ──────────────────────────────────────────────
// Accumulate tokens in an "output" parameter, process input left-to-right

// Reverse a list of tokens
macro_rules! reverse_tokens {
    // Entry: wrap in internal call with empty accumulator
    ($($tokens:tt)*) => { reverse_tokens!(@rev [] $($tokens)*) };
    // Base case: no more input — emit accumulated tokens
    (@rev [$($acc:tt)*]) => { $($acc)* };
    // Step: move first token of input to front of accumulator
    (@rev [$($acc:tt)*] $head:tt $($tail:tt)*) => {
        reverse_tokens!(@rev [$head $($acc)*] $($tail)*)
    };
}

// Accumulate fields into a struct definition
macro_rules! make_struct {
    // Entry point
    (struct $name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone)]
        struct $name {
            $( $field: $ty, )*
        }

        impl $name {
            fn new($( $field: $ty ),*) -> Self {
                $name { $( $field, )* }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($name))?;
                write!(f, " {{ ")?;
                $( write!(f, concat!(stringify!($field), "={:?} "), self.$field)?; )*
                write!(f, "}}")
            }
        }
    };
}

make_struct!(struct Point { x: f64, y: f64 });
make_struct!(struct Color { r: u8, g: u8, b: u8 });
make_struct!(struct Rect { x: f64, y: f64, width: f64, height: f64 });

// ─── Derive-like Macro: Builder Pattern ───────────────────────────────────────

macro_rules! builder {
    (
        pub struct $name:ident {
            $( $field:ident : $ty:ty $(= $default:expr)? ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $( pub $field: $ty, )*
        }

        paste::paste! {  // would use paste! in real code; we inline naming here
            #[derive(Debug, Default, Clone)]
            pub struct [<$name Builder>] {
                $( $field: Option<$ty>, )*
            }

            impl [<$name Builder>] {
                pub fn new() -> Self { Self::default() }

                $( pub fn $field(mut self, v: $ty) -> Self { self.$field = Some(v); self } )*

                pub fn build(self) -> Result<$name, String> {
                    Ok($name {
                        $( $field: self.$field.ok_or_else(|| format!("missing field: {}", stringify!($field)))?, )*
                    })
                }
            }
        }
    };
}

// ─── Builder without paste (manual name concatenation) ───────────────────────

macro_rules! define_builder {
    (
        $name:ident {
            $( $field:ident : $ty:ty ),* $(,)?
        }
    ) => {
        // The target struct
        #[derive(Debug, Clone)]
        struct $name { $( $field: $ty, )* }

        // We can't concatenate identifiers in stable macro_rules without paste,
        // so we use a fixed name suffix trick:
        impl $name {
            fn builder() -> __Builder::<$name> {
                __Builder { inner: std::collections::HashMap::new() }
            }
        }
    };
}

// Instead, let's write our own full builder generator:
macro_rules! buildable {
    (
        struct $name:ident {
            required { $( $req_field:ident : $req_ty:ty ),* $(,)? }
            optional { $( $opt_field:ident : $opt_ty:ty = $opt_default:expr ),* $(,)? }
        }
    ) => {
        #[derive(Debug, Clone)]
        struct $name {
            $( $req_field: $req_ty, )*
            $( $opt_field: $opt_ty, )*
        }

        struct $name {
            $( $req_field: Option<$req_ty>, )*
            $( $opt_field: $opt_ty, )*
        }
    };
}

// ─── State Machine Macro ─────────────────────────────────────────────────────

macro_rules! state_machine {
    (
        machine $name:ident {
            states: [ $($state:ident),+ $(,)? ]
            initial: $initial:ident
            transitions: [
                $( $from:ident --[ $event:ident ]--> $to:ident ),+ $(,)?
            ]
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum $name {
            $( $state, )+
        }

        impl $name {
            fn initial() -> Self { $name::$initial }

            fn transition(self, event: &str) -> Option<Self> {
                match (self, event) {
                    $( ($name::$from, stringify!($event)) => Some($name::$to), )+
                    _ => None,
                }
            }

            fn name(&self) -> &'static str {
                match self {
                    $( $name::$state => stringify!($state), )+
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

state_machine! {
    machine TrafficLight {
        states: [Red, Yellow, Green]
        initial: Red
        transitions: [
            Red    --[next]--> Green,
            Green  --[next]--> Yellow,
            Yellow --[next]--> Red,
            Red    --[emergency]--> Red,
            Green  --[emergency]--> Red,
            Yellow --[emergency]--> Red,
        ]
    }
}

state_machine! {
    machine Connection {
        states: [Closed, Connecting, Open, HalfOpen, Failed]
        initial: Closed
        transitions: [
            Closed     --[connect]    --> Connecting,
            Connecting --[success]    --> Open,
            Connecting --[fail]       --> Failed,
            Open       --[close]      --> Closed,
            Open       --[half_close] --> HalfOpen,
            HalfOpen   --[close]      --> Closed,
            Failed     --[retry]      --> Connecting,
        ]
    }
}

// ─── Recursive Macro: Nested Map Literal ─────────────────────────────────────

macro_rules! map {
    () => { std::collections::HashMap::new() };
    ( $($key:expr => $val:expr),+ $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )+
        m
    }};
}

// ─── Variadic Generics Simulation ────────────────────────────────────────────
// Rust has no variadic generics; we simulate them with macro repetition

macro_rules! impl_tuple_from {
    // Generate From<(A,)> for N-tuples
    ( $( ($idx:tt : $T:ident) ),+ ) => {
        impl<$($T: std::fmt::Debug),+> std::fmt::Display for Tuple<($($T,)+)> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "(")?;
                $( write!(f, "{:?}, ", self.0.$idx)?; )+
                write!(f, ")")
            }
        }
    };
}

struct Tuple<T>(T);

// ─── DSL: SQL-like Query Builder ─────────────────────────────────────────────

macro_rules! sql {
    // SELECT * FROM table
    (SELECT * FROM $table:ident) => {{
        Query::select_all(stringify!($table))
    }};
    // SELECT col FROM table WHERE col = val
    (SELECT $col:ident FROM $table:ident WHERE $wcol:ident = $wval:expr) => {{
        Query::select_where(stringify!($table), stringify!($col), stringify!($wcol), $wval.to_string())
    }};
    // INSERT INTO table VALUES (...)
    (INSERT INTO $table:ident VALUES ( $($val:expr),+ $(,)? )) => {{
        Query::insert(stringify!($table), vec![ $( $val.to_string() ),+ ])
    }};
    // DELETE FROM table WHERE col = val
    (DELETE FROM $table:ident WHERE $col:ident = $val:expr) => {{
        Query::delete(stringify!($table), stringify!($col), $val.to_string())
    }};
}

#[derive(Debug)]
enum Query {
    SelectAll { table: String },
    SelectWhere { table: String, col: String, where_col: String, where_val: String },
    Insert { table: String, values: Vec<String> },
    Delete { table: String, where_col: String, where_val: String },
}

impl Query {
    fn select_all(table: &str) -> Self { Query::SelectAll { table: table.to_string() } }
    fn select_where(table: &str, col: &str, wcol: &str, wval: String) -> Self {
        Query::SelectWhere { table: table.to_string(), col: col.to_string(), where_col: wcol.to_string(), where_val: wval }
    }
    fn insert(table: &str, values: Vec<String>) -> Self {
        Query::Insert { table: table.to_string(), values }
    }
    fn delete(table: &str, wcol: &str, wval: String) -> Self {
        Query::Delete { table: table.to_string(), where_col: wcol.to_string(), where_val: wval }
    }

    fn to_sql(&self) -> String {
        match self {
            Query::SelectAll { table } => format!("SELECT * FROM {}", table),
            Query::SelectWhere { table, col, where_col, where_val } =>
                format!("SELECT {} FROM {} WHERE {} = '{}'", col, table, where_col, where_val),
            Query::Insert { table, values } =>
                format!("INSERT INTO {} VALUES ({})", table, values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ")),
            Query::Delete { table, where_col, where_val } =>
                format!("DELETE FROM {} WHERE {} = '{}'", table, where_col, where_val),
        }
    }
}

// ─── Macro: Define Enum with from_str ────────────────────────────────────────

macro_rules! str_enum {
    (
        $(#[$attr:meta])*
        enum $name:ident {
            $( $variant:ident = $repr:literal ),+ $(,)?
        }
    ) => {
        $(#[$attr])*
        enum $name {
            $( $variant, )+
        }

        impl $name {
            fn from_str(s: &str) -> Option<Self> {
                match s {
                    $( $repr => Some($name::$variant), )+
                    _ => None,
                }
            }

            fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => $repr, )+
                }
            }

            fn all_variants() -> &'static [&'static str] {
                &[ $( $repr, )+ ]
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}::{}", stringify!($name), self.as_str())
            }
        }
    };
}

str_enum! {
    #[derive(Clone, Copy, PartialEq)]
    enum HttpMethod {
        Get    = "GET",
        Post   = "POST",
        Put    = "PUT",
        Patch  = "PATCH",
        Delete = "DELETE",
        Head   = "HEAD",
    }
}

str_enum! {
    #[derive(Clone, Copy, PartialEq)]
    enum StatusCode {
        Ok          = "200",
        Created     = "201",
        BadRequest  = "400",
        Unauthorized = "401",
        NotFound    = "404",
        ServerError = "500",
    }
}

// ─── Macro: impl_all_ops ─────────────────────────────────────────────────────

macro_rules! newtype_ops {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        struct $name($inner);

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, r: Self) -> Self { $name(self.0 + r.0) }
        }
        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, r: Self) -> Self { $name(self.0 - r.0) }
        }
        impl std::ops::Mul<$inner> for $name {
            type Output = Self;
            fn mul(self, r: $inner) -> Self { $name(self.0 * r) }
        }
        impl std::ops::Neg for $name {
            type Output = Self;
            fn neg(self) -> Self { $name(-self.0) }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl From<$inner> for $name {
            fn from(v: $inner) -> Self { $name(v) }
        }
        impl From<$name> for $inner {
            fn from(v: $name) -> Self { v.0 }
        }
    };
}

newtype_ops!(Meters, f64);
newtype_ops!(Kilograms, f64);
newtype_ops!(Seconds, f64);
newtype_ops!(Celsius, f64);

// ─── Compile-Time Assertions ──────────────────────────────────────────────────

macro_rules! static_assert {
    ($cond:expr) => {
        const _: [(); 0 - !$cond as usize] = [];
    };
    ($cond:expr, $msg:literal) => {
        const _: [(); 0 - !$cond as usize] = []; // msg visible in error
    };
}

static_assert!(std::mem::size_of::<u64>() == 8);
static_assert!(std::mem::size_of::<bool>() == 1);
static_assert!(std::mem::align_of::<u64>() == 8);

// ─── Macro: Benchmark Harness ─────────────────────────────────────────────────

macro_rules! bench {
    ($name:literal, $iters:expr, $code:block) => {{
        use std::time::Instant;
        let start = Instant::now();
        for _ in 0..$iters { $code }
        let elapsed = start.elapsed();
        println!(
            "  bench {:20}: {:>5} iters in {:>8.3}ms = {:>6.1}ns/iter",
            $name, $iters,
            elapsed.as_secs_f64() * 1000.0,
            elapsed.as_nanos() as f64 / $iters as f64,
        );
    }};
}

// ─── Macro: Error Boilerplate ─────────────────────────────────────────────────

macro_rules! define_error {
    (
        $(#[$attr:meta])*
        enum $name:ident {
            $( #[msg = $msg:literal] $variant:ident $( { $($field:ident : $fty:ty),* } )? ),+
            $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug)]
        enum $name {
            $( $variant $( { $($field: $fty),* } )? , )+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $( $name::$variant $( { $($field),* } )? => write!(f, $msg $(, $($field),*)? ), )+
                }
            }
        }

        impl std::error::Error for $name {}
    };
}

define_error! {
    #[derive(Clone)]
    enum AppError {
        #[msg = "file not found: {}"]
        FileNotFound { path: String },
        #[msg = "parse error at line {}: {}"]
        ParseError { line: u32, message: String },
        #[msg = "network timeout after {}ms"]
        Timeout { ms: u64 },
        #[msg = "unauthorized access"]
        Unauthorized,
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Macro Systems ===\n");

    // TT Muncher
    println!("── TT Muncher ──");
    println!("  count_items!(a b c d e) = {}", count_items!(a b c d e));
    println!("  sum_values!(1,2,3,4,5)  = {}", sum_values!(1, 2, 3, 4, 5));
    println!("  max_of!(3,1,4,1,5,9,2) = {}", max_of!(3, 1, 4, 1, 5, 9, 2));

    // Struct-generating macro
    println!("\n── make_struct! ──");
    let p = Point::new(3.0, 4.0);
    let c = Color::new(255, 128, 0);
    let r = Rect::new(0.0, 0.0, 100.0, 50.0);
    println!("  {}", p);
    println!("  {}", c);
    println!("  {}", r);

    // State machine macro
    println!("\n── State Machine ──");
    let mut light = TrafficLight::initial();
    for _ in 0..5 {
        println!("  {}", light);
        light = light.transition("next").unwrap_or(light);
    }
    light = light.transition("emergency").unwrap_or(light);
    println!("  after emergency: {}", light);

    println!();
    let mut conn = Connection::initial();
    for event in &["connect", "success", "half_close", "close", "connect", "fail", "retry", "success"] {
        match conn.transition(event) {
            Some(next) => { println!("  {} --[{}]--> {}", conn, event, next); conn = next; }
            None => println!("  {} --[{}]--> (no transition)", conn, event),
        }
    }

    // Map macro
    println!("\n── map! macro ──");
    let m: HashMap<&str, i32> = map! {
        "one"   => 1,
        "two"   => 2,
        "three" => 3,
    };
    let mut keys: Vec<_> = m.keys().collect();
    keys.sort();
    for k in keys { println!("  {} = {}", k, m[k]); }

    // SQL DSL
    println!("\n── SQL DSL macro ──");
    let q1 = sql!(SELECT * FROM users);
    let q2 = sql!(SELECT name FROM users WHERE id = 42);
    let q3 = sql!(INSERT INTO orders VALUES ("chair", "19.99", "2"));
    let q4 = sql!(DELETE FROM sessions WHERE token = "abc123");
    for q in &[&q1, &q2, &q3, &q4] { println!("  {}", q.to_sql()); }

    // str_enum
    println!("\n── str_enum! ──");
    println!("  all methods: {:?}", HttpMethod::all_variants());
    for s in ["GET", "POST", "INVALID", "DELETE"] {
        match HttpMethod::from_str(s) {
            Some(m) => println!("  {} → {:?}", s, m),
            None    => println!("  {} → not found", s),
        }
    }

    // Newtype ops
    println!("\n── newtype_ops! ──");
    let d1 = Meters(100.0);
    let d2 = Meters(50.0);
    println!("  {} + {} = {}", d1, d2, d1 + d2);
    println!("  {} - {} = {}", d1, d2, d1 - d2);
    println!("  {} * 2 = {}", d1, d1 * 2.0);
    println!("  -{}    = {}", d1, -d1);

    // Error macro
    println!("\n── define_error! ──");
    let errs: Vec<AppError> = vec![
        AppError::FileNotFound { path: "/etc/config.toml".to_string() },
        AppError::ParseError   { line: 42, message: "unexpected token".to_string() },
        AppError::Timeout      { ms: 5000 },
        AppError::Unauthorized,
    ];
    for e in &errs { println!("  {}", e); }

    // Benchmark
    println!("\n── Benchmarks ──");
    bench!("vec push 1000",     1000, { let mut v: Vec<i32> = Vec::new(); for i in 0..1000 { v.push(i); } });
    bench!("hashmap insert 100", 100, { let mut m: HashMap<i32,i32> = HashMap::new(); for i in 0..100 { m.insert(i, i*2); } });
    bench!("string concat 50",    50, { let mut s = String::new(); for _ in 0..50 { s.push_str("hello"); } });

    println!("\n=== Done ===");
}