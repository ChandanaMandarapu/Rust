// shadowing

// wwhat in the world is shadowing
// u can create a new varaible with the same name as a previous variable
// first variable is shadowed by second one
// shadowing is diff marking a varaible as mutable how so simply 

fn main (){
    let x = 4;
    // x is shadowed by 1st 
    // compiler now considers the second one varaible onlyy
    let x = x + 1;

    {
        // inner scope
        let x = x * 2;
        println!("the value of x in the inner scope is : {x}")
    }

    println!("the value of x is :{x}");

}