// functions

fn main (){
    println!("helloworld");
    another_func(42,'a' );
    let x = sum_diff(9,8);
    println!("the sum and diff is {:?}",x);
    // used to print the tuple {:?}
}
// need to specify the type of parameters
fn another_func (num: i32,letter : char) {
    println!("value of num is {}",num,letter);
}

// statements and expressions

// statements are instruction performs action doesnt return an value but expression return a value so in rust funcns are expressions

fn sum_diff (num1:i32,num2:i32) -> (i32,i32) {
    return (num1 + num2, num1-num2);
    println!("this will be not printed");
}
