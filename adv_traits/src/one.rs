use std::ops::{Add, Mul, Sub, Div, Neg};
use std::fmt;
use std::io::{self, Write};

trait Scalar:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + fmt::Debug
{
    fn zero() -> Self;
    fn one() -> Self;
    fn from_i32(n: i32) -> Self;
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn from_i32(n: i32) -> Self { n as f64 }
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
    fn from_i32(n: i32) -> Self { n }
}

#[derive(Clone, Copy, Debug)]
struct Vec2<T: Scalar> {
    x: T,
    y: T,
}

impl<T: Scalar> Vec2<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }
    fn dot(self, other: Self) -> T { self.x * other.x + self.y * other.y }
    fn norm_sq(self) -> T { self.dot(self) }
}

impl<T: Scalar> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Scalar> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Scalar> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, s: T) -> Self::Output {
        Self::new(self.x * s, self.y * s)
    }
}

impl<T: Scalar> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, s: T) -> Self::Output {
        Self::new(self.x / s, self.y / s)
    }
}

impl<T: Scalar> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

trait Animal {
    fn noise(&self) -> &'static str;
    fn legs(&self) -> u8;
    fn speed(&self) -> u8;
}

struct Dog;
struct Cat;
struct Spider;

impl Animal for Dog {
    fn noise(&self) -> &'static str { "woof" }
    fn legs(&self) -> u8 { 4 }
    fn speed(&self) -> u8 { 60 }
}

impl Animal for Cat {
    fn noise(&self) -> &'static str { "meow" }
    fn legs(&self) -> u8 { 4 }
    fn speed(&self) -> u8 { 50 }
}

impl Animal for Spider {
    fn noise(&self) -> &'static str { "..." }
    fn legs(&self) -> u8 { 8 }
    fn speed(&self) -> u8 { 5 }
}

fn race<A: Animal, B: Animal>(a: &A, b: &B) -> &'static str {
    if a.speed() > b.speed() { "A wins" } else { "B wins" }
}

trait Monoid {
    fn id() -> Self;
    fn op(self, other: Self) -> Self;
}

impl Monoid for i32 {
    fn id() -> Self { 0 }
    fn op(self, other: Self) -> Self { self + other }
}

impl Monoid for String {
    fn id() -> Self { String::new() }
    fn op(mut self, other: Self) -> Self { self.push_str(&other); self }
}

fn fold<T: Monoid, I: Iterator<Item = T>>(iter: I) -> T {
    iter.fold(T::id(), |acc, x| acc.op(x))
}

fn main() {
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(1.0, 2.0);
    let v3 = v1 + v2;
    let d = v3.dot(v2);
    let dog = Dog;
    let cat = Cat;
    let res = race(&dog, &cat);
    let nums = vec![1, 2, 3, 4, 5];
    let sum = fold(nums.into_iter());
    let strings = vec!["a".into(), "b".into(), "c".into()];
    let concat = fold(strings.into_iter());
    io::stdout().write_all(format!("{:?} {} {} {}\n", v3, d, sum, concat).as_bytes()).unwrap();
}