fn main ( ){

    // int

    let small_numb: u8 = 233;
    let big_numb : u128 = 9999;
    let small_num2: i8 = -233;
    let big_num2 : i128 = -9999;

    println!("small number :{} ", small_numb);
    println!("big number :{} ", small_num2);
    println!("small number :{} ", big_numb);
    println!("big number :{} ", big_num2);

    // float 

    let x = 2.0;
    let y : f32 = 3.0;

    println!("x = {}, y = {}",x,y);

    let sum = x + y;
    let sub = x - y;
    let mul = x * y;
    let div = x / y;

    println!("sum is {}",sum);
    println!("sub is {}",sub);
    println!("mul is {}",mul);
    println!("div is {}",div);

    // booleans

    let t = true; // implicit declaration

    let f : bool = false;

    println!("value of t is {}",t);
    println!("value of f is {}",f);

    if t {
        println!("t is true");
    } else {
        println!("f is true");
    }

    let not_t = !t;
    println!("not t value is {}",not_t);

    // boolean should be initialsied

    // characters

    let C = "z";
    let X = "y";

    // tuples - group a number of values

    let tup : {i32,f64,char} = {500,3.4,'x'};

    // accesing elements from a tuple

    let five_hundred = tup.0;
    let pi = tup.1;
    
    // easy peasy just like arrays

    println!("value of five_hundred is {}",five_hundred);
    println!("value of pi is {}",pi);

}