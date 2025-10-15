use std::io;

fn print_ref(s: &String) {
    println!("{}", s);
}

fn add_text(s: &mut String) {
    s.push_str(" World");
}

fn take(s: String) -> String {
    println!("{}", s);
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[0..i]; }
    }
    &s
}

struct Description<'a> { text: &'a str }

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

struct User { name: String }
impl User {
    fn get_name(&self) -> &str { &self.name }
}

struct Holder<'a> { value: &'a str }
impl<'a> Holder<'a> {
    fn show(&self) -> &str { self.value }
}

fn main() {
    let name = String::from("Rust");
    print_ref(&name);

    let mut hello = String::from("Hello");
    add_text(&mut hello);
    println!("{}", hello);

    let name = String::from("Rust");
    let name = take(name);
    println!("{}", name);

    let text = String::from("hello rust");
    let fw = first_word(&text);
    println!("{}", fw);

    let s = String::from("borrowed text");
    let d = Description { text: &s };
    println!("{}", d.text);

    let s1 = "short";
    let s2 = "longertext";
    println!("{}", longer(s1, s2));

    let u = User { name: String::from("Chandu") };
    println!("{}", u.get_name());

    let mut a = 10;
    let b = &a;
    println!("{}", b);
    let c = &mut a;
    *c += 5;
    println!("{}", c);

    let text = String::from("owned text");
    let h = Holder { value: &text };
    println!("{}", h.show());

    let mut s = String::from("Rust");
    {
        let r1 = &s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    r2.push_str(" Lang");
    println!("{}", r2);
}
