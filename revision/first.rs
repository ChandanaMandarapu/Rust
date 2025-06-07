fn main () {
    // we can redefine a variable in rust in same scope manytimes 
    let x = 5;
    println!("the value of x is {}",x);
    // adding inner scope will see how shadowing works this x works here in itself
    {
        let x = 8;
        println!("the value of x is {}",x);
    }
    let x = x + 2;
    println!("the value of x is {}",x);

    // lets try adding a string in x of outerscope and it will work perfectly

    let x = "helloworld";
    println!("the value of x is {}",x);

    // imp note here if we declare a variable as mutable then here the value can be changed but not the type unless u use shadowing rust is strongly typed lang u can mutate value but not the type

    let mut y = 6;
    println!("the value of y is {}",y);

    y = "chandu";
    println!("the value of y is {}",y);

    // uses of constant yes it will stay constant
    // u need to declare and assign the value and type at the same time

    // // shadowing with const
    // throws an error u cant use shadowing

    // heres a thing u cant even shadow the const if u add mut keyword then also u cant

    const ROLL_NUM : i32 = 100_3;
    println!("the value of max points is {}",ROLL_NUM);

    const ROLL_NUM : i32 = 99;
    println!("the value of max points is {}",ROLL_NUM);
}