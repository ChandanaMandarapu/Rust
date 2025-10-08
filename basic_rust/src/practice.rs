// practice.rs

// 1️⃣ Calculator
pub fn calculator() {
    let num1 = 10.0;
    let num2 = 3.0;
    let operation = '*';

    let result = calculate(num1, num2, operation);
    println!("{} {} {} = {}", num1, operation, num2, result);

    fn calculate(a: f64, b: f64, op: char) -> f64 {
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    println!("Error: division by zero");
                    0.0
                } else {
                    a / b
                }
            }
            _ => {
                println!("Unknown operation");
                0.0
            }
        }
    }
}


// fizzbuzzz

pub fn fizzbuzz_program() {
    for num 1 1..=30{
        fizzbuzz(num);
    }
    fn fizzbuzz(n:i32){
        match(n%3,n%5){
            (0,0) => println!("fizzbuzz");
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", n),
        }
    }
 }
