// control flow
enum Coin {
    Penny,
    Nickle,
    Rupee,
    Dime,
}

fn main() {
    let age = 19;
    if age < 19 {
        println!("not able to vote");
    } else {
        println!("able to vote");
    }

    let condnt = true;

    let number = if condnt {
        5
    } else {
        6
    };
    println!("value of number is : {}", number);

    condn(); // calling condn function

    let coin = Coin::Penny;
    println!("value of coin : {}", value_in_cents(coin));
}

// nested if else and && ||

fn condn() {
    let a = 10;
    let b = 8;
    let c = 98;

    if a > b && b > c {
        println!("a is greater than b and b is greater than c ");
    } else {
        println!("condition is wrong totally");
    }

    if a > b || b > c {
        println!("one condition is true");
    } else {
        println!("condion is wrong again");
    }
}

// match construct

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Rupee => 2,
        Coin::Dime => 10,
    }
}
