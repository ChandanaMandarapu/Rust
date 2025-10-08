// pub fn so other files can use it 
// Rust hides everything by default.
// If you create a function inside a module (another .rs file) and want to use it outside that module, you must mark it pub.


pub fn say_hello() {
    println!("Hello from funcns.rs!");
}
pub fn greet_person(name :&str){
    println!("hello,{}",name);
}

// two or more parameters
// You MUST specify types for parameters. Rust won't infer them. This is intentional - it makes functions clear and prevents bugs.

pub fn add(a: i32, b: i32) -> i32 {
    // returning value no ; in rust last expression in fun is automatically returned if u add ; it becomes a statement
    a + b
}

pub fn sub (c:i32, d:i32) -> i32 {
    c - d
}

pub fn mul (e:i32, f:i32) -> i32 {
    e * f
}
