fn main() {
    // in rust match retruns expressions not just a statement

    let number = 9;
    let description = match number {
        1 => "one",
        2 => "two",
        9 => "nine",
        _ => "many",
    };
    println!("number is {}",description)
}

// in return statemnts

// fn number_to_word(n: i32) -> String {
//     match n {
//         1 => String::from("one"),
//         2 => String::from("two"),
//         3 => String::from("three"),
//         _ => format!("number {}", n),
//     }
// }

// Match guards - adding conditions

fn describe_number(n:i32){
    match n{
        x if x < 0 => println!("{} is negative",x);
        x if x > 0 && x < 10 => println!("{} is a small positive number", x),
        x if x >= 10 && x < 100 => println!("{} is a medium number", x),
        x if x >= 100 => println!("{} is a large number", x),
        _ => println!("Zero"),
    }
}

// Matching enums - Destructing Data

enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String);
    ChangeColor(u8,u8,u8);
}

fn proccess_message(msg:Message) {
    match msg{
        Message::Quit => {
            println!("quitting");
        }
        Message::Move ( x, y) {
            println!("moving to coordinate ({} , {} )",x,y);
        }
        Message::Write(text) => {
            println!("writting message : {}",text);
        }
        Message::ChangeColor(r ,g,b) =>{
            println!("changing color to RGB ({}, {},{})",r,g,b);
        }
    }
}
