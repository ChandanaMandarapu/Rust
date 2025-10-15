fn print_ref(s: &String) {
    println!("{}", s);
}
fn main() {
    let name = String::from("Rust");
    print_ref(&name);
}

fn add_text(s: &mut String) {
    s.push_str(" World");
}
fn main() {
    let mut name = String::from("Hello");
    add_text(&mut name);
    println!("{}", name);
}


fn take(s: String) -> String {
    println!("{}", s);
    s
}
fn main() {
    let name = String::from("Rust");
    let name = take(name);
    println!("{}", name);
}
