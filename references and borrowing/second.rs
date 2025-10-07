// variables and mutability and constants
// every varaible in rust is by default is immutable u cant change it
// try changing into mutable

fn main() {
    println!("hello");
    let mut _a: i32 = 6;
    println!("value of a is {}",a);
    a = 10;
    // const mut y = 10 wrong format
    // a constant is by default immutable and u cant change it to mutable wahh wahh wahh

    const Y: i32 = 10;
    println!("value of Y is {}",Y);
    println!("the value of threee hours in seconds is : {}",THREE_HOURS_IN_SECONDS);
}

const PI: f64 = 3.14;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


