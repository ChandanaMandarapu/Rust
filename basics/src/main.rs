// datatypes

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}",x);
    println!("UnSigned Integer: {}",y);

    // diff btw i32 (32bits) and i64(64bits)
    // range : i32 -> -2^31 to 2^31
    // range : i64 -> -2^64 to 2^64

    // floats [Floating point types]
    // f32,f64

    let pi: f64 = 3.14;
    println!("value of pi: {}",pi);

    // boolean values :true, false

    let is_snowing: bool = true;
    println!("is it snowing {}", is_snowing);

    // character type - char

    let letter: char = 'a';
    println!("first letter of alphabets is {}", letter);
}
