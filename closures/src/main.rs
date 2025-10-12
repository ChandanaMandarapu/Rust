fn main() {
    println!("Closures");
    // a closure is a function without any name it can use for instant thing
    // regular function
    fn add_one_function(x:i32) -> i32 {
        x + 1
    }
    // closure same logic diff syntax
    let add_one_closure = |x:i32| -> i32 {
        x+1
    };

    // even shorter
    let add_one_short = |x| x+1;

    println!("function: {}",add_one_function);
    println!("function: {}",add_one_closure);
    println!("function: {}",add_one_short);

    // capturing environment

    let multiplier = 3;
    let multiply = |x| x*multiplier;
    println!("5 * {} = {}",multiplier,multiply(5));

    // 3 ways of capturing 
    // 1st way borrowing immutable reference

    let text = String::from("Chandu");
    let print_text = || {
        // borrows text immutably
        println!("captured text : {}",text);
    }
    print_text();
    println!("original text: {}",text);

    // way 2 - mutable bororowing

    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("counter: {}",counter);
    };

    increment();
    increment();
    println!("final counter {}",counter);

    // way 3 - taking ownership (move)
    let data = vec![1,2,3];
    let consume_data = move || {
        // move keyword takes ownership of data
        println!("consumed data: {:?}",data);
    };

    // closures wit iterators

    let nums = vec![1,2,3,4,5];
    // map() takes a closure
    let doubled: Vec<i32> = nums.iter()
    .map(|x| x*2)
    .collect();
    println!("doubled: {:?}",doubled);

    // filer takes a closure 
    let threshold = 3;
    let fileterd: Vec<i32> = nums.iter()
    .filter(|&&x| x > threshold);
    .copied()
    .collect();

    println!("greater than {} :{:?}",threshold,fileterd);

    let mut people = vec![
        ("ram":8),
        ("krishna":2),
        ("achyutha:"9),
    ];

    // sort_by() takes a closure for comparision
    people.sort_by(|a,b| a.1.cmp(&b.1)) // sort by age second element
    println!("sorted by age : {:?}",people);
    people.sort_by(|a,b| a.0.cmp(&b.0)) //sort byname
    println!("sorted by name : {:?}",people);

    // closure traits
    // Fn = can be called multiple times and borrows immutably 
    let x = 9;
    let fn_closure || println!("X is {}",x);
    fn_closure();
    fn_closure();
    // FnMut = can be called multiple times and borrows mutably
    let mut count = 0;
    let mut fnmut_closure = ||  {
        count += 1;
        println!("count is {}",count);
    };
    fnmut_closure();
    fnmut_closure();
    // FnOnce - can only be called once (consumes captured values)
    let fnonce_closure = move || {
        println!("data: {:?}", data);
        // If we do something that consumes data, this is FnOnce
    };
    fnonce_closure();

    // functions accepting closures

    // function that takes Fn closure 
    fn apply_twice<F>(f:F,x:i32) -> i32
    where
    F : Fn(i32) -> i32, // F must implement Fn trait
    {
        f(f(x))
    }

    let add_two =|x| x+2;
    let result = apply_twice(add_two,5);
    println!("5 + 2 + 2 = {}", result);

    // Function that takes FnMut closure
    fn do_n_times<F>(mut f: F, n: usize)
    where
        F: FnMut(),
    {
        for _ in 0..n {
            f();
        }
    }
    
    let mut call_count = 0;
    do_n_times(|| {
        call_count += 1;
        println!("Called {} time(s)", call_count);
    }, 3);

    // closures have unique anonymous types
    // to return them we need to use trait objects or impl trait

    // using impl trait 
    fn make_adder(n:i32) -> impl Fn(i32) -> i32{
        move |x| x+n;
    }

    let add_5 = make_adder(5);
    println!("10 + 5 = {}", add_5(10));
    
    let add_100 = make_adder(100);
    println!("10 + 100 = {}", add_100(10));

    // button click handlers - example 

    struct Button {
        label:String,
        onclick: Box<dyn Fn()>,
    }

    impl Button{
        fn click(&self){
            (self.onclick()); //calling closure
        }
    }

    // let mut/ click_count = 0;
    let button = Button{
        label : String::from("click me");
        onclick: Box::new(|| {
            println!("button '{}' was clicked","click me");
        }),
    };

    button.click();
    button.click();

    println!("\n=== REAL EXAMPLE: RETRY LOGIC ===\n");
    
    fn retry<F, T>(mut operation: F, max_attempts: u32) -> Option<T>
    where
        F: FnMut() -> Option<T>,
    {
        for attempt in 1..=max_attempts {
            println!("Attempt {}", attempt);
            if let Some(result) = operation() {
                return Some(result);
            }
        }
        None
    }
    
    let mut attempt_count = 0;
    let result = retry(|| {
        attempt_count += 1;
        if attempt_count >= 3 {
            Some("Success!")
        } else {
            None
        }
    }, 5);
    
    match result {
        Some(msg) => println!("Result: {}", msg),
        None => println!("All attempts failed"),
    }
    
    
    // ────────────────────────────────────────────
    // CLOSURE AS CONFIGURATOR
    // ────────────────────────────────────────────
    
    println!("\n=== EXAMPLE: CONFIGURATION ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Let user configure how to process data
    fn process_numbers<F>(nums: &[i32], config: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        nums.iter().map(|&x| config(x)).collect()
    }
    
    // Different configurations
    let doubled = process_numbers(&numbers, |x| x * 2);
    println!("Doubled: {:?}", doubled);
    
    let squared = process_numbers(&numbers, |x| x * x);
    println!("Squared: {:?}", squared);
    
    let custom = process_numbers(&numbers, |x| {
        if x % 2 == 0 {
            x / 2
        } else {
            x * 3
        }
    });
    println!("Custom: {:?}", custom);
    
    
    // ────────────────────────────────────────────
    // COMBINING CLOSURES WITH FILTER & MAP
    // ────────────────────────────────────────────
    
    println!("\n=== ADVANCED: PIPELINE WITH CLOSURES ===\n");
    
    let sales_data = vec![
        ("Product A", 100),
        ("Product B", 250),
        ("Product C", 75),
        ("Product D", 500),
        ("Product E", 150),
    ];
    
    let min_sales = 100;
    let tax_rate = 0.2;
    
    let total_revenue: f64 = sales_data.iter()
        .filter(|(_, sales)| *sales >= min_sales)  // Filter low sales
        .map(|(_, sales)| *sales as f64)            // Convert to f64
        .map(|sales| sales * (1.0 + tax_rate))      // Add tax
        .sum();                                      // Sum it up
    
    println!("Total revenue (filtered + taxed): ${:.2}", total_revenue);

}
