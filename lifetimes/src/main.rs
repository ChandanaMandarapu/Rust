// fixing 10 lifetime errors
// this code has multiple bugs and errors its just for practice purposes ok da
// Error 1 Dangling Reference

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s is dropped here but we are here returning a referne it now this fails y because we are returning a refeernce to data thats abotu to get destroyed

// how to fix - return owned data

fn no_dangle() ->String{
    let s = String::from("hello");
    s // returning string itself no refernce 
}

// Error 2 - reference outlives data
// why this fails x dies at the end of the inner scope and r is taking x as refernce so throws error 
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}",r);
    
    // fixin error 2 

    let x = 9;
    let r = &x;
    println!("{}",r);
}

// Error 3 - Ambiguous Lifetime in function 
// here see this function actually compiles because each parameter gets its own lifetime and ellision rule 2 doesnt apply because multiple inputs 
fn first_word(s:&str, other:&str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
// so this fails as rust cant figure out which input the output is tied to 
fn choose_string(s1:&str, s2:&str,choice:bool) -> &str {
    if choice {
        s1
    } else {
        s2
    }
}

// error 3 fix 

fn choose_string<'a>(s1: &'a str,s2: &'a str,choice:bool) -> &'a str {
    if choice {s1} else {s2}
}

// error 4  struct with dangling reference

struct Wrapper {
    data: &str, // throws error missing lifetime annotation 
}
// quick fix ukk right data: &'a str

// erro 5 method returning refernce to local

// BROKEN:
impl<'a> Wrapper<'a> {
    fn get_owned(&self) -> &String {
        let owned = self.data.to_string();
        &owned  // Returning reference to local variable!
    }
}

// fixing error 5 - returned owned string thats it

impl<'a> Wrapper<'a> {
    fn get_owned(&self) -> String {
        self.data.to_string()  // Return owned String
    }
}

// error 6 mixing lifetimes incorrectly for example

// BROKEN: we declared that y has a diff lifetime b but we are trying to retun it as 'a so thats not ok maa

fn longer<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y  // throws error ma y has lifetime 'b, but we're returning 'a
    }
}

// quick fix 

fn longer<'a>(x:&'astr,y:&'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

// error 7 returning reference from diff sources 
// here one branch returns static data and other returns refernce to local data 
fn get_str(choicce:bool) -> &str{
    if choice{
        "static string"
    } else {
        let s = String::from("dynamic");
        &s[..]
    }
}

// fixin erro 7
// make both of em owned 
fn get_str(choice: bool) -> String {
    if choice {
        "static string".to_string()
    } else {
        String::from("dynamic")
    }
}

// error 8 - mutable and immutable references confilct 

// we learnt first itself we cant have mutable and immutable refernces at the same time 
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // can't borrow as mutable while borrowed as immutable
    println!("{}, {}, {}", r1, r2, r3);
}

// so fixing this is easy printout the r1 and r2 so they no longer are used anywhere u can acces r3 then so whats happening behind scenses rust uses non lexical lifetimes NLL a references lifetime ends at its last use not at the end of scocpe 

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    
    
    let r3 = &mut s;
    println!("{}", r3);
}

// last error lifetime parameter on impl block
// throws error quick fix - 'a annotation 

struct Parser<'a> {
    data: &'a str,
}
impl<'a> Parser <'a> {
    fn parse(&self) -> &'a str {
        self.data 
    }
}