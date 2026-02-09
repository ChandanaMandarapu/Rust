// Traits, Generics, and Associated Types

use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Mul, Sub};

/// A trait defining drawable behavior
/// Traits are like interfaces in other languages
trait Drawable {
    /// Returns the name of the drawable object
    fn name(&self) -> &str;
    
    /// Draws the object to the screen
    fn draw(&self);
    
    /// Returns the area of the drawable object
    fn area(&self) -> f64;
    
    /// Default implementation for describing the object
    fn describe(&self) {
        println!("{} with area {:.2}", self.name(), self.area());
    }
}

/// A circle structure implementing Drawable
struct Circle {
    radius: f64,
    label: String,
}

impl Circle {
    /// Creates a new Circle
    fn new(radius: f64, label: String) -> Self {
        Circle { radius, label }
    }
}

impl Drawable for Circle {
    fn name(&self) -> &str {
        &self.label
    }

    fn draw(&self) {
        println!("Drawing circle '{}' with radius {}", self.label, self.radius);
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// A rectangle structure implementing Drawable
struct Rectangle {
    width: f64,
    height: f64,
    label: String,
}

impl Rectangle {
    /// Creates a new Rectangle
    fn new(width: f64, height: f64, label: String) -> Self {
        Rectangle { width, height, label }
    }
}

impl Drawable for Rectangle {
    fn name(&self) -> &str {
        &self.label
    }

    fn draw(&self) {
        println!("Drawing rectangle '{}' ({}x{})", self.label, self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// A generic container that works with any type
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    /// Creates a new empty container
    fn new() -> Self {
        Container { items: Vec::new() }
    }

    /// Adds an item to the container
    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    /// Returns the number of items
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the container is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns a reference to an item at index
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}

/// Additional methods for containers with Display types
impl<T: Display> Container<T> {
    /// Prints all items in the container
    fn print_all(&self) {
        for (i, item) in self.items.iter().enumerate() {
            println!("Item {}: {}", i, item);
        }
    }
}

/// A trait with associated types
trait Iterator {
    /// The type of items being iterated over
    type Item;

    /// Returns the next item
    fn next(&mut self) -> Option<Self::Item>;

    /// Returns the count of remaining items
    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while self.next().is_some() {
            count += 1;
        }
        count
    }
}

/// A counter implementing the Iterator trait
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    /// Creates a new counter
    fn new(max: u32) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

/// A trait for objects that can be summarized
trait Summarizable {
    /// Returns a summary of the object
    fn summary(&self) -> String;

    /// Returns the author or creator
    fn author(&self) -> String;

    /// Default implementation combining summary and author
    fn full_summary(&self) -> String {
        format!("{} (by {})", self.summary(), self.author())
    }
}

/// A blog post structure
struct BlogPost {
    title: String,
    content: String,
    author: String,
}

impl BlogPost {
    /// Creates a new blog post
    fn new(title: String, content: String, author: String) -> Self {
        BlogPost { title, content, author }
    }
}

impl Summarizable for BlogPost {
    fn summary(&self) -> String {
        format!("{}: {}", self.title, &self.content[..self.content.len().min(50)])
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

/// A tweet structure
struct Tweet {
    username: String,
    content: String,
    likes: u32,
}

impl Tweet {
    /// Creates a new tweet
    fn new(username: String, content: String) -> Self {
        Tweet {
            username,
            content,
            likes: 0,
        }
    }

    /// Increments the like count
    fn like(&mut self) {
        self.likes += 1;
    }
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }

    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

/// Function demonstrating trait bounds
fn print_summary<T: Summarizable>(item: &T) {
    println!("Summary: {}", item.full_summary());
}

/// Function with multiple trait bounds
fn complex_function<T>(item: &T)
where
    T: Summarizable + Display,
{
    println!("Display: {}", item);
    println!("Summary: {}", item.summary());
}

/// A generic pair structure
#[derive(Debug, Clone)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    /// Creates a new pair
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    /// Returns a reference to the first element
    fn first(&self) -> &T {
        &self.first
    }

    /// Returns a reference to the second element
    fn second(&self) -> &U {
        &self.second
    }

    /// Swaps the pair elements
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

/// Implementation for pairs where both types are the same
impl<T: PartialOrd> Pair<T, T> {
    /// Returns the larger of the two values
    fn max(&self) -> &T {
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

/// A trait for objects that can be converted to JSON
trait ToJson {
    /// Converts the object to a JSON string
    fn to_json(&self) -> String;
}

/// A user structure
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    /// Creates a new user
    fn new(id: u32, name: String, email: String) -> Self {
        User { id, name, email }
    }
}

impl ToJson for User {
    fn to_json(&self) -> String {
        format!(
            r#"{{"id": {}, "name": "{}", "email": "{}"}}"#,
            self.id, self.name, self.email
        )
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "User(id={}, name={})", self.id, self.name)
    }
}

/// A trait for cloneable objects
trait Cloneable {
    /// Creates a clone of the object
    fn clone_item(&self) -> Self;
}

/// A point structure
#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    /// Creates a new point
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

/// Implementation for points with numeric types
impl<T: Add<Output = T> + Copy> Point<T> {
    /// Adds two points
    fn add(&self, other: &Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy> Cloneable for Point<T> {
    fn clone_item(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

/// A trait for objects that can be compared
trait Comparable {
    /// Compares this object with another
    fn compare(&self, other: &Self) -> std::cmp::Ordering;
}

/// A person structure for comparison
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    /// Creates a new person
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

impl Comparable for Person {
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.age.cmp(&other.age)
    }
}

/// A generic function that works with comparable types
fn find_max<T: Comparable>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }

    let mut max = &items[0];
    for item in items.iter().skip(1) {
        if item.compare(max) == std::cmp::Ordering::Greater {
            max = item;
        }
    }
    Some(max)
}

/// A trait with default implementations
trait Animal {
    /// Returns the name of the animal
    fn name(&self) -> &str;

    /// Makes a sound (default implementation)
    fn make_sound(&self) {
        println!("{} makes a generic animal sound", self.name());
    }

    /// Returns the species
    fn species(&self) -> &str {
        "Unknown"
    }
}

/// A dog structure
struct Dog {
    name: String,
}

impl Dog {
    /// Creates a new dog
    fn new(name: String) -> Self {
        Dog { name }
    }
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("{} barks: Woof! Woof!", self.name);
    }

    fn species(&self) -> &str {
        "Canis familiaris"
    }
}

/// A cat structure
struct Cat {
    name: String,
}

impl Cat {
    /// Creates a new cat
    fn new(name: String) -> Self {
        Cat { name }
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("{} meows: Meow! Meow!", self.name);
    }

    fn species(&self) -> &str {
        "Felis catus"
    }
}

/// A trait for mathematical operations
trait Arithmetic<T = Self> {
    /// Adds two values
    fn add(&self, other: &T) -> Self;
    
    /// Subtracts two values
    fn subtract(&self, other: &T) -> Self;
    
    /// Multiplies two values
    fn multiply(&self, other: &T) -> Self;
}

/// A complex number structure
#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    /// Creates a new complex number
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    /// Returns the magnitude of the complex number
    fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

impl Arithmetic for Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    fn subtract(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }

    fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

/// A trait for serializable objects
trait Serializable {
    /// Serializes the object to a byte vector
    fn serialize(&self) -> Vec<u8>;
    
    /// Deserializes from a byte vector
    fn deserialize(data: &[u8]) -> Self where Self: Sized;
}

/// A matrix structure demonstrating advanced generics
#[derive(Debug)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone + Default> Matrix<T> {
    /// Creates a new matrix filled with default values
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    /// Gets a value at the specified position
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            self.data.get(row * self.cols + col)
        } else {
            None
        }
    }

    /// Sets a value at the specified position
    fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
            true
        } else {
            false
        }
    }
}

