const SPEED_OF_LIGHT: u32 = 299792458;  // Known at compile time

fn main() {
    let x = 9;
    println!("the value of x is : {}",x);
    let mut y = 10;
    println!("value of y is : {}",y);
    y = 8;
    println!("value of y is is {}",y);

    let mut score = 0;
    println!("initial score is {}",score);
    score += 10;
    println!("after first goal {}",score);
    score += 5;
    println!("final score {}",score);

    const BIRTH_YEAR : u32 = 2004;
    println!("and the birth year is {}",BIRTH_YEAR);

    // edgecases of constant

    // let user_input = "42";  // this comes from user
    // let number = user_input.parse::<i32>().unwrap();  // Computed at runtime
    
    // This would be WRONG:
    // const USER_NUMBER: i32 = user_input.parse().unwrap();
    // Constants can't depend on runtime values!

    // ----SHADOWING -----

    let s = 7;
    println!("s is {}",s);
    let s = s + 9;
    println!("s is {}",s);
    let s = s * 8;
    println!("s is {}",s);
    let s = "ram";
    println!("s is {}",s);

    // shadowing is amazing 

    // Im not CHANGING the original s. Im creating a NEW variable that SHADOWS (hides) the old one. The old s still exists in memory, but you can't access it anymore because the new s is in the way. so the new value of s is ntg but ram ..

    
    let spaces = "   ";           // spaces is a string
    let spaces = spaces.len();    // spaces is now a number
    
    println!("Number of spaces: {}", spaces);


    // edge cases with mut 

    let a = 9;
    let a = "chandu"; // we can change the type in shadowing

    let mut b = 8;
    b = b * 8;
    // b = "ball"; this doesnt work mutables cant change the type

    let p: i8 = 127;      // 8-bit signed
    let q: u8 = 255;      // 8-bit unsigned
    let r: i32 = -50000;  // 32-bit signed (default)
    let t: u32 = 100000;  // 32-bit unsigned

    // Signed (i): Can be negative or positive. Uses one bit for the sign.
// Unsigned (u): Only positive (and zero). No sign bit means bigger positive range.

// Integer literals

let decimal = 98_222;      // Underscore for readability
    let hex = 0xff;            // Hexadecimal
    let octal = 0o77;          // Octal
    let binary = 0b1111_0000;  // Binary
    let byte = b'A';           // Byte (u8 only)

    // FLOATING 

    let h = 2.4; //f64 by default
    let i : f32 = 9.18; 

    // Always use f64 unless you have a specific reason (like graphics programming where you need speed over precision).

    let result = 0.1 + 0.2;
    println!("0.1 + 0.2 = {}", result);

    // BOOLEANS

    let is_raining: bool = true;
    let is_sunny = false;  // Type inferred
    
    println!("Is it raining? {}", is_raining);

    // characters

    let letter: char = "m";
    let emoji: char = '😊';


    // TYPE CHANGINGGG

    // Rust looks at how you USE the variable to figure out the type so

    let guess = "42".parse().expect("Not a number!");
    // Error! Parse into what type?
    
    let guess: u32 = "42".parse().expect("Not a number!");
    // Works! Now Rust knows.


    let player_name = "hari";
    let player_health : u32 = 100;
    let player_mana : f32 = 50.9;
    let is_alive : bool = true;
    let rank : char = 'S';
    
    println!("Name: {}", player_name);
    println!("Health: {}", player_health);
    println!("Mana: {}", player_mana);
    println!("Alive: {}", is_alive);
    println!("Rank: {}", rank);


}
