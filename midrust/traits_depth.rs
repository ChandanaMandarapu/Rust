// File 12: Traits In Depth — Real Patterns
// Default methods, trait objects, dynamic dispatch, impl Trait, blanket impls,
// operator overloading, and the standard library trait ecosystem

use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, Index, IndexMut};
use std::cmp::Ordering;
use std::collections::HashMap;

// ─── Operator Overloading ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self { Vec2 { x, y } }
    fn zero() -> Self { Vec2::new(0.0, 0.0) }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(&self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn normalized(&self) -> Vec2 {
        let len = self.length();
        if len == 0.0 { return Vec2::zero(); }
        Vec2::new(self.x / len, self.y / len)
    }

    fn distance_to(&self, other: Vec2) -> f64 {
        (*self - other).length()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 { Vec2::new(self.x + rhs.x, self.y + rhs.y) }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 { Vec2::new(self.x - rhs.x, self.y - rhs.y) }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// ─── Custom Index ─────────────────────────────────────────────────────────────

struct Grid<T> {
    data: Vec<T>,
    cols: usize,
    rows: usize,
}

impl<T: Default + Clone> Grid<T> {
    fn new(rows: usize, cols: usize) -> Self {
        Grid {
            data: vec![T::default(); rows * cols],
            cols,
            rows,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            self.data.get(row * self.cols + col)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &T {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.data[row * self.cols + col]
    }
}

// ─── Trait Objects & Dynamic Dispatch ────────────────────────────────────────

trait Renderable {
    fn render(&self) -> String;
    fn bounding_box(&self) -> (Vec2, Vec2);
    fn area(&self) -> f64;
    fn name(&self) -> &str;

    // Default implementation using other methods
    fn describe(&self) -> String {
        let (min, max) = self.bounding_box();
        format!(
            "{}: area={:.2}, bounds=[{} → {}]",
            self.name(), self.area(), min, max
        )
    }
}

struct Circle {
    center: Vec2,
    radius: f64,
}

struct Rect {
    top_left: Vec2,
    bottom_right: Vec2,
}

struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2,
}

impl Renderable for Circle {
    fn render(&self) -> String {
        format!("⬤ Circle at {} r={:.2}", self.center, self.radius)
    }
    fn bounding_box(&self) -> (Vec2, Vec2) {
        let r = self.radius;
        (
            Vec2::new(self.center.x - r, self.center.y - r),
            Vec2::new(self.center.x + r, self.center.y + r),
        )
    }
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Renderable for Rect {
    fn render(&self) -> String {
        format!("▬ Rect {} → {}", self.top_left, self.bottom_right)
    }
    fn bounding_box(&self) -> (Vec2, Vec2) {
        (self.top_left, self.bottom_right)
    }
    fn area(&self) -> f64 {
        let w = (self.bottom_right.x - self.top_left.x).abs();
        let h = (self.bottom_right.y - self.top_left.y).abs();
        w * h
    }
    fn name(&self) -> &str { "Rect" }
}

impl Renderable for Triangle {
    fn render(&self) -> String {
        format!("△ Triangle {} {} {}", self.a, self.b, self.c)
    }
    fn bounding_box(&self) -> (Vec2, Vec2) {
        let min_x = self.a.x.min(self.b.x).min(self.c.x);
        let min_y = self.a.y.min(self.b.y).min(self.c.y);
        let max_x = self.a.x.max(self.b.x).max(self.c.x);
        let max_y = self.a.y.max(self.b.y).max(self.c.y);
        (Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
    }
    fn area(&self) -> f64 {
        // Shoelace formula
        ((self.a.x * (self.b.y - self.c.y)
            + self.b.x * (self.c.y - self.a.y)
            + self.c.x * (self.a.y - self.b.y)) / 2.0).abs()
    }
    fn name(&self) -> &str { "Triangle" }
}

// ─── impl Trait vs Box<dyn Trait> ────────────────────────────────────────────

// impl Trait in return position — single concrete type, zero-cost
fn make_default_shape() -> impl Renderable {
    Circle {
        center: Vec2::zero(),
        radius: 1.0,
    }
}

// Box<dyn Trait> — dynamic dispatch, heap-allocated, multiple types possible
fn make_shape(kind: &str) -> Box<dyn Renderable> {
    match kind {
        "circle" => Box::new(Circle { center: Vec2::new(0.0, 0.0), radius: 5.0 }),
        "rect"   => Box::new(Rect   { top_left: Vec2::new(0.0, 0.0), bottom_right: Vec2::new(10.0, 5.0) }),
        _        => Box::new(Triangle { a: Vec2::new(0.0, 0.0), b: Vec2::new(4.0, 0.0), c: Vec2::new(2.0, 3.0) }),
    }
}

fn total_area(shapes: &[Box<dyn Renderable>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ─── Blanket Implementations ──────────────────────────────────────────────────

trait Summary {
    fn summarize(&self) -> String;
}

// Blanket impl: any T that implements Display also gets Summary for free
impl<T: fmt::Display> Summary for T {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)
    }
}

// ─── Associated Types vs Generics ─────────────────────────────────────────────

// Associated type — one impl per type
trait Converter {
    type Output;
    fn convert(self) -> Self::Output;
}

struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

impl Converter for Celsius {
    type Output = Fahrenheit;
    fn convert(self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

impl Converter for Fahrenheit {
    type Output = Celsius;
    fn convert(self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
}

impl Converter for Kelvin {
    type Output = Celsius;
    fn convert(self) -> Celsius {
        Celsius(self.0 - 273.15)
    }
}

impl fmt::Display for Celsius    { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.2}°C", self.0) } }
impl fmt::Display for Fahrenheit { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.2}°F", self.0) } }
impl fmt::Display for Kelvin     { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.2}K",  self.0) } }

// ─── Trait Inheritance ────────────────────────────────────────────────────────

trait Named {
    fn name(&self) -> &str;
}

trait Described: Named {
    fn description(&self) -> &str;

    fn full_description(&self) -> String {
        format!("{}: {}", self.name(), self.description())
    }
}

trait Categorized: Named {
    fn category(&self) -> &str;

    fn label(&self) -> String {
        format!("[{}] {}", self.category(), self.name())
    }
}

trait Item: Described + Categorized {
    fn price(&self) -> f64;

    fn formatted_price(&self) -> String {
        format!("${:.2}", self.price())
    }
}

struct Product {
    name: String,
    description: String,
    category: String,
    price: f64,
}

impl Named      for Product { fn name(&self) -> &str        { &self.name } }
impl Described  for Product { fn description(&self) -> &str { &self.description } }
impl Categorized for Product { fn category(&self) -> &str   { &self.category } }
impl Item for Product {
    fn price(&self) -> f64 { self.price }
}

// ─── Custom Iterator via Trait ────────────────────────────────────────────────

struct Chunks<'a, T> {
    data: &'a [T],
    chunk_size: usize,
    pos: usize,
}

impl<'a, T> Chunks<'a, T> {
    fn new(data: &'a [T], chunk_size: usize) -> Self {
        Chunks { data, chunk_size, pos: 0 }
    }
}

impl<'a, T> std::iter::Iterator for Chunks<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let end = (self.pos + self.chunk_size).min(self.data.len());
        let chunk = &self.data[self.pos..end];
        self.pos = end;
        Some(chunk)
    }
}

// ─── PartialOrd / Ord implementations ────────────────────────────────────────

#[derive(Debug, Clone, Eq, PartialEq)]
struct Priority {
    level: u8,  // 1 = low, 5 = critical
    label: String,
}

impl Priority {
    fn new(level: u8, label: &str) -> Self {
        Priority { level: level.clamp(1, 5), label: label.to_string() }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let star = "★".repeat(self.level as usize);
        write!(f, "{} [{}] {}", star, self.level, self.label)
    }
}

// ─── Plugin / Extension Pattern ───────────────────────────────────────────────

trait Plugin: fmt::Debug {
    fn name(&self) -> &str;
    fn on_start(&self);
    fn on_stop(&self);
    fn process(&self, input: &str) -> String;
}

#[derive(Debug)]
struct UppercasePlugin;

#[derive(Debug)]
struct TrimPlugin;

#[derive(Debug)]
struct ReversePlugin;

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str { "uppercase" }
    fn on_start(&self) { println!("  UppercasePlugin started"); }
    fn on_stop(&self)  { println!("  UppercasePlugin stopped"); }
    fn process(&self, input: &str) -> String { input.to_uppercase() }
}

impl Plugin for TrimPlugin {
    fn name(&self) -> &str { "trim" }
    fn on_start(&self) { println!("  TrimPlugin started"); }
    fn on_stop(&self)  { println!("  TrimPlugin stopped"); }
    fn process(&self, input: &str) -> String { input.trim().to_string() }
}

impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "reverse" }
    fn on_start(&self) { println!("  ReversePlugin started"); }
    fn on_stop(&self)  { println!("  ReversePlugin stopped"); }
    fn process(&self, input: &str) -> String { input.chars().rev().collect() }
}

struct PluginPipeline {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginPipeline {
    fn new() -> Self { PluginPipeline { plugins: Vec::new() } }

    fn register(&mut self, plugin: Box<dyn Plugin>) {
        println!("Registering plugin: {}", plugin.name());
        plugin.on_start();
        self.plugins.push(plugin);
    }

    fn run(&self, input: &str) -> String {
        self.plugins.iter().fold(input.to_string(), |acc, p| p.process(&acc))
    }

    fn shutdown(&self) {
        for plugin in &self.plugins {
            plugin.on_stop();
        }
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Traits In Depth ===\n");

    // Vec2 with operator overloading
    println!("── Vec2 Math ──");
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);
    println!("a = {}, b = {}", a, b);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * 2 = {}", a * 2.0);
    println!("-a    = {}", -a);
    println!("|a|   = {:.2}", a.length());
    println!("â     = {}", a.normalized());
    println!("a·b   = {:.2}", a.dot(b));
    println!("dist  = {:.2}", a.distance_to(b));

    // Grid with custom Index
    println!("\n── Grid ──");
    let mut grid: Grid<i32> = Grid::new(3, 3);
    grid[(0, 0)] = 1; grid[(1, 1)] = 5; grid[(2, 2)] = 9;
    println!("grid[0,0] = {}", grid[(0, 0)]);
    println!("grid[1,1] = {}", grid[(1, 1)]);
    println!("grid[1,0] = {:?}", grid.get(1, 0));

    // Trait objects and dynamic dispatch
    println!("\n── Shapes (dynamic dispatch) ──");
    let shapes: Vec<Box<dyn Renderable>> = vec![
        make_shape("circle"),
        make_shape("rect"),
        make_shape("triangle"),
    ];
    for s in &shapes {
        println!("{}", s.render());
        println!("  {}", s.describe());
    }
    println!("Total area: {:.2}", total_area(&shapes));

    // impl Trait
    let default_shape = make_default_shape();
    println!("\nimpl Trait shape: {}", default_shape.render());

    // Temperature converter
    println!("\n── Temperature Conversions ──");
    let boiling = Celsius(100.0);
    println!("{} → {}", boiling, Celsius(100.0).convert());
    println!("{} → {}", Fahrenheit(32.0), Fahrenheit(32.0).convert());
    println!("{} → {}", Kelvin(373.15), Kelvin(373.15).convert());

    // Blanket impl
    println!("\n── Blanket Summary ──");
    println!("{}", 42_i32.summarize());
    println!("{}", "hello world".summarize());
    println!("{}", 3.14_f64.summarize());

    // Trait inheritance and Item
    println!("\n── Product ──");
    let laptop = Product {
        name: "ThinkPad X1".to_string(),
        description: "Lightweight business laptop".to_string(),
        category: "Electronics".to_string(),
        price: 1499.99,
    };
    println!("{}", laptop.full_description());
    println!("{}", laptop.label());
    println!("Price: {}", laptop.formatted_price());

    // Custom iterator via trait
    println!("\n── Chunks Iterator ──");
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    for chunk in Chunks::new(&data, 3) {
        println!("chunk: {:?}", chunk);
    }

    // PartialOrd + sort
    println!("\n── Priorities ──");
    let mut tasks = vec![
        Priority::new(2, "Write docs"),
        Priority::new(5, "Fix prod crash"),
        Priority::new(1, "Update README"),
        Priority::new(4, "Code review"),
        Priority::new(3, "Add tests"),
    ];
    tasks.sort();
    for t in &tasks {
        println!("{}", t);
    }

    // Plugin pipeline
    println!("\n── Plugin Pipeline ──");
    let mut pipeline = PluginPipeline::new();
    pipeline.register(Box::new(TrimPlugin));
    pipeline.register(Box::new(UppercasePlugin));
    pipeline.register(Box::new(ReversePlugin));
    let result = pipeline.run("  hello world  ");
    println!("Input:  '  hello world  '");
    println!("Output: '{}'", result);
    pipeline.shutdown();

    println!("\n=== Done ===");
}