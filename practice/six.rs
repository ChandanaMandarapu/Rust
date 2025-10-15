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
// slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[0..i]; }
    }
    &s
}
fn main() {
    let text = String::from("hello rust");
    let fw = first_word(&text);
    println!("{}", fw);
}

//structs with ref

struct Description<'a> {
    text: &'a str,
}
fn main() {
    let s = String::from("borrowed text");
    let d = Description { text: &s };
    println!("{}", d.text);
}


fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
fn main() {
    let s1 = "short";
    let s2 = "longertext";
    println!("{}", longer(s1, s2));
}
