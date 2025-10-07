fn main () {
    let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10{
    //         break counter * 2;
    //     }
    // };
    // println!("the result is {}",result)

    while counter ! = 0 {
        println!("{}!",counter);

        counter += 1;
        std::thread::sleep(std::Duration::from_secs(1));
    }

    println!("yay");

    let a = [1,2,4,5,6];

    for element in a.iter(){
        println!("value is {}", element);
    }

    let s = "chandu mndrpu";

    for c in s.chars(){
        println!("value is {}",c);
    }
}