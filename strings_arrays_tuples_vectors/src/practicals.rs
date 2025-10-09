//  Calculator Using References
// Let's build a calculator that takes references to avoid unnecessary copying:

fn add(a: &i32, b:&i32) -> i32 {
    a+b
}
fn subtract(a: &i32, b:&i32) -> i32 {
    a-b
}
fn multiply(a: &i32, b:&i32) -> i32 {
    a*b
}
fn divide(a: &i32, b:&i32) -> i32 {
    if *b == 0{
        None
    } else {
        Some(*a as f64 / *b as f64)
    }
}

fn power(base: &i32, exp: &u32) -> i32 {
    base.pow(*exp)
}
fn calculate (operation:&str, a:&i32, b:&i32) -> String{
    match operation{
        "add" | "+" => format!("{} + {} = {}", a , b, add(a,b)),
        "subtract" | "-" => format!("{} - {} = {}", a , b, add(a,b)),
        "multiply" | "*" => format!("{} * {} = {}", a , b, add(a,b)),
        "divide" | "/" => {
            match divide(a,b){
                Some(result) => format!("{}/{}={}",a,b,result),
                None => String::from("Error: Division by zero"),
            },
            _ => String::from("unknown")
        }
    }
}

fn main() {
    let x = 10;
    let y = 5;
    
    println!("{}", calculate("+", &x, &y));
    println!("{}", calculate("-", &x, &y));
    println!("{}", calculate("*", &x, &y));
    println!("{}", calculate("/", &x, &y));
}