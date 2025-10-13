// ======================================================
// COMBINED: Advanced Lifetimes + Advanced Traits Examples
// All user-provided examples are included below (fixed to compile).
// I kept your comments and explanations exactly as you wrote them.
// ======================================================

use std::fmt;
use std::error::Error;
use std::ops::{Add, Sub, Mul, Neg};

// ---------------------------------------------------------------------------------
// Advanced Lifetime Concepts (Deep Knowledge)
// ---------------------------------------------------------------------------------

// Lifetime Subtyping (The Variance Concept)
// This is advanced, but important. Lifetimes have a subtyping relationship:
// If 'a lives longer than 'b, then 'a is a subtype of 'b. Written as 'a: 'b (read as "'a outlives 'b").
//
// rust
// fn example<'a, 'b>(x: &'a str) -> &'b str
// where
//     'a: 'b,  // 'a must outlive 'b
// {
//     x  // This is safe because 'a lives longer
// }
//
// Why this matters: You can use a longer-lived reference where a shorter-lived one is expected, but not vice versa.

fn lifetime_subtyping_example() {
    // function showing lifetime subtyping: 'a outlives 'b allows returning x as a shorter lifetime.
    fn example<'a, 'b>(x: &'a str) -> &'b str
    where
        'a: 'b,
    {
        x
    }

    // Use a 'static string so both lifetimes can be satisfied safely.
    let r: &'static str = example("hello static");
    println!("lifetime_subtyping_example -> {}", r);
}

// Lifetime Bounds on Types
// You can specify that a type must contain only references that live long enough:
//
// rust
// struct Ref<'a, T: 'a> {
//     reference: &'a T,
// }
//
// What 'T: 'a' means: "The type T must contain only references that live for at least 'a."
// This is often unnecessary because Rust assumes it, but you'll see it in complex generic code.

fn lifetime_bounds_on_types_example() {
    struct RefWrapper<'a, T: 'a> {
        reference: &'a T,
    }

    let value = 42;
    let wrapper = RefWrapper { reference: &value };
    println!("lifetime_bounds_on_types_example -> {}", wrapper.reference);
}

// Higher-Ranked Trait Bounds (HRTB)
// This is very advanced, but you'll see it in real code:
//
// rust
// fn apply<F>(f: F)
// where
//     F: for<'a> Fn(&'a str) -> &'a str,
// {
//     let s = String::from("hello");
//     println!("{}", f(&s));
// }
//
// What for<'a> means: "This function works for ANY lifetime 'a." This is needed when you're passing closures that work with references, and the lifetime isn't known at the time you define the function.

fn hrbt_example() {
    fn apply<F>(f: F)
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let s = String::from("hello");
        println!("hrbt_example -> {}", f(&s));
    }

    // A simple closure that works for any lifetime: returns the same borrowed str
    apply(|x| x);
}

// ---------------------------------------------------------------------------------
// HOUR 7-8: ADVANCED TRAITS - THE REAL POWER
// ---------------------------------------------------------------------------------

// Associated Types (The Clean Generic Alternative)
// We've seen traits with generic type parameters:
// rust
// trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
// But there's a better way when a trait should only have ONE type for a given implementation:
// rust
// trait Iterator {
//     type Item;  // Associated type
//     fn next(&mut self) -> Option<Self::Item>;
// }

// The difference:
// Generic type parameter: A type can implement the trait MULTIPLE TIMES with different types
// Associated type: A type can implement the trait ONLY ONCE

// Example of the problem with generics:
// rust
// trait Add<T> {
//     fn add(&self, other: T) -> T;
// }

// With associated types, this is prevented:
// rust
// trait Add {
//     type Output;
//     fn add(self, other: Self) -> Self::Output;
// }

// Real-world example - Iterator:

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // This Counter produces u32s

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn associated_types_example() {
    let mut counter = Counter::new();
    print_items(&mut counter);
}

