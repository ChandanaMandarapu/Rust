// datatypes

fn main (){
    let mut x = 5;
    println!("the value of x is : {}",x);

    // even if we use let keyword variables are by default immutable in rust to make it mutable use mut keyword

    // shadowing - we can redefine the same variable with same name in same scope in rust so its intersting

    // inner scope

    {
        let x = 10;
        println!("the value of x is : {}",x);

    }

    let x = x+3;
    println!("the value of x is : {}",x);

    x = "hello"; 
    println!("the value of x is : {}",x);

    // if i define a variable as a mut one then we can change the value but not the type that means x here should be indeed an integer only so yeah trickyy

    // constants  We need to definetly define the typeof variable while using const
    // also declaring and initialised at the same time in rust

    // also shadowing doesnt work in const 

    const MAX_POINTS : u32 = 100_00;
    println!("the value of MAX_POINTS is : {}",MAX_POINTS);


}