fn main() {
    // ownership concepts
    // copy for stack based concepts a is still usable after copying because it lives fully on stack 
    let a = 8;
    let b = a;
    println!("a = {} , b = {}",a,b);
    // move for heap based values like string vec
    let s1 = String::from("hello");
    // fixing the error s1 gave away ownership to clone. You can't use it anymore
    // let clone = s1;
    // println!("{}",s1);

    // using .clone() method But .clone() is heavier — it copies the heap memory too. Only use when needed.

    let clone = s1.clone();
    println!("{}",s1);
    println!("{}",clone);

    // so how its done borrowing and referencingg lets see

    // immutable borrowing 
    // creating a new string stored in heap s holds a pointer on the stack to "hello" in the heap s owns the data

    // &s = we are not moving but we are borrowint it using &s you can read only access but the original variable s still owns it 

    // mutable borrowing only one mutable borrowing is allowed in rust

    let mut name = String::from("Chandu");
    let s = String::from("hello");
    print_reference(&s);
    println!("{}",s);

    // mutable borrowing

    add_surname(&mut name);
    println!("After : {}",name);

}

fn print_reference(data: &String){
    println!("GOT : {}",data);
}
fn add_surname(data : &mut String){
    data.push_str("Mandarapu");
}
