use std::fmt::Display;

trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    size: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.size * self.size
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.1415 * self.radius * self.radius
    }
}

fn total_area<T: Shape>(items: Vec<T>) -> f64 {
    items.into_iter().map(|x| x.area()).sum()
}

fn main() {
    let mut squares = Vec::new();
    for i in 1..150 {
        squares.push(Square { size: i as f64 });
    }

    let mut circles = Vec::new();
    for i in 1..150 {
        circles.push(Circle { radius: i as f64 });
    }

    println!("{}", total_area(squares));
    println!("{}", total_area(circles));
}
