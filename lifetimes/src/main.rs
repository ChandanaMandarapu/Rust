
    // most imp topic of rust - lifetimes
    // life time definition lies in the name itself a lifetime how long a pieve of data is valid for every reference in rust has a lifetime which is the scope for which reference is valid lifetime is not about how long a value lives they're about how long a REFERENCE to that value is valid

    /* fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+ */
    // lifetime annotations (syntax explained)

    // this wont compile the error will say something x does nor live long enough so whats happening here 
    // we declared r but didnt initialise it yet
    // we enter a new scope {}
    // inside the scope we create x with value 6
    // we make r point to x 
    // scope ends and x is destroyed then nd their itself
    // we try to use r but r points to dead memory which is like a dangling pointer in cpp 

    // when rust cant figure out lifetimes automatically u have to annotate them 
    // whats happening
    // <'a> - declares a lifetime parameter named 'a 
    // x :&'a str - x is a refernce to string slice that lives for lifetime 'a
    // y : &'a str - y is a refernce to string slice that lives for same lifetime 'a
    // -> &'astr - return value is refernce that lives for lifetime 'a again

    // chandu chandu listen its easy - in simple words this below function takes two string slices that both live for same lifetime 'a and returns a string slice that lives for that same lifetime 'a as simple as that


    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
        if x.len() > y.len() {
            x 
        } else {
            y
        }
    }


    fn main () {
        let string1 = String::from("long string is long"); 
        let string2 = String::from("xyz");

        let result = lonest(string1.as_str(),string2.as_str());
        println!("longest string is {}",result);
    }


    // what if lifetimes are diff
    // string2 dies at teh end of innerscope but here we are asking result as result might have a refernce to string 2 rust wont at all allow it throws error 
    /*fn main() {
    let string1 = String::from("long string is long");
    let result;
    
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    
    println!("The longest string is {}", result);  // ERROR!
}*/

// life time ellision rules - when u dont neeed to annotations

// rule 1 - each refernce parameter gets its own lifetime parameter 

fn fool(x:&str) -> &str which is automaticaly fn fool<'a>(x:&'astr) -> &str

// rule2 - if theres exactly one input lifetime parameter that lifetime is assigned to all output lifetime parameters what does that mean

fn foo(x:&str) -> &str which becomess fn foo<'a>(x:&'a str) -> &'a str = see the subtle difference lifetime of a parameter will be assigned to al outcomes 

// rule 3 - if there are multiple input lifetime parameters but one of them is &self or &mut self = because its a method lifetime of self is assigned to all output lifetime parameters

impl MyStruct{
    fn get_data(&self) -> &str{
        &self.data
    }
}
// becomes 

impl MyStuct{
    fn get_data<'a>(&'a self) -> &'a str{
        &self.data
    }
}

// lifetime annotations in structs

struct ImportantExcerpt<'a> {
    part: &'a str,
}
this means struct holds a refernce to a string slice and that refernce has lifetime 'a the struct cannot outlive the data its referencing 

fn main () {
    let book = String::from(" SriMad Bhagvadgita..");
    let first_sentence = book.split('.').next().expect("could not find a '.'");

    let excerpt = ImportantExcerpt{
        part:first_sentence,
    };
    println!("Excerpt {}",excerpt.part);
}