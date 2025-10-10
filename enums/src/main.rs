// creates a new type called attackresult A variable of type AttackResult can ONLY be one of these four variants. Not two at once. Not something else. Just one.

enum AttackResult{
    Hit(i32),
    Miss,
    Critical(i32),
    Blocked(String),
}
// accessing
let outcome = AttackResult::Hit(23);
let outcome2 = AttackResult::Critical(40);
let outcome3 = AttackResult::Blocked(String::from("shielf"));

// Oficially a Null in rust 

enum Option<T> {
    Some(T),
    None,
}
// example of option<T>

fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

let result = find_user(5);
// You CANNOT just use result as a String
// The compiler won't let you

// rust error handling foundation
// Result<T, E> - Two generic type parameters:

// T is the type of the success value
// E is the type of the error value

// Ok(T) - The operation succeeded, here's the result
// Err(E) - The operation failed, here's the error
// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

fn divide(a:f64, b:f64) -> Result<f64,String>{
    if b = 0.0{
        Err(String::from("cant divide zero"));
    } else{
        Ok(a/b)
    }
}

let result2 = divide(10.0,0.0){
    match result2{
        Ok(value) => println!("Result : {}",value);
        Err(error) => println!("Error: {}",error);
    }
}

match GameState{
    MainMenu,
    Playing,
    Paused,
    Gameover,
}

fn handle_state(state:GameState){
    match state{
        GameState::MainMenu => {
            println!("showing main menu");
        }
        GameState::Playing => {
            println!("Game is running");
        }
        GameState::Paused =>{
            println!("game is paused");
        }
    }
}


enum PaymentMethod {
    Cash,
    CreditCard{
        number:String,
        expiry: String,
        cvv: String,
    },
    PayPal{
        email:String,
    }
    CryptoCurrency{
        wallet_address:String,
        currency:String,
    },
}
fn proccess_payment(method:PaymentMethod,amount:f64){
    match method{
        PaymentMethod::Cash=>{
            println!("proccessing rs 300 cash payment",amount);
        }
        PaymentMethod::CreditCard (number,expiry,cvv)=> {
            println!("proccessing rs 400 credi card payment",amount);
            println!("card ending in {} ",&number[number.len()-4..1]);
        }
        PaymentMethod::PayPal {email} => {
            println!("processing rs 400 paypal payment",amount);
            println!("paypal accoung : {}",email);
        }
        PaymentMethod::CryptoCurrency(wallet_address,currency){
            println!("Proccessing{:.8} {} payment",amount,currency);
            println!("wallet: {}", wallet_address);
        }
    }
}
fn main() {
    println!("Hello, world!");
}
