fn main () {
    let s1 = give_ownership();
    println!("s1 :{}",s1);

    let s2 = String::from("Hello from main");
    let s3 = take_and_give_ownership(s2);
    println!("s3 : {}",s3);
}

// func to give owenrship of a string to another function 
fn give_ownership -> String {
    let some_string = String::from("Hello from give_ownership",some_string);
}

// function to take and return ownership of a string

fn take_and_give_ownership(some_string : String) -> String {
    some_string
}

fn caluclate_length (s:String) -> (String,usize){
    let length = s.len();
    (s,length)
}
