fn main() {
    println!("Hello, world!");
    let x: i32 = -90;
    println!("x: {}", x);
    let hello: String = String::from("hello world");
    println!("{}", hello);

    // ownership us a set of rules that govern how rust program manages memory rust here being strict asks u to follow all ownership rules and then only compiles the whole code

    // heap variables should have owner and if owner goes out of scope it gets deallocated the memory gets deallocated after s1 valuse is assigned to s2 s1 is no longer valid now the owner of data on heap is s2

    let s1 = String::from("hello there");
    let s2: String = s1;
    println!("{}", s2);

    // ownership - invalid code when the owner is changed to a function here my string is no longer the owner as now the ownership is transferred to the funcn takes ownership u cant accesss as heap has only one owner at a time. But we can fix it by return some string and making my string mutably changed and then we can use it actually this method is without references

    // or else u can .clone() also

    let mut my_string = String::from("hello");
    my_string = takes_ownership(my_string);
    println!("{}", my_string);

    // references
    // references mean giving the address of a string than the ownership of the string ovre to a function

    let s = String::from("hello");
    let v = &s;

    println!("{}", v);

    // borrowing example

    let our_str = String::from("hey");
    borrow_variable(&our_str);
    println!("{}", our_str);
    // let there_str = &our_str;

    // mutable reference passing

    // rules of mutable references borrowing is once u borrow a mutable ref u can neither pass a mutable borrow nor an immutable reference borrow

    let mut c = String::from("chandu");
    update_str(&mut c);
    println!("{}", c);
}
// ownershipfunc
fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}
// borrowing reference
fn borrow_variable(u_str: &String) {
    println!("{}", u_str);
}

// mutable reference func

fn update_str(s: &mut String) {
    s.push_str("MANDARAPU");
}
