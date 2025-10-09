fn main() {
    println!("Hello, world!");
    // slice -- slices are references and so they follow borrowing rules
    // 3 types of strings

    /*let mut s = String::from("chandu");
    s.push_str("mndrpu"); //can  modify its located in heap so u can add and remove characters

    // second type string this is a borrowed slice but u cant modify it 

    let a : &str = "hello"; // string literal 
    let a2 : &str = &String::from("hello"); // borrowed from string

    // hardcoded strings literals

    let c = "chandu"

    // practing all 3 of em 

    let r : &str = "ram";

    let r: String = "ram".to_string();
    let r: String = String::from("ram");

    let r = String::from("ram");
    let r_slice: &str = &r;
    let r_slice: &str = &s[..]; // full slice

    let r_slice = "ram";
    let r = r_slice.to_string(); */

    // string slices in action 

    let a = String::from("achyutha hari");
    // let achyutha = &a[0..7];
    // let hari = &a[8..12];
    // println!("{} {}",achyutha,hari);

    // ranges 

    let achyutha = &a[..7];
    let hari = &a[8..];
    println!("{} {}",achyutha,hari);

    // iteration by characters

    let b = String::from("sitarama");
    for c in b.chars() {
        println!("{}",c);
    }
    // iteration by bytes
    for d in b.bytes() {
        println!("{}",d);
    }

    // ok now string methods

    let mut h = String::from("radha krishna");

    let trimmed = h.trim(); //trims whitespace
    // splitting bby delimeter
    let words: Vec<&str> = h.trim().split('').collect();

    // replacing

    let replaced = h.replace("krishna","murari");

    // contains
    if h.contains("radha"){
        println!("found it");
    }

    // parsing to other types

    let num_str = "42";
    let num : i32 = num_str.parse().unwrap();

    // uper and lowercase

    let upper = h.to_uppercase();
    let lower = h.to_lowercase();

    // concatnation

    let h1 = String::from("radha, ");
    let h2 = String::from("krishna!");

    // another method using taking ownership of left side

    let h3 = h1 + &h2;

    // third method format! macro (doesnt need ownership)

    let h4 = String::from("Hello, ");
    let h5 = format!("{}{}",h1,h2); 

}
