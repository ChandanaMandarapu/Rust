enum Direction {
    North,
    East,
    South,
    West,
}

// enums with some values
enum Shape {
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

// pattern matching of shape

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Square(side) => side * side,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(base, height) => 0.5 * base * height,
    }
}

// pattern matching of move around

fn move_around(direction: Direction) {
    match direction {
        Direction::North => println!("Moving north"),
        Direction::East => println!("Moving east"),
        Direction::South => println!("Moving south"),
        Direction::West => println!("Moving west"),
    }
}

fn main() {
    let my_dirn = Direction::North;
    move_around(my_dirn);

    let square = Shape::Square(9.0);
    let rectangle = Shape::Rectangle(8.9, 9.8);
    let triangle = Shape::Triangle(9.7, 4.5);

    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
    println!("Area of triangle: {}", calculate_area(triangle));

}
