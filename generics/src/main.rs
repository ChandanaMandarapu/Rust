// generics - code that works with any type

// generics are like placeholder fro a type instead of saying this func works with a specific type u say this func works with some type T when we use the function 

// generics are resolved at compile time - like for example when u use a generic function with i32 rust generates a version of that function specifically for i32 and when use it f64 rust generates another veersion for f64 - monomorphization again

// here t could be anything a number a string a struct rust dont know -> supports so added a trait bound whcih shows T is a datatype that can be compared..
// <T: PartialOrd + Copy> - This is the generic type parameter with trait bounds
// // T is our placeholder type name (you can use any name, but T is conventional)
// : PartialOrd means "T must implement the PartialOrd trait" (which provides the > operator)
// + Copy means "T must also implement the Copy trait" (so we can copy values around easily)

// Why Copy? When we do let mut largest = list[0], we're copying the first element. And when we do for &item, we're copying each item. Without Copy, we'd be trying to move values, which gets complicat

fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// GENERIC STRUCTS

// point can hold any type both x and y must be the same type at the end  just remember her both x and y should hold the same type ntg else
struct Point<T> {
    x: T,
    y: T,
}

// implemeting methods on generic types
// impl<T> - We're implementing for a generic type T
// Point<T> - We're implementing for Point of any type T
// The method x() returns a reference to the x coordinate

// fn print_comparison<T>(a: T, b: T)
// where
//     T: std::fmt::Display + PartialOrd,
// {
//     if a > b {
//         println!("{} is greater than {}", a, b);
//     } else {
//         println!("{} is less than or equal to {}", a, b);
//     }
// }

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 2 diff types - 2 diff parameters

struct Point2<T, U> {
    x: T,
    y: U,
}

// GENERIC ENUMS
enum MyOption<T> {
    Some(T),
    None,
}

fn complex_function<T, U>(t: T, u: U)
where
    T: Clone + std::fmt::Display,
    U: Clone + std::fmt::Debug,
    // This says: "T must be cloneable and printable with Display, U must be cloneable and printable with Debug."
{
    let t_clone = t.clone();
    let u_clone = u.clone();
    println!("t : {}", t_clone);
    println!("u : {:?}", u_clone);
}

fn main() {
    let numbers = vec![32, 40, 39, 100, 83];
    let result = find_largest(&numbers);
    println!("The largest number is {}", result);

    let floats = vec![3.14, 2.71, 1.41, 9.99];
    let result = find_largest(&floats);
    println!("The largest float is {}", result);

    // struct exectution
    let integer_point = Point { x: 5, y: 10 }; // Point<i32>
    let float_point = Point { x: 1.0, y: 4.0 }; // Point<f64>

    println!("integer_point.x = {}", integer_point.x());
    println!("float_point.x = {}", float_point.x());

    // struct with diff datatypes execution 
    let mixed_point = Point2 { x: 3, y: 4.0 };

    complex_function(42, "Hello");
    complex_function("Rust", vec![1, 2, 3]);
}
