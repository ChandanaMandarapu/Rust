// arrays

fn main(){
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("number array: {:?}", numbers);

    let fruits: [&str;3] = ["apple","banana","orange"];

    println!("fruits array : {:?}",fruits);
    println!("fruits array : {}",fruits[0]);
    println!("fruits array : {}",fruits[1]);
    println!("fruits array : {}",fruits[2]);

    // tuples
    // converting string slice to string

    let human: (String,i32,bool) = ("Sia".to_string(),30,false);
    println!("Human tuple : {:?}",human);

    // mixed tuple we can include any type of dataype in tupil

    let my_mix_tuple = ("alex",23,true,[1,2,3,4,5]);
    println!("my mix tuple : {:?}",my_mix_tuple);

    // 3rd compound dataype - slice
    // slice is simply dynamically sized into a contigous sequence of elemts wtf... i didnt understand one thing also lets see

    // slices are very good for memory efficiency means 

    let number_slices:&[i32] = &[1,23,4,5];
    println!("Number slice : {:?}",number_slices);

    let animal_slices:&[&str] = &["lion,tiger,dog"];
    println!("animal_slices: {:?}",animal_slices);

    let book_slices:&[&String] = &[&"Bell Jar".to_string(),&"Stranger".to_string(),&"whitenights".to_string()];

    println!("book_slices: {:?}",book_slices);

    // 4th type diff btw string and string slice u can increase  i mean strings are mutable they are owned string datypes 
    // it will be allocated to heap string objects it can grow and shrink dynamically so the memory allocation happens like that and its very sloww but they are very useful but not everytime

    let mut stone_cold : String = String::from("Heaven,");
    stone_cold.push_str("Yeah!");
    println!("stone cold says: {:?}",stone_cold);

    // B- &str (string slice)
    // string slices are used to reference string literals or sub string objects u dont have to copy same variable blah blah i need to read abt thiss

    let string : String = String::from("Hello, World!");
    let slice: &str = &string;
    println!("slice value:{:?}",slice);
}