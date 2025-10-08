fn main () {
    // first exercise - function ownership transfer write funcns that take ownership and return it

    let s = String::from("hello");
    let s = add_world(s); // shadowing
    println!("{}",s);
    let s = add_exclamation(s);
    println!("{}",s);

    // second exercise - multiple funcn calls

    let n = String::from("chandu");
    print_length(n.clone());
    print_uppercase(n.clone());

    // copy traits for integers

    let x = 8;
    let y = x;
    println!("x:{},y:{}",x,y);

    takes_integer(x);
    println!("x after func : {}",x);
    
    let c = String::from("sina");
    let d = c;
    takes_string(d);
}
// takes and returns ownership
fn add_world(mut s : String) -> String {
    s.push_str("world");
    s 
}
// takes and returns ownership
fn add_exclamation(mut s : String) -> String{
    s.push_str("!")
    s
}
fn print_length(n : String) {
    println!("lengthi :{}",n.len());
}
fn print_uppercase(n: String) {
    println!("uppercase: {}",n.to_uppercase());
}
fn takes_integer(n:i32){
    println!("Integer: {}",n);
}
fn takes_string (s:String) {
    println!("String: {}",s);
}