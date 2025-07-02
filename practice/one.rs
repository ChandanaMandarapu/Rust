// write a function is_even that takes a number as an input and returns true if it is even

// funcn to check its odd

fn main (){
    println!("{}",is_even(6));
    println!("{}",fib(10));
    let name = String::from("Chandana");
    let len = get_string_length(s:name);
    println!("the length of string is {}",len)
}

fn is_even(num:u32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

// fibannocci of a number 

fn fib (num : i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if ( num == 0) {
        return first;
    }

    for i in 1..num - 2{
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}


// func that takes a string as an input and returns its length

fn get_string_length(s : String) -> usize {
    s.chars().count()
}
