// fizzbuzz prblm is a classic test 
// the task to prin the number from 1 to 100 but for multiples of 3 print fizz instead of number and for multiples of 5 u print buzz instead of number ok 

fn main () {
    for number in 1..=100 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("fizzbuzz");
        } else if number % 3 == 0 {
            println!("fizz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}",number);
        }
    }
}