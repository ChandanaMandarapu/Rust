fn main (){
    println!("hello");
}

// integer datatypes - signed and unsigned
    // i8 i16 --- i128 u can have both positive and negative numbers stored
    // u8 u16 -- u128 only positive
    // range of a signed integer is limited  
    // use as low range as possible

// floating dataypes - f32 f64

// boolean dataypes  - true or false

// immutable and mutable variables diff 
// variables are by default immutable only so we cant change some values though 
// if u want to modify variable u need to change immutable variable to mutable variable using mut keyword

fn test_func(){
    // declaring x as a unit type () unit type is just like void
    let x : () = ();
    println!("{:?}",x);
    let y : i8 = -9;
    println!("{:?}",y);
    let a : f32 = 93.4;
    println!("{:?}",a);
    let z : u8 = a as u8 - 5;
    println!("{:?}",z);

    // subtracting an integer frm floating
    // mathematical operations only applies on same datatypes

    // type coersion
    // we use the keyword as

    let mut iamold : bool = true;
    iamold = false;
    println!(iamold);
    // throws an error immutable videos are never ever can be modified

    let mystr: char = 'A';
    println!("()",mystr);

    let mut first_name :&str = "chandu";
    println!("{}",first_name);
}