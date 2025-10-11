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

    
}