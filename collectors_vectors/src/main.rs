fn main() {
    // diff type of collections
    // vec - vector like a organised boook u can and remove from the end easily
    // hashmap - keys and values
    // hashset = list of uniqque items 

    // first collecttion

    // Vce<T>  vector a dynamic array and <T> this is a generic type anymeter this parameter can hold anything either int string bool anythin literally

    // creating vectors

    println!("creating vectors - 4ways ===\n");

    // way 1 Vec::new() - the escplicit way
    let mut numbers: Vec<i32> = Vec::new();
    println!("empty vector : {:?}",numbers); // gives an empty vector 

    // way2 of vec!macro - easy way macro is like a code generator vec![1,2,3]

    let fruits = vec!["apple","banana","mango"];
    // rust will consider this as &str vec<&str> borrow string data
    println!("fruits : {:?}",fruits);

    // way 3 - vector with capacity

    let mut scores : Vec<i32> = Vec::with_capacity(100);
    println!("capacity : {},length: {}",scores.capacity(),scores.len());

    scores.push(99);
    scores.push(87);
    scores.push(98);

    println!("after adding : {:?}",scores);
    println!("capacity: {},length:{}",scores.capacity(),scores.len());

    // way4 range vec

    let range_vec : Vec<i32> = (1..6).collect(); // creates a range from 1 to 5 (6 is excluded) .collect() execute code and collect results into a vec

    println!("Range vector: {:?}",range_vec);

    // inclusive range
    let inclusive_range: Vec<i32> = (1..=5).collect();
    println!("inclusive range : {:?}",inclusive_range);

    // capacity vs length

    let mut example = Vec::with_capacity(10);
    example.push(9);
    example.push(2);

    println!("length: {}",example.len()); //2
    println!("length: {}",example.capacity()); // 10

    let mut mutable_vec = vec![1,2,3];
    mutable_vec.push(8);
    println!("mutable vector : {:?}",mutable_vec);

    let mut shopping_cart = Vec::new();
    shopping_cart.push("dal");
    shopping_cart.push("curd");
    shopping_cart.push("millk");

    println!("shopping cart :{:?}",shopping_cart);

    let mut nums = vec![10,30,40,50,60];
    let last = nums.pop();
    println!("popped value : {:?}",last);
    println!("after pop : {:?}",nums);

    let mut empty_vec : Vec<i32> = Vec::new();
    let nothing = empty_vec.pop();
    println!("Pop from empty vector: {:?}", nothing);

    // option values

    let mut points = vec![87,93,78];
    let popped = points.pop();
    match popped{
        Some(point){
            println!("you got a point of {}",point);
        }
        None => {
            println!("no point to pop");
        }
    }

    if let Some(point) = points.pop() {
        println!("another point: {}",point);
    } else {
        println!("no more points");
    }

    points.push(100);
    let point = points.pop().unwrap();

    // accessing elements

    let colors = vec!["Red","blue","white","green"];
    let first_color = colors[0];
    println!("first color : {}",first_color);
    let third_color = colors[3];
    println!("second color : {}",third_color);

    // using get()

    let maybe_color = colors.get(2);
    println!("thirdcolor (safe) : {:?}",maybe_color);

    let out_of_bounds = colors.get(10);
    println!("invalid index : {:?}",out_of_bounds);

    match colors.get(1){
        Some(color) => println!("found color : {}",color),
        None => prinln!("index doesnt exist");
    }

    // other operations

    let mut data = vec![5,10,15,20,25];

    println!("length : {}",data.len());
    println!("is empty ? {}",data.is_empty());
    // data.clear() clears everything
    println!("after clear :{:?}",data);
    println!("is empty now ? {}",data.is_empty());

    // insert options

    let mut sequence = vec![1,2,4,5];
    println!("before insert: {:?}",sequence);
    sequence.insert(2,3);
    println!("after insert :{:?}",sequence);

    // contains check

    println!("contains 30 {}",nums.contains(&30));
    println!("contains 10 ? {}",nums.contains(&10));

    // first last and empty 

    let values = vec![100,200,300,400];
    let f1 = values.first();
    println!("first : {:?}",f1);
    let f2 = values.last();
    println!("first : {:?}",f2);
    let empty: Vec<i32> = Vec::new();
    println!("First of empty: {:?}", empty.first()); 
}

