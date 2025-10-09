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

// string reverser using slices
// reverese whole string
fn reverse_string(s: &str) -> String{
    s.chars().rev().collect()
}

// reverse words in string first 

fn reverse_word(s :&str) -> String {
        .split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
} 

fn reverse_word_order(s: &str) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn reverse_substring(s: &str, start: usize, end: usize) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    
    if start >= chars.len() || end > chars.len() || start >= end {
        return s.to_string(); // Invalid range, return original
    }
    
    // Reverse the specified range
    chars[start..end].reverse();
    chars.iter().collect()
}

fn main() {
    let x = 10;
    let y = 5;
    
    println!("{}", calculate("+", &x, &y));
    println!("{}", calculate("-", &x, &y));
    println!("{}", calculate("*", &x, &y));
    println!("{}", calculate("/", &x, &y));

    let text = "Hello World";
    
    println!("Original: {}", text);
    println!("Reversed: {}", reverse_string(text));
    println!("Reversed words: {}", reverse_words(text));
    println!("Reversed order: {}", reverse_word_order(text));
    println!("Reverse 0..5: {}", reverse_substring(text, 0, 5));
}