// Using associated types in functions:
// rust
// fn print_items<I>(mut iterator: I)
// where
//     I: Iterator,
//     I::Item: std::fmt::Display,  // Access the associated type
// {
//     while let Some(item) = iterator.next() {
//         println!("{}", item);
//     }
// }
// The I::Item syntax accesses the associated type. This is like saying "whatever type this Iterator produces."

fn print_items<I>(iterator: &mut I)
where
    I: Iterator,
    I::Item: std::fmt::Display,
{
    while let Some(item) = iterator.next() {
        println!("associated_types_example -> {}", item);
    }
}

// Operator Overloading (Making Your Types Feel Native)
// Rust lets you overload operators like +, -, *, etc. by implementing special traits.
// The Add Trait (Overloading +)

// We will implement a Point struct for operator overloading examples.

#[derive(Debug, Clone, Copy)]
struct PointOp {
    x: i32,
    y: i32,
}

impl Add for PointOp {
    type Output = PointOp; // Adding two Points gives a Point

    fn add(self, other: Self) -> Self::Output {
        PointOp {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for PointOp {
    type Output = PointOp;
    fn sub(self, other: Self) -> Self::Output {
        PointOp {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for PointOp {
    type Output = PointOp;
    fn mul(self, scalar: i32) -> Self::Output {
        PointOp {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Neg for PointOp {
    type Output = PointOp;
    fn neg(self) -> Self::Output {
        PointOp { x: -self.x, y: -self.y }
    }
}

fn operator_overloading_example() {
    let p1 = PointOp { x: 1, y: 2 };
    let p2 = PointOp { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("operator_overloading_example -> {:?}", p3); // PointOp { x: 4, y: 6 }

    let p = PointOp { x: 1, y: 2 };
    let result = p + PointOp { x: 10, y: 10 };
    println!("operator_overloading_example -> {:?}", result);

    let p = PointOp { x: 3, y: 4 };
    let result1 = p * 2; // Mul
    println!("operator_overloading_example -> {:?}", result1);
    let result2 = -PointOp { x: 3, y: 4 }; // Neg
    println!("operator_overloading_example -> {:?}", result2);
}

// Full list of overloadable operators referenced in comments (no code needed):
// Arithmetic: Add, Sub, Mul, Div, Rem (remainder/modulo)
// Bitwise: BitAnd, BitOr, BitXor, Shl (shift left), Shr (shift right)
// Comparison: PartialEq, PartialOrd, Eq, Ord
// Unary: Neg, Not
// Indexing: Index, IndexMut
// Function call: Fn, FnMut, FnOnce

// The From and Into Traits (Conversion Magic)
// These traits enable type conversions. They're incredibly useful for writing flexible APIs.
#[derive(Debug, Clone, Copy)]
struct Celsius(f64);
#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

fn from_into_example() {
    let f = Fahrenheit(98.6);
    let c: Celsius = Celsius::from(f);
    println!("from_into_example -> {}°C", c.0);

    let f2 = Fahrenheit(212.0);
    let c2: Celsius = f2.into();
    println!("from_into_example -> {}°C", c2.0);
}

// Real-world pattern - String conversions:
struct Username(String);

impl From<String> for Username {
    fn from(s: String) -> Self {
        Username(s)
    }
}

impl From<&str> for Username {
    fn from(s: &str) -> Self {
        Username(s.to_string())
    }
}

fn from_into_usage_example() {
    let user1 = Username::from("Alice");
    let user2 = Username::from(String::from("Bob"));
    let user3: Username = "Charlie".into();

    println!("from_into_usage_example -> {}, {}, {}", user1.0, user2.0, user3.0);
}

// Using Into in function parameters:
fn greet(name: impl Into<String>) {
    let name_string = name.into();
    println!("greet -> Hello, {}!", name_string);
}

// Display and Debug Traits (Controlling How Things Print)
// Debug - For Programmers
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// Custom Debug implementation example
struct PointDebug {
    x: i32,
    y: i32,
}

impl fmt::Debug for PointDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PointDebug({}, {})", self.x, self.y)
    }
}

// Display - For End Users
struct Temperature {
    celsius: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.celsius)
    }
}

// A complete example combining both for a Book
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }
}

fn display_debug_examples() {
    let user = User { name: "Alice".into(), age: 30 };
    println!("display_debug_examples Debug: {:?}", user);

    let temp = Temperature { celsius: 23.5 };
    println!("display_debug_examples Display: {}", temp);

    let book = Book {
        title: "The Rust Book".into(),
        author: "Steve Klabnik".into(),
        pages: 500,
    };
    println!("display_debug_examples Debug: {:?}", book);
    println!("display_debug_examples Display: {}", book);
}

// Advanced Error Handling with Display
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
        }
    }
}

impl Error for MathError {}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn error_handling_example() {
    match divide(10.0, 2.0) {
        Ok(v) => println!("error_handling_example -> 10/2 = {}", v),
        Err(e) => println!("error_handling_example -> Error: {}", e),
    }

