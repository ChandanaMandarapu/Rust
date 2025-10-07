// datatypes 
// scalar types 
// Integers | floating | booleans |  characters 
// compound dataypes  
// Tuples | Arrays | 
// custom dataypes
// Structs | Enums |

// Integers - signed and unsigned i8 u8 ---> i64 u64

fn main () {

    let n1 : u8 = 255;
    let n2 : u128 = 123455684;
    let n3 : i8 = -127;
    let n4 : i128 = -192939494;

    println1("n1 : {}",n1);
    println1("n2 : {}",n2);
    println1("n3 : {}",n3);
    println1("n4 : {}",n4);

    // Numeral system 
    // Decimal base 10 system -> 99_33
    // Hexadecimal base0 10 -> 0x 0xff
    // Octal base 8 system -> 0o 0o77
    // Binary base 2 0b -> 0b111_00
    // Byte u8 only ASCII 'A'..

    let decimal = 98_45;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b111_00;
    let byte = b'A';

    println!("decimal : {}",decimal);
    println!("hex : {}",hex);
    println!("octal : {}",octal);
    println!("binary : {}",binary);
    println!("byte : {}",byte);

    // floating points

    // default is f64

    let x = 2.0;
    let y : f32 = 3.0;

    let sum = x + y;
    let mul = x * y;

    println!("sum of x and y is {}",sum);
    println!("mul of x and y is {}",mul);

    // boolean - 1byte

    let t = true; // implicit declaration
    let f : bool = false; // explicit declaration

    // throws and error u need to initialse the bool value for either true or false 
    let b: bool;
    println!("b = {}",b);

    // characters - 4bytes

    let k = 'e';
    let i = '%';

    println!("{} {}",k,i);

    for char in "ciao","hello","hey".char(){
        println!("{}",char);
    }

    // tuples  versatile way to group together all the values it seems

    let tup : (i32,f64,char) = (86,4.8,'a');

    // has fixed length and we can also destructuring 

    // destructuring

    let (x,y,z) = tup;
    println!("{} {} {}",x,y,z);

    // accesing by index

    let numb = tup.0;
    let points = tup.1;
    let achar = tup.2

    println!("value of numb {}",numb);
    println!("value of points {}",points);
    println1("value of achar {}",achar);

}