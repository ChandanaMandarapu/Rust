// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = String::from("CHANDU");

//     let s3 = s1;

//     // RUST VALUE IS OWNED BY s1
//     // here we are borrowing the reference of the s1 string from above 
//     let len = caluclate_length(&s1);
//     println!("length of '{}' is {}", s1, len);

//     // practicing the same example again
//     let leng = caluclate_namelength(&s2);
//     println!("length of '{}' is {}", s2, leng);
// }

// // this s is a reference to that above string
// fn caluclate_length(s: &String) -> usize {
//     s.len()
// }

// fn caluclate_namelength(s: &String) -> usize {
//     s.len()
// }

// // 2nd rule there can be only one owner at a time

// 3rd rule

fn main() {
    let s1 = String::from("RUST");
    let len = caluclate_length(&s1);
    println!("length of '{}' is {}", s1, len);
}
// s1 goes out of scope and its value it wil be dropped

fn printLost(s: &string) {
    println!("{}",&s1);
}
fn caluclate_length(s: &String) -> usize {
    s.len()
}