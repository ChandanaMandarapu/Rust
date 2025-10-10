// destructuring structs

struct Points{
    x:i32;
    y:i32;
}

// partial destructuring (..ignore remaining field)

struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    // destructuring tuples
    let point = (10,20);

    match point {
        (0,0) => println!("orgin");
        (x, 0) => println!("On the x-axis at {}", x),
        (0, y) => println!("On the y-axis at {}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    };

    let points = Point(x:10,y:20);
    match points {
        Point { x: 0, y: 0 } => println!("At origin"),
        Point { x: 0, y } => println!("On y-axis at y={}", y),
        Point { x, y: 0 } => println!("On x-axis at x={}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    };

    let person = person{
        name: String::from("achyutha"),
        age: 9,
        email:String::from("achyutha@googl.com");
    };

    let Person { name, age, .. } = person;
    println!("{} is {} years old", name, age);
    // person has been moved (name and age moved out). Can't use `person` after this.

    // Destructuring arrays and slices

    let array = [1,2,3,4,5];
    match array{
        [1,2,3,4,5] => println!("exact match"),
        [first,second,..] => {
            println!("starts with {} and {}",first,second);
        }
        _ => println!("Something else"),
    }

    // fixed size pattern binding

    let numberss = [1,2,3];
    let[a,b,c] = numberss;
    println!("a={}, b={}, c={}", a, b, c);
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
    
    fn wait_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
    
    fn can_go(&self) -> bool {
        match self {
            TrafficLight::Green => true,
            _ => false,
        }
    }
}

enum Command {
    Move { direction: String, steps: u32 },
    Attack { target: String, damage: u32 },
    Heal { amount: u32 },
    Quit,
}

fn execute_command(cmd: Command) {
    match cmd {
        Command::Move { direction, steps } if steps == 0 => {
            println!("Cannot move 0 steps");
        }
        Command::Move { direction, steps } => {
            println!("Moving {} steps {}", steps, direction);
        }
        Command::Attack { target, damage } if damage > 100 => {
            println!("CRITICAL HIT on {} for {} damage!", target, damage);
        }
        Command::Attack { target, damage } => {
            println!("Attacking {} for {} damage", target, damage);
        }
        Command::Heal { amount } if amount > 50 => {
            println!("Major heal: +{} HP", amount);
        }
        Command::Heal { amount } => {
            println!("Healing: +{} HP", amount);
        }
        Command::Quit => {
            println!("Exiting game...");
        }
    }
}

enum FileOperation {
    Read { bytes: u64 },
    Write { bytes: u64 },
    Delete,
    Error { code: i32, message: String },
}

fn log_operation(op: FileOperation) {
    match op {
        FileOperation::Read { bytes } if bytes == 0 => {
            println!("Warning: Read operation returned no data");
        }
        FileOperation::Read { bytes } => {
            println!("Successfully read {} bytes", bytes);
        }
        FileOperation::Write { bytes } if bytes > 1_000_000 => {
            println!("Large write operation: {} MB written", bytes / 1_000_000);
        }
        FileOperation::Write { bytes } => {
            println!("Written {} bytes", bytes);
        }
        FileOperation::Delete => {
            println!("File deleted successfully");
        }
        FileOperation::Error { code, message } if code == 404 => {
            println!("File not found: {}", message);
        }
        FileOperation::Error { code, message } => {
            println!("Error {}: {}", code, message);
        }
    }
}

enum Discount {
    Percentage(f64),
    FixedAmount(f64),
    BuyOneGetOne,
    NoDiscount,
}

fn calculate_price(original: f64, discount: Discount) -> f64 {
    match discount {
        Discount::Percentage(percent) if percent > 100.0 => {
            println!("Warning: Invalid discount percentage");
            original
        }
        Discount::Percentage(percent) => {
            let discount_amount = original * (percent / 100.0);
            original - discount_amount
        }
        Discount::FixedAmount(amount) if amount > original => {
            println!("Warning: Discount exceeds price, making it free");
            0.0
        }
        Discount::FixedAmount(amount) => original - amount,
        Discount::BuyOneGetOne => original / 2.0,
        Discount::NoDiscount => original,
    }
}

enum Discount {
    Percentage(f64),
    FixedAmount(f64),
    BuyOneGetOne,
    NoDiscount,
}

fn calculate_price(original: f64, discount: Discount) -> f64 {
    match discount {
        Discount::Percentage(percent) if percent > 100.0 => {
            println!("Warning: Invalid discount percentage");
            original
        }
        Discount::Percentage(percent) => {
            let discount_amount = original * (percent / 100.0);
            original - discount_amount
        }
        Discount::FixedAmount(amount) if amount > original => {
            println!("Warning: Discount exceeds price, making it free");
            0.0
        }
        Discount::FixedAmount(amount) => original - amount,
        Discount::BuyOneGetOne => original / 2.0,
        Discount::NoDiscount => original,
    }
}