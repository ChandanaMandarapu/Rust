// fn main () {
//     let mut s = String::from("hello");
//     s.push_str(", world!");
//     println!("{}",s);
// }
// here s is not valid anymore we call drop 

fn main () {
    let s1 = 8;
    // let s1 = "hello";
    // runs good butt
    // let s1 = String::from("hello"); throws an error s1 move occurs s1 has type string which doesnt implement the copy trait
    let s2 = s1;
    println!("s2 - {}",s2);
    println!("s1 - {}",s1);
    // runs good

}