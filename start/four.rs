// arrays and enums

// structs
struct Person {
    name: String,
    age: u8,
}

enum Zebra {
    black,
    white,
    stripes,
}

fn main() {
    // arrays - same dataypes
    let arr = [1, 2, 45, 6];
    let first = arr[0];
    println!("first : {}", first);

    for element in arr.iter() {
        println!("element : {}", element);
    }

    // using custom dataypes structs

    let person = Person {
        name: String::from("Abhi"),
        age: 24,
    };
    println!("person name is {} and age is {}", person.name, person.age);

    // enums

    let color = Zebra::black;

    match color {
        Zebra::black => println!("WOW"),
        Zebra::white => println!("HEHE"),
        Zebra::stripes => println!("ZEBRA STRIPES!"), // needed to match all variants
    }
}
