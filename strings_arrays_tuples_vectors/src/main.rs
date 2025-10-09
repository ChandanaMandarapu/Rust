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

    // Arraysss man

    // Arrays in Rust are different from most languages - they're fixed size and part of the type:

    
//     Fixed size: Cannot grow or shrink
// Stack-allocated: Entire array on the stack (if it fits)
// Size is part of type: [i32; 5] and [i32; 6] are different types
// All elements same type: Homogeneous

// creating arrays

// first method
let arr : [i32; 5] = [1,3,4,5,6]; //fixed size and allocate in memory with fixed datatype
// second even if u dont mention rust allocates this to i32

let arr2 = [1,2,3,4,5];

// initialise with same value 

let arr3 = [0,10]; //[0,0,0,0,0,0,0,0,]
let arr4 = [3,5];

// zero sized array 

let arr5 : [i32,0] = [];

println!("{}",arr);

// accessing array elements

let first = arr[0];
let second = arr[1];

// another method Safe access with get (returns Option)

let third = arr.get(0);
let fourth = arr.get(10); // none 

match arr.get(3) {
    Some(&value) => println!("value: {}",value);
    None => println!("out of bounds"),
}

// array slicess

let slice1 : &[i32] = &arr[1..4]; // 3 4 5
let slice2 : &[i32] = &arr[..3]; // 1 3 4
let slice3 : &[i32] = &arr[2..]; //4 5 6
let slice4 : &[i32] = &arr[..]; // entire array

// iteration on arrays

let array = [10,20,30,40,50];

for element in &array{
    println!("{}",element);
}

// method 2 with indices

for i in 0..array.len(){
    println!("{}",array[i]);
}

// method 3 eneumrate index and value

for(index,value)  in array.iter().eneumrate(){
    println!("array[{}] = {}",index,value);
}

// MULTIDIMENSIONAL ARRAYS

let matrix: [[i32; 3]; 2] = [
    [1, 2, 3],
    [4, 5, 6],
];

// Access elements
let element = matrix[0][1]; // 2 (first row, second column)

// Iterate
for row in &matrix {
    for col in row {
        print!("{} ", col);
    }
    println!();
}


}
