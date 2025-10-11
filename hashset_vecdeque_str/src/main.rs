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
    

}