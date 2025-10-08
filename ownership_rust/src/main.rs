/* 
fn main() {
    // println!("Hello, world!");

//     the problem that ownership solves, because if you don't understand the problem, the solution won't make sense.
// The Memory Problem in Programming:
// Your computer has limited memory (RAM). When your programRetryClaude does not have the ability to run the code it generates yet.CContinueruns, it needs to store data in memory. There are three ways programming languages handle memory:
    // ownereship


    // stack and heap

    // -----STACK-----

//     1. THE STACK (Fast, Organized, Limited)
//  You can only add to the top (push) or remove from the top (pop). Last in, first out.
// When a function runs:

// Its variables are pushed onto the stack
// When the function ends, its variables are popped off

// rustfn main() {
//     let x = 5;      // x pushed onto stack
//     let y = 10;     // y pushed onto stack
    
//     {
//         let z = 15; // z pushed onto stack
//     }               // z popped off! (out of scope)
    
//     println!("{}", x);  // x still here
//     // println!("{}", z);  // ERROR! z doesn't exist anymore
// }
// Stack characteristics:

// Fast - Adding/removing is instant
// Fixed size - Must know size at compile time
// Automatic cleanup - When scope ends, variables disappear
// Limited space - Usually a few megabytes

// -----HEAP------

// The heap is like a huge warehouse. You can allocate (reserve) space whenever you need it, but you have to manage it.
// When you need the heap:

// Data size unknown at compile time
// Data that needs to live longer than a function
// Large data (moving stuff on the stack is copying, which is slow for big things)


// fn main() {
//     let s = String::from("hello");  // This data is on the HEAP
// }

//  --- OWNERSHIP ----

// RULES

// THE THREE OWNERSHIP RULES
// Memorize these. Everything else follows from them:

// Each value in Rust has an OWNER
// There can only be ONE owner at a time
// When the owner goes out of scope, the value is DROPPED (freed)

// Let's break down what this means.
// {
//     let s = String::from("chandu"); //s owns this string
//     // s is the owner of string here
//     println!("{}",s);

// }
// when goes s is out of scope here s is no longer owner and rust automatically calls drop to free the memory

// rule 2 only one owner at a time 

// let s1 = String::from("hello");
// let s2 = s1; // ownership of s1 is shifted to s2 now
// println!("value of s2 is {}",s2);
// println!("{}",s1) // throws error no longer owns the value
// here s1 ownership is moved to s2 it didnt copy the valaue it just gave its ownersship

// works diff in stack like with integer simple types

    let x = 5;
    let y = x;  // COPY, not move
    
    println!("{}", x);  // Works!
    println!("{}", y);  // Also works!

// integers are stored on stack and they are very tiny so they can get easily copied in stack but
// strings are stored on heap and the pointer to them is on stack but copying the data would be slow so heap types like string vector dont use copy they instead use move 

// MOVE SEMANTICES IN FUNCTIONS WHEN U PASS A VALUE TO A FUNCTION IT MOVESSSS

let s = String::from("hello");
take_ownership(s); // s moves into function 
// println!("{}",s); //error s is moved already

fn take_ownership(some_string: String) {
    println!("{}",some_string);
}

// RETURNING OWNERSHIP

let s1 = String::from("hello");
// owenrship from s1 is moved to s2 now and 

let s2 = take_and_give_back(s1);
// u can no longer access s1
println!("{}",s2); // this works as s2 is owner

fn take_and_give_back(a_string: String) -> String {
    println!("{}",a_string);
    a_string // return moves ownership back to caller
}

// CLONEEEEEE two variables owning same data makes a complete copy of heap data 

// let s3 = String::from("chand");
// let s4 = s3.clone(); // deep copy - duplicates heap data

// println!("{}",s1);
// println!("{}",s2);

// s1 -> "hello" on heap (original)
// s2 -> "hello" on heap (copy)
//two sperate strings two sperate owners 


// ownership with functions and scope

let s9 = String::from("hello");
    
    {
        let s2 = 9;  // s moved to s2
        println!("{}", s2);
    }  // s2 dropped here, memory freed
    
    // println!("{}", s9);  // ERROR! s9 was moved

*/


// practice sessions

fn main() {
    // basic ownership s1 -> s2 moved
    let s1 = String::from("RUST");
    let s2 = s1;

    println!("{}",s2);

    let s3 = String::from("ownership");
    takes_ownership(s3);

    let s4 = gives_ownership();
    println!("{}",s4);

    let s5 = String::from("hello");
    let s3 = take_and_give_back(s5);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); 
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn take_and_give_back(a_string:String) -> String {
    a_string
}