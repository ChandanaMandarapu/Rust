// life time annotations in structs
// here the struct holds a refernce to a string slice and that reference has liftime of 'a 
struct ImportantExcerpt <'a>{part: &'a str,}

fn main() {
    let book = String::from("srimad. bhagvatam"),
    let first_sentence = book.split('.').next().expect("could not find '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Excerpt : {}",excerpt.part);
}
// here book lives for entire main fun so the refernce first_Sentence is valdie and also the excerpt is valid 

// heres a thing which will not work see

/*fn main() {
    let excerpt;
    
    {
        let novel = String::from("srimad . bhagvadam");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        
        excerpt = ImportantExcerpt {
            part: first_sentence,
    };
    }
    
    println!("Excerpt: {}", excerpt.part);  // ERROR!
} */
// Why? Because novel dies at the end of the inner scope, which means first_sentence (which is a reference to part of novel) becomes invalid, 

// which means excerpt.part is now a dangling reference like a dangling pointer in spp . Rust catches this at compile time.

// The struct lifetime parameter 'a means: "This struct is only valid for as long as the data it references is valid."

// things getting complex here

// multiple lifetime parameters
// first and second can have diff lifetimes 
struct Container <'a,'b>{
    first: &'a str,
    second: &'b str,
}
// using above lifetime scenario here

fn main () {
    let long_lived = String::from("I live long");

    {
        let short_lived = String::from("I die soon");

        let container = Container {
            first: &long_lived,
            second: &short_lived,
        };

        println!("{} and {}",container.first,container.last);
    }

    // long lived is still accesiblr here but not short lived as said it ends in that scope itseslf

    // the 'static lifetime - wahh this is my fav this lives for whole entire program i was thinking what if smtng needs to live until very end 

    // just got to know strin literals are static because they are within in ur programs binary they exist before main runs and after main() ends

    // let s : &'static str = "lives forever";

    static GREETING: &str = "Hello, world!";  // A global constant

    fn get_greeting() -> &'static str {
    GREETING
    }
}