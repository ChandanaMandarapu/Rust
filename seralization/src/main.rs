// a simple defn to understand seralization is its a procces of converting ur whole programs data into a format that can be saved or transmitted deserlization is opposit  like for example if u want to save ur data to a file to a databse across a network to another computer but files and networks doesnt udnerstand rust structs they can only understand bytes right so thats when seralisation comes in mann

// some forms of seralization 
// JSON | Binary Formats | XML | YML 

// adding serde to project serde is (serialise + deserialise ) its a sperate crate library that becomes the standard way to handle serialisation 

//  we have added this to our cargo.tml under dependencies section soo lets see

// serde = { version = "1.0", features = ["derive"] } - we are adding the serde crate also enabling derive feature this features lets us automatically generates serialisation code using rust derive macros
// serde_json = "1.0"  this is a sperate crate that implements JSON serialisation using serdes framework 

// basic serialisation example

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]

struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let person = Person{
        name: String::from("rama"),
        age: 21,
        email: String::from("rama@gmail.com");
    };

    // serialise to JSON string

    let json = serde_json::to_string(&person)
            .expect("failed to serialise");

    println!("JSON : {}",json);

    // Deserialize back to struct
    let person2: Person = serde_json::from_str(&json)
        .expect("Failed to deserialize");
    
    println!("Deserialized: {:?}", person2);

}