// creating 5 traits

// TRAIT ONE - DESCRIBABLE why this is imp sometimes in real word scenarios we often need to convert things to human-readable strings. This trait enforces that any type can describe itself.

trait Describable {
    fn describable(&self) -> String;
}

// TRAIT TWO - RESETTABLE most imp thing to notice here is we are using &mut as the data inside can change 
trait Resettable {
    fn reset(&mut self);
}

// TRAIT THREE - VALIDATABLE
trait Validatable {
    fn is_valid(&self) -> bool;
}

// TRAIT FOUR - CALCULATABLE
trait Calculable {
    fn calculate(&self) -> f64;
}

// TRAIT FIVE - CONVERTIBLE
trait Convertible<T> {
    fn convert(&self) -> T;
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

struct Counter {
    count: i32,
}

struct Email {
    address: String,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

struct Fahrenheit(f64);
struct Celsius(f64);

impl Describable for Book {
    fn describable(&self) -> String {
        format!("{} by {}, {} pages", self.title, self.author, self.pages)
    }
}

impl Resettable for Counter {
    fn reset(&mut self) {
        self.count = 0;
    }
}

impl Validatable for Email {
    fn is_valid(&self) -> bool {
        self.address.contains('@') && self.address.contains('.')
    }
}

impl Calculable for Rectangle {
    fn calculate(&self) -> f64 {
        self.width * self.height
    }
}

impl Calculable for Circle {
    fn calculate(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

impl Convertible<Celsius> for Fahrenheit {
    fn convert(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
}
fn main() {
    // 1️⃣ Describable
    let book = Book {
        title: String::from("Rust Mastery"),
        author: String::from("Ferris Crab"),
        pages: 420,
    };
    println!("Book description: {}", book.describable());

    // 2️⃣ Resettable
    let mut counter = Counter { count: 10 };
    println!("Counter before reset: {}", counter.count);
    counter.reset();
    println!("Counter after reset: {}", counter.count);

    // 3️⃣ Validatable
    let email = Email {
        address: String::from("test@example.com"),
    };
    println!("Is '{}' valid? {}", email.address, email.is_valid());

    // 4️⃣ Calculable
    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let circle = Circle { radius: 3.0 };
    println!("Rectangle area: {}", rect.calculate());
    println!("Circle area: {}", circle.calculate());

    // 5️⃣ Convertible
    let temp_f = Fahrenheit(98.6);
    let temp_c = temp_f.convert();
    println!("{}°F = {}°C", temp_f.0, temp_c.0);
}
