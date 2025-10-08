pub fn check_temp(temperature : i32) {
    // let temperature = 25; wronggg because overwrite parameter
    // Key rule
    // Do not redeclare a parameter inside the function with let — it shadows the value you passed.
    // Pass the value as an argument, and just use it inside the function.          

    if temperature > 30 {
        println!("It's hot!");
    } else if temperature > 20 {
        println!("It's pleasant");
    } else if temperature > 10 {
        println!("It's cool");
    } else {
        println!("It's cold!");
    }
}

// enter if else produces a valure

pub fn produce_value(age:i32){
    let status = if age>= 18{
        "adult"
    } else {
        "minor"
    };
    println!("status: {}",status);
}

pub fn check_grade(score: i32){
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'F'
    };
    
    println!("Grade: {}", grade);
}

// returning values from loops 

pub fn double_counter_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Return value when breaking
        }
    };

    result
}

pub fn attempt_loop() -> &'static str {
    let mut attempt = 0;

    let successful_value = loop{
        attempt += 1;
        println!("attempt {}...",attempt);

        if attempt == 3{
            break "Sucess";
        }
        if attempt == 10{
            break "failed after 10 attempts";
        }
    };
}
