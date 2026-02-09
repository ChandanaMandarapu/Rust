// Macros and Metaprogramming

macro_rules! vec_repeat {
    ($elem:expr; $count:expr) => {{
        let mut v = Vec::new();
        for _ in 0..$count {
            v.push($elem);
        }
        v
    }};
}

/// A macro for creating a hashmap
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

/// A macro for logging with different levels
macro_rules! log {
    (INFO: $msg:expr) => {
        println!("[INFO] {}", $msg);
    };
    (WARN: $msg:expr) => {
        println!("[WARN] {}", $msg);
    };
    (ERROR: $msg:expr) => {
        eprintln!("[ERROR] {}", $msg);
    };
    ($level:ident: $msg:expr, $($arg:tt)*) => {
        log!($level: format!($msg, $($arg)*));
    };
}

/// A macro for creating getters and setters
macro_rules! create_accessors {
    ($struct_name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        impl $struct_name {
            $(
                paste::paste! {
                    pub fn [<get_ $field>](&self) -> &$type {
                        &self.$field
                    }

                    pub fn [<set_ $field>](&mut self, value: $type) {
                        self.$field = value;
                    }
                }
            )*
        }
    };
}

/// A macro that implements a trait for multiple types
macro_rules! impl_display_for {
    ($($type:ty),* $(,)?) => {
        $(
            impl std::fmt::Display for $type {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        )*
    };
}

/// A macro for creating enums with associated values
macro_rules! create_enum {
    ($name:ident { $($variant:ident($type:ty)),* $(,)? }) => {
        #[derive(Debug)]
        enum $name {
            $(
                $variant($type),
            )*
        }

        impl $name {
            fn variant_name(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant(_) => stringify!($variant),
                    )*
                }
            }
        }
    };
}

/// A macro for measuring execution time
macro_rules! time_it {
    ($label:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let elapsed = start.elapsed();
        println!("{} took: {:?}", $label, elapsed);
        result
    }};
}

/// A macro for conditional compilation based on features
macro_rules! feature_gate {
    ($feature:tt, $code:block) => {
        #[cfg(feature = $feature)]
        $code
    };
}

/// A macro for asserting equality with custom messages
macro_rules! assert_eq_msg {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            panic!(
                "Assertion failed: {} != {}\nMessage: {}",
                $left, $right, $msg
            );
        }
    };
}

/// A macro for creating a builder pattern
macro_rules! builder {
    ($struct_name:ident {
        $($field:ident: $type:ty),* $(,)?
    }) => {
        paste::paste! {
            #[derive(Default)]
            struct [<$struct_name Builder>] {
                $(
                    $field: Option<$type>,
                )*
            }

            impl [<$struct_name Builder>] {
                fn new() -> Self {
                    Self::default()
                }

                $(
                    fn $field(mut self, value: $type) -> Self {
                        self.$field = Some(value);
                        self
                    }
                )*

                fn build(self) -> Result<$struct_name, String> {
                    Ok($struct_name {
                        $(
                            $field: self.$field.ok_or(format!("Missing field: {}", stringify!($field)))?,
                        )*
                    })
                }
            }
        }
    };
}

/// A macro for creating test cases
macro_rules! test_cases {
    ($test_name:ident: $($input:expr => $expected:expr),* $(,)?) => {
        #[test]
        fn $test_name() {
            $(
                assert_eq!($input, $expected);
            )*
        }
    };
}

/// A macro for matching multiple patterns
macro_rules! match_any {
    ($value:expr, $($pattern:pat)|+ => $result:expr) => {
        match $value {
            $($pattern)|+ => $result,
            _ => panic!("No pattern matched"),
        }
    };
}

/// A recursive macro for counting items
macro_rules! count {
    () => (0);
    ($head:tt $($tail:tt)*) => (1 + count!($($tail)*));
}

/// A macro for creating a simple DSL
macro_rules! calc {
    ($left:expr + $right:expr) => {
        $left + $right
    };
    ($left:expr - $right:expr) => {
        $left - $right
    };
    ($left:expr * $right:expr) => {
        $left * $right
    };
    ($left:expr / $right:expr) => {
        $left / $right
    };
}

/// A macro for creating documentation
macro_rules! documented_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        struct $name {
            $(
                $(#[$field_meta])*
                $field: $type,
            )*
        }
    };
}

/// Demonstrates basic macro usage
fn demonstrate_basic_macros() {
    println!("=== Basic Macros ===");
    
    // vec_repeat macro
    let repeated = vec_repeat!(5; 10);
    println!("Repeated vector: {:?}", repeated);
    
    // hashmap macro
    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("HashMap: {:?}", map);
    
    // log macro
    log!(INFO: "This is an info message");
    log!(WARN: "This is a warning");
    log!(ERROR: "This is an error");
    log!(INFO: "User {} logged in", "Alice");
}