/// Implementation for numeric matrices
impl<T: Add<Output = T> + Clone + Default> Matrix<T> {
    /// Adds two matrices
    fn add_matrix(&self, other: &Matrix<T>) -> Option<Matrix<T>> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i].clone() + other.data[i].clone();
        }
        Some(result)
    }
}

/// Main function demonstrating all trait concepts
fn main() {
    println!("=== Rust Traits and Generics Demo ===\n");

    // Drawable trait demonstration
    let circle = Circle::new(5.0, String::from("Circle1"));
    let rectangle = Rectangle::new(10.0, 8.0, String::from("Rect1"));
    
    circle.draw();
    circle.describe();
    rectangle.draw();
    rectangle.describe();

    // Generic container demonstration
    let mut int_container = Container::new();
    int_container.add(10);
    int_container.add(20);
    int_container.add(30);
    println!("\nInteger container has {} items", int_container.len());
    int_container.print_all();

    // Counter demonstration
    let mut counter = Counter::new(5);
    println!("\nCounting:");
    while let Some(num) = counter.next() {
        println!("Count: {}", num);
    }

    // Summarizable trait demonstration
    let post = BlogPost::new(
        String::from("Rust Traits"),
        String::from("Traits are a powerful feature in Rust that enable polymorphism and code reuse."),
        String::from("Alice"),
    );
    print_summary(&post);

    let mut tweet = Tweet::new(String::from("rustlang"), String::from("Loving Rust traits!"));
    tweet.like();
    print_summary(&tweet);

    // Pair demonstration
    let pair = Pair::new(10, 20);
    println!("\nPair max: {}", pair.max());
    let swapped = pair.swap();
    println!("Swapped pair: {:?}", swapped);

    // User and JSON demonstration
    let user = User::new(1, String::from("Bob"), String::from("bob@example.com"));
    println!("\nUser JSON: {}", user.to_json());
    println!("User Display: {}", user);

    // Point demonstration
    let p1 = Point::new(3, 4);
    let p2 = Point::new(1, 2);
    let p3 = p1.add(&p2);
    println!("\nPoint addition: {:?}", p3);

    // Animal trait demonstration
    let dog = Dog::new(String::from("Buddy"));
    let cat = Cat::new(String::from("Whiskers"));
    println!();
    dog.make_sound();
    cat.make_sound();

    // Complex numbers demonstration
    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, 2.0);
    let c3 = c1.add(&c2);
    println!("\nComplex addition: {} + {} = {}", c1, c2, c3);
    println!("Magnitude of {}: {:.2}", c1, c1.magnitude());

    // Matrix demonstration
    let mut matrix = Matrix::new(2, 2);
    matrix.set(0, 0, 1);
    matrix.set(0, 1, 2);
    matrix.set(1, 0, 3);
    matrix.set(1, 1, 4);
    println!("\nMatrix: {:?}", matrix);

    println!("\n=== Demo Complete ===");
}