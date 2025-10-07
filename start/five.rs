// functions in rust

fn another_function(num: i32){
    println!("value of num is : {}",num);
}
// return values
// for early return u can add return keyword

fn sum_diff(num1:i32, num2:i32) -> (i32,i32) {
    (num1 + num2 , num1 - num2);
}

fn main () {
    println!("hello");
    another_function(89);

    // expressions

    let x = 9;
    let y = 9;
    let z = x + y;
    println!("value of z is {}",z);

    let c = {
        let f = 9;
        f + 1
    };
    println!("value of f is {}",c);

    let v = sum_diff(9,5);
    println!("sum and diff is {:?}",x);
    // {:?} is used to print the tuple

}

// statements and expressions
// statements are instructions perform some action but they dont return a value simple
// expressions evaluate and return a value
// in rust funcns are expressions