    match divide(1.0, 0.0) {
        Ok(v) => println!("error_handling_example -> {}", v),
        Err(e) => println!("error_handling_example -> Error: {}", e),
    }
}

// Practice: Implementing Traits for Custom Types (Vector2D example)
// Let's build a complete example that uses everything we've learned:

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

// Display trait - user-friendly output
impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// From trait - converting from tuple
impl From<(f64, f64)> for Vector2D {
    fn from(tuple: (f64, f64)) -> Self {
        Vector2D { x: tuple.0, y: tuple.1 }
    }
}

// Add trait - vector addition
impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Sub trait - vector subtraction
impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Mul trait - scalar multiplication
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Custom trait - vector operations
trait VectorOps {
    fn magnitude(&self) -> f64;
    fn normalize(&self) -> Vector2D;
    fn dot(&self, other: &Self) -> f64;
}

impl VectorOps for Vector2D {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Vector2D {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector2D { x: 0.0, y: 0.0 }
        } else {
            Vector2D {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

fn vector2d_example() {
    // Using From
    let v1 = Vector2D::from((3.0, 4.0));
    let v2: Vector2D = (1.0, 2.0).into();

    // Using operator overloading
    let v3 = v1 + v2;
    let v4 = v1 - v2;
    let v5 = v1 * 2.0;

    // Using Display
    println!("vector2d_example v1 = {}", v1);
    println!("vector2d_example v2 = {}", v2);
    println!("vector2d_example v3 = {}", v3);

    // Using Debug
    println!("vector2d_example v4 = {:?}", v4);

    // Using custom trait
    println!("vector2d_example magnitude of v1: {:.2}", v1.magnitude());
    let normalized = v1.normalize();
    println!("vector2d_example Normalized v1: {}", normalized);
    println!("vector2d_example Dot product: {:.2}", v1.dot(&v2));
}

// ---------------------------------------------------------------------------------
// All-in-one main that runs each example and prints clear labels
// ---------------------------------------------------------------------------------

fn main() {
    println!("=== Advanced Lifetimes + Advanced Traits Master File ===\n");

    // Lifetimes
    lifetime_subtyping_example();
    lifetime_bounds_on_types_example();
    hrbt_example();

    println!("\n--- Associated Types example ---");
    associated_types_example();

    println!("\n--- Operator Overloading example ---");
    operator_overloading_example();

    println!("\n--- From / Into example ---");
    from_into_example();
    from_into_usage_example();
    greet("World");
    greet(String::from("Rust"));

    println!("\n--- Display / Debug example ---");
    display_debug_examples();

    println!("\n--- Error Handling example ---");
    error_handling_example();

    println!("\n--- Vector2D full example ---");
    vector2d_example();

    println!("\n=== All examples finished ===");
}