/// Demonstrates counting macro
fn demonstrate_count_macro() {
    println!("\n=== Count Macro ===");
    
    let count1 = count!(1 2 3 4 5);
    println!("Count of (1 2 3 4 5): {}", count1);
    
    let count2 = count!(a b c);
    println!("Count of (a b c): {}", count2);
}

/// Demonstrates calculation DSL
fn demonstrate_calc_dsl() {
    println!("\n=== Calculation DSL ===");
    
    let result1 = calc!(10 + 5);
    println!("10 + 5 = {}", result1);
    
    let result2 = calc!(20 * 3);
    println!("20 * 3 = {}", result2);
    
    let result3 = calc!(100 / 4);
    println!("100 / 4 = {}", result3);
}

/// A struct for demonstrating builder macro
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

// Note: This would normally use the builder! macro, but we'll define it manually
// since procedural macros require special setup
#[derive(Default)]
struct PersonBuilder {
    name: Option<String>,
    age: Option<u32>,
    email: Option<String>,
}

impl PersonBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    fn age(mut self, value: u32) -> Self {
        self.age = Some(value);
        self
    }

    fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    fn build(self) -> Result<Person, String> {
        Ok(Person {
            name: self.name.ok_or("Missing name")?,
            age: self.age.ok_or("Missing age")?,
            email: self.email.ok_or("Missing email")?,
        })
    }
}

/// Demonstrates builder pattern with macros
fn demonstrate_builder_pattern() {
    println!("\n=== Builder Pattern ===");
    
    let person = PersonBuilder::new()
        .name("Alice".to_string())
        .age(30)
        .email("alice@example.com".to_string())
        .build()
        .unwrap();
    
    println!("Person: {:?}", person);
}

/// A macro for creating test data
macro_rules! test_data {
    (users: [$($name:expr),* $(,)?]) => {{
        vec![
            $(
                Person {
                    name: $name.to_string(),
                    age: 25,
                    email: format!("{}@example.com", $name.to_lowercase()),
                }
            ),*
        ]
    }};
}

/// Demonstrates test data generation
fn demonstrate_test_data() {
    println!("\n=== Test Data Generation ===");
    
    let users = test_data!(users: ["Alice", "Bob", "Charlie"]);
    for user in users {
        println!("{:?}", user);
    }
}

/// A macro for creating a simple state machine
macro_rules! state_machine {
    (
        states: [$($state:ident),* $(,)?],
        transitions: {
            $($from:ident -> $to:ident on $event:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum State {
            $($state),*
        }

        #[derive(Debug)]
        enum Event {
            $($event),*
        }

        impl State {
            fn transition(self, event: &Event) -> Option<State> {
                match (self, event) {
                    $(
                        (State::$from, Event::$event) => Some(State::$to),
                    )*
                    _ => None,
                }
            }
        }
    };
}

/// Demonstrates state machine creation
fn demonstrate_state_machine() {
    println!("\n=== State Machine ===");
    
    state_machine! {
        states: [Idle, Running, Paused, Stopped],
        transitions: {
            Idle -> Running on Start,
            Running -> Paused on Pause,
            Paused -> Running on Resume,
            Running -> Stopped on Stop,
            Paused -> Stopped on Stop
        }
    }
    
    let mut state = State::Idle;
    println!("Initial state: {:?}", state);
    
    if let Some(new_state) = state.transition(&Event::Start) {
        state = new_state;
        println!("After Start: {:?}", state);
    }
    
    if let Some(new_state) = state.transition(&Event::Pause) {
        state = new_state;
        println!("After Pause: {:?}", state);
    }
    
    if let Some(new_state) = state.transition(&Event::Resume) {
        state = new_state;
        println!("After Resume: {:?}", state);
    }
}

/// A macro for creating JSON-like structures
macro_rules! json {
    (null) => {
        JsonValue::Null
    };
    ($value:expr) => {
        JsonValue::from($value)
    };
    ([$($elem:tt),* $(,)?]) => {
        JsonValue::Array(vec![$(json!($elem)),*])
    };
    ({$($key:expr => $value:tt),* $(,)?}) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.to_string(), json!($value));
        )*
        JsonValue::Object(map)
    }};
}

#[derive(Debug, Clone)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Bool(b)
    }
}

impl From<f64> for JsonValue {
    fn from(n: f64) -> Self {
        JsonValue::Number(n)
    }
}

impl From<i32> for JsonValue {
    fn from(n: i32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(s.to_string())
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(s)
    }
}

/// Demonstrates JSON macro
fn demonstrate_json_macro() {
    println!("\n=== JSON Macro ===");
    
    let json_value = json!({
        "name" => "Alice",
        "age" => 30,
        "active" => true,
        "scores" => [95, 87, 92],
        "address" => {
            "city" => "New York",
            "zip" => "10001"
        }
    });
    
    println!("JSON value: {:#?}", json_value);
}

/// A macro for creating SQL-like queries
macro_rules! select {
    ($table:ident where $field:ident = $value:expr) => {{
        println!("SELECT * FROM {} WHERE {} = {:?}", 
                 stringify!($table), 
                 stringify!($field), 
                 $value);
    }};
    ($table:ident) => {{
        println!("SELECT * FROM {}", stringify!($table));
    }};
}

