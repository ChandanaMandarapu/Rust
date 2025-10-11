use std::collections::HashMap;

fn main ( ) {
    println!("HASHMAP");

    // creating hashmap 
    // HashMap <K V> where : 
    // K = Keytype
    // V = Valueteype

    let mut phone_book : HashMap<String,String> = HashMap::new();
    // creating a hashmap which has both strings

    // insert adding key value pairs

    phone_book.insert(String::from("Ram"),String::from("9998989"));
    phone_book.insert(String::from("krishna"),String::from("99978787"));
    phone_book.insert(String::from("Hari"),String::from("89899892"));

    println!("phone book {:?}",phone_book);

    let old_value = phone_book.insert (
        String::from("Ram"),
        String::from("990000") // new number for ram
    );

    println!("ram's old number: {:?}",old_value);
    println!("phone book now: {:?}",phone_book);

    // getting data from hashmap

    // get(key) returns Option(&V)
    // WHY option? Because the key might not exist sometimes 
    // &V - a reference 

    let ram_number = phone_book.get("Ram");
    match ram_number {
        // FIX: Match arm must end with a comma ','
        Some(number) => println!("Ram number :{}",number), 
        // FIX: Match arm must end with a comma ','
        None => println!("ram not found"),
    }

    // if key doesnt exist

    let unknown = phone_book.get("Jay");
    println!("jay's number: {:?}",unknown);

    // Entry API is another form of insertion it lets u check if key exists and inserrt/modify in on go nice na

    let mut scores:HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("Blue Team"),10);

    // or_insert( ) - insert only if key doesnt exist

    // ntg if blue team doesnt exist insert 50 if it does exist do nothing give me existing value

    scores.entry(String::from("Blue Team")).or_insert(50);

    // FIX: Format string changed from :{?} to {:?}
    println!("Blue Team score: {:?}",scores.get("Blue Team"));

    // trying with a new key

    scores.entry(String::from("Red Team")).or_insert(30);
    // FIX: Format string changed from :{?} to {:?}
    println!("Red Team score: {:?}",scores.get("Red Team"));

    // modifying entry

    let blue_score = scores.entry(String::from("Blue Team")).or_insert(0);

    *blue_score += 10;
    println!("Updated scores: {:?}", scores);

    // 	word counter

    let text = "hare krishna hare rama";
    let words = text.split_whitespace(); 

    let mut word_count : HashMap<&str, i32> = HashMap::new();

    // FIX: Corrected loop syntax from 'let word in words' to 'for word in words'
    for word in words {
        // checking if word exists in hashmap
        // if 	yes - increase its count
        // if no - insert with count 0, thehn increment

        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word frequencies: {:?}", word_count);

    // iterating through hashmap

    let mut grades : HashMap<&str,i32> = HashMap::new();
    grades.insert("achyutha",98);
    grades.insert("rama",99);
    grades.insert("krishna",92);

    // method 1 iterate over key value pairs

    println!("\nAll Grades:");
    for (student, grade) in &grades {
        // (student and grade tuple destructuring)
        // &grades = borrow hashmap 
        println!("{} scored {}",student,grade);
    }

    // method2 iterate over just keys

    println!("\nAll students:");
    for student in grades.keys() {
        // FIX: Cleaned up the println! inside the loop
        println!("  {}",student);
    }

    // method 3 iterate over just values

    println!("\nAll scores:");
    for grade in grades.values() {
        println!("  {}", grade);
    }

    // method 4 mutable iteration we can modify values
    
    println!("\nAdding Bonus Points...");
    for grade in grades.values_mut(){
        *grade += 5; // add 5 marks to everyone
    }

    println!("updated grades :{:?}",grades);

    // 	checking exists and removing 

    println!("\nChecking and removing");

    let mut inventory: HashMap<&str,i32> = HashMap::new();
    inventory.insert("appples",50);
    inventory.insert("bananas",30);
    inventory.insert("kiwi",90);

    if inventory.contains_key("kiwi"){
        println!("we have kiwi in stock");
    }

    // len() - Number of entries
    println!("Items in inventory: {}", inventory.len());

    // remove() - Remove entry and return its value
    let removed = inventory.remove("bananas");
    println!("Removed bananas: {:?}", removed); 	// Some(30)
    println!("Inventory now: {:?}", inventory);

    // try removing non-ezistent key

    let nothing = inventory.remove("grapes");
    // FIX: Corrected variable name from 'grapes' to 'nothing'
    println!("Removed grapes: {:?}",nothing);

    // creating hashmap from vectors
    // we can zip two vectors together in a hashmap

    let keys = vec!["one","two","three"];
    let values = vec![1,2,3];

    // / zip() pairs up elements: ("one", 1), ("two", 2), ("three", 3)
    // collect() turns these pairs into a HashMap

    let number_map: HashMap<_,_> = keys.into_iter()
        .zip(values.into_iter())
        .collect(); // FIX: Removed misplaced semicolon and called .collect()

    println!("Number map: {:?}", number_map);

    // common hashmap patterns

    let mut settings: HashMap<&str,i32> 	= HashMap::new();
    settings.insert("volume",87);

    // get value or use default if key doesnt exist

    let volume = settings.get("volume").unwrap_or(&50);
    let brightness = settings.get("brightness").unwrap_or(&100);

    println!("volume : {}",volume); //87 exists
    println!("brightness: {}",brightness); //100 default

    // update insert 

    let mut cache: HashMap<&str,String> = HashMap::new();
    // LOGICAL FIX: Insert initial data so and_modify has something to modify
    cache.insert("user_data", String::from("initial data: ")); 

    cache.entry("user_data")
        .and_modify(|v|v.push_str(" updated")) // if exists modify
        .or_insert(String::from("fresh data")); // if not insert

    // Test or_insert for a new key that doesn't exist
    cache.entry("new_key")
        .and_modify(|v|v.push_str("ignored"))
        .or_insert(String::from("only inserted"));
    
    println!("cache : {:?}",cache);

    let people = vec![
        ("Achyutha", 25),
        ("Balarama", 30),
        ("Chandra", 25),
        ("Damodara", 30),
        ("Eeshwar", 25),
    ];

    let mut age_groups: HashMap<i32, Vec<&str>> = HashMap::new();

    for (name, age) in people {
        age_groups.entry(age)
            .or_insert(Vec::new()) 	// Create empty vec if age doesn't exist
            .push(name); 	 	 	// Add name to the vec
    }
    
    println!("People grouped by age: {:?}", age_groups);

}