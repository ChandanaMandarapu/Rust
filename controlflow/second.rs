// loops
// the loop while and for loop
fn main (){
    // loop keyword runs until u say it to stop

    // loop {
    //     println!("hello");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {result}")

    // loop labless

    
}