/// Demonstrates SQL-like macro
fn demonstrate_sql_macro() {
    println!("\n=== SQL-like Macro ===");
    
    select!(users);
    select!(users where name = "Alice");
    select!(products where price = 100);
}

/// A macro for creating validators
macro_rules! validate {
    ($value:expr, not_empty) => {
        if $value.is_empty() {
            Err("Value cannot be empty")
        } else {
            Ok($value)
        }
    };
    ($value:expr, min_length: $min:expr) => {
        if $value.len() < $min {
            Err(format!("Value must be at least {} characters", $min))
        } else {
            Ok($value)
        }
    };
    ($value:expr, max_length: $max:expr) => {
        if $value.len() > $max {
            Err(format!("Value must be at most {} characters", $max))
        } else {
            Ok($value)
        }
    };
    ($value:expr, range: $min:expr, $max:expr) => {
        if $value < $min || $value > $max {
            Err(format!("Value must be between {} and {}", $min, $max))
        } else {
            Ok($value)
        }
    };
}

/// Demonstrates validation macro
fn demonstrate_validation() {
    println!("\n=== Validation Macro ===");
    
    match validate!("Hello", not_empty) {
        Ok(v) => println!("Valid: {}", v),
        Err(e) => println!("Error: {}", e),
    }
    
    match validate!("Hi", min_length: 5) {
        Ok(v) => println!("Valid: {}", v),
        Err(e) => println!("Error: {}", e),
    }
    
    match validate!(42, range: 0, 100) {
        Ok(v) => println!("Valid: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

/// A macro for creating configuration structs
macro_rules! config {
    (
        $name:ident {
            $($field:ident: $type:ty = $default:expr),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        struct $name {
            $(
                $field: $type,
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    $(
                        $field: $default,
                    )*
                }
            }
        }
    };
}

/// Demonstrates configuration macro
fn demonstrate_config_macro() {
    println!("\n=== Configuration Macro ===");
    
    config! {
        AppConfig {
            host: String = "localhost".to_string(),
            port: u16 = 8080,
            debug: bool = false,
            max_connections: usize = 100,
        }
    }
    
    let config = AppConfig::default();
    println!("Default config: {:?}", config);
    
    let custom_config = AppConfig {
        host: "0.0.0.0".to_string(),
        port: 3000,
        debug: true,
        max_connections: 500,
    };
    println!("Custom config: {:?}", custom_config);
}

/// A macro for creating enums with string conversion
macro_rules! string_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum $name {
            $($variant),*
        }

        impl $name {
            fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                }
            }

            fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        stringify!($variant) => Some($name::$variant),
                    )*
                    _ => None,
                }
            }
        }
    };
}

/// Demonstrates string enum macro
fn demonstrate_string_enum() {
    println!("\n=== String Enum Macro ===");
    
    string_enum! {
        Color {
            Red,
            Green,
            Blue,
            Yellow,
        }
    }
    
    let color = Color::Red;
    println!("Color as string: {}", color.as_str());
    
    if let Some(parsed_color) = Color::from_str("Blue") {
        println!("Parsed color: {:?}", parsed_color);
    }
}

/// A macro for creating retry logic
macro_rules! retry {
    ($times:expr, $code:block) => {{
        let mut result = None;
        for attempt in 1..=$times {
            match (|| $code)() {
                Ok(val) => {
                    result = Some(Ok(val));
                    break;
                }
                Err(e) => {
                    if attempt == $times {
                        result = Some(Err(e));
                    } else {
                        println!("Attempt {} failed, retrying...", attempt);
                    }
                }
            }
        }
        result.unwrap()
    }};
}

/// Demonstrates retry macro
fn demonstrate_retry_macro() {
    println!("\n=== Retry Macro ===");
    
    let mut attempts = 0;
    let result = retry!(3, {
        attempts += 1;
        if attempts < 3 {
            Err("Failed")
        } else {
            Ok("Success!")
        }
    });
    
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Final error: {}", e),
    }
}

/// Main function demonstrating all macro concepts
fn main() {
    println!("=== Rust Macros Demo ===\n");

    demonstrate_basic_macros();
    demonstrate_count_macro();
    demonstrate_calc_dsl();
    demonstrate_builder_pattern();
    demonstrate_test_data();
    demonstrate_state_machine();
    demonstrate_json_macro();
    demonstrate_sql_macro();
    demonstrate_validation();
    demonstrate_config_macro();
    demonstrate_string_enum();
    demonstrate_retry_macro();

    // Time measurement example
    println!("\n=== Time Measurement ===");
    let result = time_it!("Heavy computation", {
        let mut sum = 0;
        for i in 1..=1000000 {
            sum += i;
        }
        sum
    });
    println!("Result: {}", result);

    println!("\n=== Demo Complete ===");
}