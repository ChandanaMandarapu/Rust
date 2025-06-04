fn main(){
    hello_world();
    tell_height(150);
    human_id("chandu", 21, 150.0);

    let _X: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        // any expression in a mathematical value will evaluate to the last line in that expression 
        price * qty
    };

    println!("Result is {}",_X);

    add ( 3,5);
    sub ( 9,6);
    mul (9,8);
}



fn hello_world(){
    println!("hello,rust");
}

fn tell_height(height: i32){
    println!("my heaight is; {}",height);
}

fn human_id(name: &str, age:u32, height: f32){
    println!("my name is  {}, Iam {} years old, and my height is {} cm",name, age, height);
}

// expressions and statements

// expressions : anything that returns a value
// statement: anything that doest return a value

// expression 
// ---------- 
// 5
// true & false
// add (3,4)
// if else

// functions returning values 

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(c: i32, d: i32) -> i32 {
    c - d
}

fn mul(a: i32, b:i32) -> i32{
    a*b
}

// statements
