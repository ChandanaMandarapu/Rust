fn main() {
    // A REFERENCE is ntg but it lets u to refer a value without taking ownership

    let s1 = String::from("hello");
    // here what happens is when u use refernce u just keep the ownership to s1 only but
    let len = caluclate_length(&s1); // refering s1 here
    println!("the length of {} is {}",s1,len); // s1 is still valid here 

    // the & AND and * Operators
    // & creates a refernce borrows the value * dereferences follows the references to the actual value

    let x = 4;
    let r = &x; // r is referncing to x borrowed 

    println!("x is {}",x);
    println!("r is {}",r);
    println!("*r is {}",*r);  // explicitly derefencing

    //println!("{}",r+1); //error
    println!("{}",*r+1); // works

    // two types of borrows
    // immutable borrow &T can read cant modify
    // mutable borrow &mut T can read nd modify
    // rules of borrowing - one mutable reference and multiple number of immutable refernces

    // multiple immutable references hoo the main trick of borowing lies here r is an immutable reference here can read as many as u can but cant change it 
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s3;

    // but here while we change that will obviously throw an error because u can make multiple refernces but cant change it 

    let r4= &s1;
    // r.push_str("world") error

    println!("{}, {}, {}",r1,r2,r3);

    // mutable refernces

    let mut s = String::from("hello"); 
    change(&mut s);
    println!("{}",s);

    // MIXING IMMUTABLE AND MUTABLE REFERENES
    // irony u cant have immutable and mutable references at same time

    // reference scope (lifetime)

    let mut c = String::from("chandu");

    let d1 = &c;
    let d2 = &c;
    println!("{} {}",d1 d2);

    //  d1 and d2 are no longer used after this point

    let d3 = &mut c;
    println!("{}",d3); // this is ok d3 can be refences as d1 and d2 are no longer used

    let a = 9;
    let b = *a;

    // let n = b + 1; cant add reference to integer

    // first dereference

    let n = *b + 1;
    println!("n : {}",n);

    // modifying through mutable reference 

    let mut k = 9;
    let l = &mut k;

    *k += 1;
    println!("k is {}",x);

    // references to references

    let m = 9;
    let m1 = &m;
    let m2 = &m1;
    let m3 = &m2;

    println!("m {}",m);
    println!("m1 {}",m1);
    println!("m2 {}",m2);

    // borrowing with immutable functionss

    let name = String::from("ram");

    print_string(&name);
    print_string(&name); // can borrow multiple times

    println!("still have s : {}",name);

    // borrowing with mutable where data can be modified

    let mut mata_name = String::from("sita");

    add_prabhuname(&mut mata_name);
    println!("{}",mata_name);

    // borrowing with diff scopees

    {
        let r = &mata_name //immutable borrow
        println!("inner: {}",r);
    }

    let j2 = &mut mata_name;
    j2.push_str("rama");
    println!("outer {}",j2);


    // borrowing with loops

    let mut digits = vec![1,2,3,4];

    for n in &digits {
        println!("{}",n);
    }
    for n in &mut digits {
        // defferencing and modifying data
        *n *= 2;
    }

    println!("{:?}",numbers);

}

fn caluclate_length(s: &String) -> usize {
    // the s value here points out to s1 but doesnt own like refering to it so what happens is function borrows the data it can use it but when the func ends ownership stays with orginal variable only woww
    s.len()
}
 
fn change(s : &mut String) {
    s.push_str(",world");
}
fn print_string (name : &String){
    println!("Borrowed string : {}",name);
}

fn add_prabhuname(s : &mut String) {
    s.push_str("ram");
}