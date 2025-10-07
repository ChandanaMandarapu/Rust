// enums

enum Traffic {
    Red,
    Yellow,
    Green,
}

enum Shape {
    Square(f64),
    Circle(f64),
    Rectangle(f64, f64),
}

fn main() {
    let my_sign = Traffic::Yellow;
    let next_sign = my_sign;
    move_around(next_sign);

    let square = Shape::Square(8.0);
    let circle = Shape::Square(6.0);
    let rectangle = Shape::Rectangle(9.0, 8.0);

    caluclate_area(rectangle);
    caluclate_area(circle);
    caluclate_area(square);
}

fn move_around(_light: Traffic) {
    println!("getsteadygo");
}

// pattern matching
fn caluclate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(s) => s * s,
    };

    println!("Area is: {}", area);
    return area;
}
