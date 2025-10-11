use std::collections::{HashSet,VecDeque};

fn main(){
    println!("HASHET - UNIQUE COLLECTION");

    // Hashset - a similar thing to hashmap but it only stores keys no values and no duplicates allowed

    let mut unique_numbers: HashSet<i32> = HashSet::new();

    println!("=== INSERTION ===\n");

    // insert returns bool 

    let was_inserted = unique_numbers.insert(10);
    println!("Inserted 10: {}", was_inserted); 

    unique_numbers.insert(20);
    unique_numbers.insert(30);
    println!("Set: {:?}", unique_numbers);

    // trying inserting duplicate:
    let inserted_again = unique_numbers.insert(10);
    println!("Inserted 10 again: {}", inserted_again);  // false
    println!("Set unchanged: {:?}", unique_numbers);  // Still {10, 20, 30}

    // real world use case - removing duplicate

    let numbers_with_dupes = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    println!("Original: {:?}", numbers_with_dupes);
    
    // converting vec to HashSet - duplicates automatically removed! 
    let unique: HashSet<i32> = numbers_with_dupes.into_iter().collect();
    println!("Unique numbers: {:?}", unique);
    
    // Convert back to Vec if needed:
    let unique_vec: Vec<i32> = unique.into_iter().collect();
    println!("Back to Vec: {:?}", unique_vec);

    // membership checks

    let mut visited_pages: HashSet<&str> = HashSet::new();
    visited_pages.insert("home");
    visited_pages.insert("about");
    visited_pages.insert("contact");

    if visited_pages.contains("home") {
        println!("user visited home page");
    }

    if !visited_pages.contains("products") {
        println!("User hasn't seen products yet");
    }
    
    // len() - number of unique items
    println!("Pages visited: {}", visited_pages.len());
    
    // is_empty() - check if set is empty
    println!("Any visits? {}", !visited_pages.is_empty());

    // practical examples - finding common intersts like hobbies kinda

    let ram_hobbies : HashSet<&str> = 
    vec!["reading","coding","sketchiing","music"].into_iter().collect();
    let krishna_hobbies : HashSet<&str> = 
    vec!["music","playing","reading","sketching","magic"].into_iter().collect();

    println!("ram's hobbies : {:?}",ram_hobbies);
    println!("krishna's  hobbies : {:?}",krishna_hobbies);

    let common : Vec<_> = ram_hobbies.intersection(&krishna_hobbies).collect();
    println!("\nCommon intersts : {:?}",common);

    println!("iteration and removal");

    let mut tags:HashSet<&str> = HashSet::new();
    tags.insert("rust");
    tags.insert("programming");
    tags.insert("tutorial");

    println!("all tags");
    for tag in &tags {
        println!(" #{}",tag);
    }

    // remove an item

    let was_removed = tags.remove("tutorial");
    println!("\nRemoved 'tutorial': {}", was_removed);  // true
    println!("Tags now: {:?}", tags);

    // clears all items
    
    tags.clear();
    println!("after clear : {?}",tags);

    // VECDEQUE 
    // Vec: Only the person at the back can leave/join easily
    // - VecDeque: People can leave/join from BOTH front and back

    println!("VECDEQUE - DOUBLE-ENDED QUEUE");

    let mut queue: VecDeque<&str> = VecDeque::new();

    // adding both end elements

    queue.push_back("First");
    queue.push_back("Second");
    queue.push_back("Third");

    println!("after push_back : {:?}",queue);

    // remove from both ends

    let from_back = queue.pop_back();
    // pop_back
    println!("popped from back : {:?}",from_back);
    // pop_front

    let from_front = queue.pop_front();
    println!("popped from front: {:?}",from_front);
    println!("Queue now : {:?}",queue);

    // practice of vec deque task queue

    let mut task_queue : VecDeque<&str> = VecDeque::new();

    // adding normal prirority tasks to back 

    task_queue.push_back("send email");
    task_queue.push_back("Update database");
    task_queue.push_back("Generate report");

    println!("task queue : {:?}",task_queue);

    task_queue.push_front("URGENT FIX SERVER");

    println!("after urgent task : {:?}",task_queue);

    while let Some(task) = task_queue.pop_front() {
        println!("Processing: {}", task);
    }

    // other operations of vecdeque

    let mut deque: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4, 5]);
    
    // Access front and back
    println!("Front: {:?}", deque.front());  // Some(&1)
    println!("Back: {:?}", deque.back());    // Some(&5)
    
    // Access by index (like Vec)
    println!("Index 2: {:?}", deque.get(2));  // Some(&3)
    println!("Index 2 direct: {}", deque[2]);  // 3
    
    // Insert at arbitrary position (still slower than front/back)
    deque.insert(2, 99);
    println!("After insert at index 2: {:?}", deque);
    
    // Remove from arbitrary position
    let removed = deque.remove(2);
    println!("Removed: {:?}", removed);  // Some(99)
    
    // Rotate elements
    deque.rotate_left(2);  // Move first 2 elements to back
    println!("After rotate left: {:?}", deque);
    
    deque.rotate_right(1);  // Move last element to front
    println!("After rotate right: {:?}", deque);

    // STRING VS &str

    let string_slice : &str = "Hello Rust";
    println!("String slice {}",string_slice);

    let mut owned_string: String = String::from("Hello");
    owned_string.push_str("Rust");
    println!("owned string : {}",owned_string);

    // flexibility building a strings 
    let mut dynamic = String::new();
    dynamic.push_str("User: ");
    dynamic.push_str("Ram");

    // 5 diff ways of creating strings

    // String::new() - empty string
    let mut s1 = String::new();
    println!("emoty string: {}",s1);

    // way 2 - String::from() from string literal
    let s2 = String::from("HelloChandu");
    println!("from literal : {}",s2);

    // way 3 - to_string() method
    let s3 = "Chandu".to_string();
    println!("Using to_string: {}", s3);

    // way 4 - to_owned()
    let s4 = "Hello".to_owned();
    println!("Using to_owned: {}", s4);

    // way 5 - format !macro(powerful)

    let name = "Aradhya";
    let age = 21;
    let s5 = format!("my names is {} and im {} years old",name,age);
    println!("using format : {}",s5);

    // String manipulation 

    let mut message = String::from("Hello");
    
    // push_str() - Append a string slice
    message.push_str(", World");
    println!("After push_str: {}", message);
    
    // push() - Append a single character
    message.push('!');
    println!("After push: {}", message);
    
    // insert() - Insert at specific position
    message.insert(5, ',');  // Insert comma at index 5
    println!("After insert: {}", message);
    
    // insert_str() - Insert string at position
    message.insert_str(0, "Greeting: ");
    println!("After insert_str: {}", message);

    // Operations - Joining strings concatenating strings 

    let s6 = String::from("hello");
    let s7 = String::from("world");

    let s8 = s6 + &s7;

    println!("Concatenated: {}", s3);
    // println!("{}", s1);  // ERROR! s1 was moved
    println!("s2 still usable: {}", s2);  // s2 is fine (we borrowed it)
    
    // Why does + take ownership?
    // Performance! It can reuse s1's memory instead of allocating new memory

    println!("Concatenated: {}", s3);
    // println!("{}", s1);  // ERROR! s1 was moved
    println!("s2 still usable: {}", s2);  // s2 is fine (we borrowed it)
    
    // Why does + take ownership?
    // Performance! It can reuse s1's memory instead of allocating new memory
    
    // METHOD 2: Using format! macro (easier, clearer)
    let first = String::from("Hello");
    let second = String::from("World");
    let combined = format!("{}, {}!", first, second);
    println!("With format!: {}", combined);
    // first and second are still usable!
    println!("first: {}, second: {}", first, second);

    // ITERATORS

    let digits = vec![1,2,4,5,3];

    let mut iter = digits.iter();

    // next() gives you the next item (returns Option)
    println!("First call to next(): {:?}", iter.next());   // Some(&1)
    println!("Second call to next(): {:?}", iter.next());  // Some(&2)
    println!("Third call to next(): {:?}", iter.next());   // Some(&3)
    println!("Fourth: {:?}", iter.next());   // Some(&4)
    println!("Fifth: {:?}", iter.next());    // Some(&5)
    println!("Sixth: {:?}", iter.next());    // None (exhausted)
    println!("Seventh: {:?}", iter.next());  // None (still exhausted)

    // three ways t iterate

    let dataa = vec![10,20,30];

    // type 1  = iter - Borrows each element (&T)

    println!("using iter() - borrows:");
    for item in dataa.iter(){
        println!("{}",item);
    }

    // data is still usable after 
    println!("data still exists :{:?}",dataa);

    let mut data2 = vec![10,30,40];
    println!("iter_mut() - mutable borrow");
    for item in data2.iter_mut() {
        *item *= 2;
    }

    println!("data2 after modification: {:?}", data2); 

    // into_iter() - takes ownership (T)

    // Map = transforming each element

    let n = vec![1,2,3,4,5];
    // map() applies a function to each element

    let doubled : Vec<i32> = n.iter();
    .map(|x| x*2); // |X| is a closure 
    .collect(); // collect() consumes iterator 

    println!("Original: {:?}", n);
    println!("Doubled: {:?}", doubled);

    // map() is lazy it doesnt do anything until u call collect()

    let lazy_map = n.iter().map(|x| {
        println!("processingg {}",x);
        x * 2;
    });

    println!("lazy map created");

    let result : Vec<i32> = lazy_map.collect();
    println!("result {:?}",result);

    // practicing maps

    let words = vec!["hello","world","rust"];

    le lengths: Vec<usize> = words.iter()
    .map(|word| word.len())
    .collect();

    println!("Word lengths : {:?}",lengths);

    let uppercased: Vec<String> = words.iter()
    .map(|word| word.to_uppercase())
    .collect();

    println!("Uppercased: {:?}",uppercased);

    // FILTER - SELECTING ELEMENTS

    let nums = vec![1,2,3,4,5,6,7,8,9,10];

    // filer() keeps only elements where closure returns true

    let evens : Vec<i32> = nums.iter()
    .filter(|x| *x%2 == 0) //derefernce the values
    .copied()
    .collect();

    println!("even numbers : {:?}",evens);

    let odds : Vec<i32> = nums.iter()
    .filter(|x| *x%2 !=0)
    .copied()
    .collect();

    println!("Odd numbers {:?}",odds);

    // filter strings

    let words = vec!["rust", "is", "awesome", "and", "fast"];
    let long_words: Vec<&str> = words.iter()
        .filter(|word| word.len() > 3)
        .copied()
        .collect();
    
    println!("Long words: {:?}", long_words);

    // chaining operationss

    let text = vec!["hello","Ram","vasudeva","krishna"];
    let processed: Vec<String> = text.iter()
    .filter(|word| word.len() > 4)
    .map(|word| word.to_uppercase())
    .map(|word| format!("{}",word))
    .collect();

    println!("processed words ", processed );

    // fold reduction
    // fold() combines all elements into a single value


    let ns = vec![1,2,3,4,5];
    let sum = ns.iter().fold(0, |acc,x|, acc*x);

    println!("sum : {}",sum);
    
    let product = numbers.iter().fold(1,|acc,x| acc*x);
    println!("product : {}",product);

     // Build a string
    let words = vec!["Hello", "World", "Rust"];
    let sentence = words.iter().fold(String::new(), |mut acc, word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(word);
        acc
    });
    println!("Sentence: {}", sentence);
    
    
    // ────────────────────────────────────────────
    // COMMON ITERATOR METHODS
    // ────────────────────────────────────────────
    
    println!("\n=== COMMON METHODS ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // sum() - Sum all elements
    let total: i32 = numbers.iter().sum();
    println!("Sum: {}", total);
    
    // product() - Multiply all elements
    let prod: i32 = numbers.iter().product();
    println!("Product: {}", prod);
    
    // count() - Count elements
    let count = numbers.iter().count();
    println!("Count: {}", count);
    
    // max() and min() - Find extremes
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("Max: {:?}, Min: {:?}", max, min);
    
    // any() - Check if any element satisfies condition
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    println!("Has even number: {}", has_even);
    
    // all() - Check if all elements satisfy condition
    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);
    
    // find() - Find first matching element
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);
    
    // position() - Find index of first match
    let pos = numbers.iter().position(|x| *x == 3);
    println!("Position of 3: {:?}", pos);
    
    
    // ────────────────────────────────────────────
    // TAKE AND SKIP
    // ────────────────────────────────────────────
    
    println!("\n=== TAKE AND SKIP ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // take(n) - Take first n elements
    let first_three: Vec<i32> = numbers.iter().take(3).copied().collect();
    println!("First 3: {:?}", first_three);
    
    // skip(n) - Skip first n elements
    let after_five: Vec<i32> = numbers.iter().skip(5).copied().collect();
    println!("After skipping 5: {:?}", after_five);
    
    // Combine them
    let middle: Vec<i32> = numbers.iter()
        .skip(3)   // Skip first 3
        .take(4)   // Take next 4
        .copied()
        .collect();
    println!("Middle elements: {:?}", middle);
    
    
    // ────────────────────────────────────────────
    // ENUMERATE - WITH INDICES
    // ────────────────────────────────────────────
    
    println!("\n=== ENUMERATE ===\n");
    
    let fruits = vec!["apple", "banana", "cherry"];
    
    // enumerate() gives (index, value) pairs
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}. {}", index + 1, fruit);
    }
    
    
    // ────────────────────────────────────────────
    // ZIP - COMBINING ITERATORS
    // ────────────────────────────────────────────
    
    println!("\n=== ZIP ===\n");
    
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    
    // zip() pairs elements from two iterators
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
    
    
    // ────────────────────────────────────────────
    // REAL-WORLD EXAMPLE: Data Processing Pipeline
    // ────────────────────────────────────────────
    
    println!("\n=== REAL EXAMPLE: SALES DATA ===\n");
    
    let sales = vec![100, 250, 75, 500, 150, 800, 50];
    
    let analysis = sales.iter()
        .filter(|&&sale| sale >= 100)        // Filter sales >= 100
        .map(|&sale| sale as f64 * 1.2)      // Add 20% tax
        .fold(0.0, |acc, x| acc + x);        // Sum them up
    
    println!("Total revenue (filtered + taxed): ${:.2}", analysis);
    
    
    println!("\n=== PRACTICE: FINDING AVERAGE ===\n");
    
    let scores = vec![85, 92, 78, 95, 88];
    
    let sum: i32 = scores.iter().sum();
    let count = scores.len() as f64;
    let average = sum as f64 / count;
    
    println!("Scores: {:?}", scores);
    println!("Average: {:.2}", average);
    
    
    // ────────────────────────────────────────────
    // FLAT_MAP - FLATTENING NESTED STRUCTURES
    // ────────────────────────────────────────────
    
    println!("\n=== FLAT_MAP - FLATTENING ===\n");
    
    let words = vec!["hello world", "rust programming"];
    
    // We want all individual words
    let all_words: Vec<&str> = words.iter()
        .flat_map(|sentence| sentence.split_whitespace())
        .collect();
    
    println!("All words: {:?}", all_words);
    // ["hello", "world", "rust", "programming"]
    
    // flat_map() is like map() + flatten()
    // It maps each element and then flattens the result
    
    
    // ────────────────────────────────────────────
    // COLLECT - DIFFERENT COLLECTION TYPES
    // ────────────────────────────────────────────
    
    println!("\n=== COLLECT INTO DIFFERENT TYPES ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Collect into Vec
    let vec: Vec<i32> = numbers.iter().copied().collect();
    println!("Vec: {:?}", vec);
    
    // Collect into HashSet (duplicates removed)
    use std::collections::HashSet;
    let duplicates = vec![1, 2, 2, 3, 3, 3];
    let set: HashSet<i32> = duplicates.into_iter().collect();
    println!("HashSet: {:?}", set);
    
    // Collect into String
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let string: String = chars.into_iter().collect();
    println!("String: {}", string);
    
    
    // ────────────────────────────────────────────
    // PARTITION - SPLIT INTO TWO GROUPS
    // ────────────────────────────────────────────
    
    println!("\n=== PARTITION ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // partition() splits into two collections based on predicate
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.into_iter()
        .partition(|x| x % 2 == 0);
    
    println!("Evens: {:?}", evens);
    println!("Odds: {:?}", odds);

}