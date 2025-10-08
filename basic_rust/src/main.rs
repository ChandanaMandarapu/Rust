mod funcns;
mod controlflow;
// const SPEED_OF_LIGHT: u32 = 299792458;  // Known at compile time

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

    let letter: char = 'm';
    let emoji: char = '😊';


    // TYPE CHANGINGGG

    // Rust looks at how you USE the variable to figure out the type so

    // let guess = "42".parse().expect("Not a number!");
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


    funcns::say_hello();
    funcns::say_hello();
    funcns::say_hello();

    funcns::greet_person("ram");
    funcns::greet_person("sri");
    funcns::greet_person("hari");

    funcns::add(8,9);

    let result = funcns::sub(9, 7);         
    println!("Subtraction is {}", result);

    let product = funcns::mul(9, 6);       
    println!("Product is {}", product);

    // diff btw statements and expressions

    let n = 9; // this is a statement where a value is not produced 
    println!("chandu"); // this is a statement where a value is not produced 

    // expressions produce a value
    // thse are expressions that produce a value man
    // 8 + 8;
    // if x > 5 {10} else {20}

    // early returns

    // fn divide(a: i32, b: i32) -> i32 {
    // if b == 0 {
    //     println!("Can't divide by zero!");
    //     return 0;  // Exit early
    // }
    // a / b  // Normal return (no semicolon)


// THE UNIT TYPE: When Functions Return Nothing
// secretly, this returns () - the "unit type". It's like "void" in other languages, but it's an actual type in Rust. You could write:
//     fn say_hello() -> () {
//     println!("hello");
// }

// control flow

let age = 9;

if age < 18 {
    println!("Age is less than 18");
} else {
    println!("age is greater");
}

controlflow::check_temp(27);
controlflow::check_temp(39);

// also if is an expression

let condition = true;
let numb = if condition {3} else {5};
println!("number is {}",numb);

controlflow::produce_value(9);

controlflow::check_grade(89);


// loopss


// loop{
//     println!("loop prints forever");
// }

let mut counts = 0;

loop {
    counts += 1;
    println!("Counts:{}",counts);

    if counts == 5{
        break;
    }
}
println!("loop finished");

let mut counter = 0;

loop {
    counter += 1;

    if counter % 2 == 0{
        continue;
    }
    println!("odd number: {}",counter);

    if counter >= 9{
        break;
    }
}

let loop_result = controlflow::double_counter_loop();
println!("the result is {}", loop_result);

let attempt_loop_result = controlflow::attempt_loop();
println!("result :{}",attempt_loop_result);

let nested_loop_result = controlflow::nested_loop();
println!("finished",nested_loop_result);


// while conditions

let mut numerle = 3;

while numerle != 0{
    println!("{}",number);
    numerle -= 1;
}
println!("liftoff");

let mut attempts = 0;
    let max_attempts = 5;
    
    while attempts < max_attempts {
        println!("Attempt {} of {}", attempts + 1, max_attempts);
        attempts += 1;
        
        // we're checking something
        if attempts == 3 {
            println!("Success!");
            break;
        }
    }
    
    if attempts == max_attempts {
        println!("Out of attempts!");
    }

    // for loops

    for number 1..6 {
        println!("number:{}",number);
    }

    // reverse iteration using for loops

    for numb in (1..6).rev() {
        println!("{}",number);
    }
    println!("liftoff");

    // for loops with arrays

    let birds = ["eagle","pigeon","parrot"];

    for bird in birds {
        println!("bird:{}",bird);
    }

    // -------MATCH---------


    let numeric = 9;

    match numeric {
        1 => println!("one");
        2 => println!("two");
        _=> println("three");
    }

    // match multiple values

    let result = match number {
        1 | 2 => println!("one or two");
        3 |4 | 5 => println!("three to five");
        1 ..=5 => number*2;
        _=> println!("something else");
    }

    // matching ranges

    match age {
        0..=12 => println!("child");
        13..=19 => println!("teenager");
        20..=64 => println!("adults");
        _ => println!("Senior");
    }

    // matching an expression

    let k = 9;
    let description = match k {
        1 => "one";
        2 => "two";
        3 => "three";
        _ => "something else",
    };

    println!("number is {}",description);

    // matching with guards here a acts as guards

    let ageee = 25;
    let is_student = true;

    match ageee{
        a if a < 18 => println!("minor");
        a if a >= 18 => println!("major");
        a if a >=65 => println!("senior");
        _ => println!("regular human  being");
    }

}
