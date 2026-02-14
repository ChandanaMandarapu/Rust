// File 11: Real-World Error Handling
// Custom error types, error chaining, From/Into conversions, and the ? operator

use std::fmt;
use std::num::ParseIntError;
use std::collections::HashMap;

// ─── Custom Error Types ───────────────────────────────────────────────────────

#[derive(Debug)]
enum ConfigError {
    MissingKey(String),
    InvalidValue { key: String, value: String, expected: String },
    ParseError(String),
    IoError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingKey(key) =>
                write!(f, "Missing required config key: '{}'", key),
            ConfigError::InvalidValue { key, value, expected } =>
                write!(f, "Invalid value '{}' for key '{}', expected {}", value, key, expected),
            ConfigError::ParseError(msg) =>
                write!(f, "Parse error: {}", msg),
            ConfigError::IoError(msg) =>
                write!(f, "IO error: {}", msg),
        }
    }
}

// Automatic conversion from ParseIntError into ConfigError
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::ParseError(e.to_string())
    }
}

// Automatic conversion from std::io::Error
impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::IoError(e.to_string())
    }
}

type ConfigResult<T> = Result<T, ConfigError>;

// ─── Config System ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Config {
    data: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Config { data: HashMap::new() }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> ConfigResult<&str> {
        self.data
            .get(key)
            .map(|s| s.as_str())
            .ok_or_else(|| ConfigError::MissingKey(key.to_string()))
    }

    fn get_int(&self, key: &str) -> ConfigResult<i64> {
        let raw = self.get(key)?;
        raw.parse::<i64>().map_err(|_| ConfigError::InvalidValue {
            key: key.to_string(),
            value: raw.to_string(),
            expected: "integer".to_string(),
        })
    }

    fn get_bool(&self, key: &str) -> ConfigResult<bool> {
        let raw = self.get(key)?;
        match raw.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(true),
            "false" | "0" | "no" => Ok(false),
            _ => Err(ConfigError::InvalidValue {
                key: key.to_string(),
                value: raw.to_string(),
                expected: "boolean (true/false/yes/no/1/0)".to_string(),
            }),
        }
    }

    fn get_with_default<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.data.get(key).map(|s| s.as_str()).unwrap_or(default)
    }
}

// ─── Error Chaining ───────────────────────────────────────────────────────────

#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    Database(DbError),
    Validation(Vec<String>),
    NotFound { resource: String, id: u32 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Configuration error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Validation(errors) => {
                write!(f, "Validation failed: {}", errors.join(", "))
            }
            AppError::NotFound { resource, id } => {
                write!(f, "{} with id {} not found", resource, id)
            }
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        AppError::Database(e)
    }
}

type AppResult<T> = Result<T, AppError>;

// ─── Database Error ───────────────────────────────────────────────────────────

#[derive(Debug)]
enum DbError {
    ConnectionFailed(String),
    QueryFailed { query: String, reason: String },
    DuplicateKey(String),
    Timeout,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::ConnectionFailed(addr) =>
                write!(f, "Failed to connect to '{}'", addr),
            DbError::QueryFailed { query, reason } =>
                write!(f, "Query '{}' failed: {}", query, reason),
            DbError::DuplicateKey(key) =>
                write!(f, "Duplicate key violation: '{}'", key),
            DbError::Timeout =>
                write!(f, "Database operation timed out"),
        }
    }
}

// ─── Validation ───────────────────────────────────────────────────────────────

struct Validator {
    errors: Vec<String>,
}

impl Validator {
    fn new() -> Self {
        Validator { errors: Vec::new() }
    }

    fn require_non_empty(&mut self, field: &str, value: &str) -> &mut Self {
        if value.trim().is_empty() {
            self.errors.push(format!("'{}' cannot be empty", field));
        }
        self
    }

    fn require_min_length(&mut self, field: &str, value: &str, min: usize) -> &mut Self {
        if value.len() < min {
            self.errors.push(format!(
                "'{}' must be at least {} characters (got {})",
                field, min, value.len()
            ));
        }
        self
    }

    fn require_email(&mut self, field: &str, value: &str) -> &mut Self {
        if !value.contains('@') || !value.contains('.') {
            self.errors.push(format!("'{}' must be a valid email address", field));
        }
        self
    }

    fn require_range(&mut self, field: &str, value: i64, min: i64, max: i64) -> &mut Self {
        if value < min || value > max {
            self.errors.push(format!(
                "'{}' must be between {} and {} (got {})",
                field, min, max, value
            ));
        }
        self
    }

    fn validate(self) -> AppResult<()> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::Validation(self.errors))
        }
    }
}

// ─── User Service ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

struct UserService {
    users: HashMap<u32, User>,
    next_id: u32,
    config: Config,
}

impl UserService {
    fn new(config: Config) -> AppResult<Self> {
        // Validate required config keys at startup
        let _max_users = config.get_int("max_users")?;
        let _debug = config.get_bool("debug")?;

        Ok(UserService {
            users: HashMap::new(),
            next_id: 1,
            config,
        })
    }

    fn create_user(&mut self, name: String, email: String, age: u32) -> AppResult<&User> {
        // Validate input
        let mut validator = Validator::new();
        validator
            .require_non_empty("name", &name)
            .require_min_length("name", &name, 2)
            .require_non_empty("email", &email)
            .require_email("email", &email)
            .require_range("age", age as i64, 0, 150);
        validator.validate()?;

        // Check max users limit
        let max_users = self.config.get_int("max_users")? as usize;
        if self.users.len() >= max_users {
            return Err(AppError::Database(DbError::QueryFailed {
                query: "INSERT INTO users".to_string(),
                reason: format!("max user limit ({}) reached", max_users),
            }));
        }

        // Check for duplicate email
        let email_exists = self.users.values().any(|u| u.email == email);
        if email_exists {
            return Err(AppError::Database(DbError::DuplicateKey(
                format!("email '{}'", email),
            )));
        }

        let user = User {
            id: self.next_id,
            name,
            email,
            age,
        };
        self.next_id += 1;
        self.users.insert(user.id, user);
        Ok(self.users.get(&(self.next_id - 1)).unwrap())
    }

    fn get_user(&self, id: u32) -> AppResult<&User> {
        self.users.get(&id).ok_or(AppError::NotFound {
            resource: "User".to_string(),
            id,
        })
    }

    fn update_email(&mut self, id: u32, new_email: String) -> AppResult<()> {
        // Validate email
        let mut validator = Validator::new();
        validator.require_email("email", &new_email);
        validator.validate()?;

        // Check user exists
        if !self.users.contains_key(&id) {
            return Err(AppError::NotFound {
                resource: "User".to_string(),
                id,
            });
        }

        // Check duplicate
        let duplicate = self.users.values().any(|u| u.email == new_email && u.id != id);
        if duplicate {
            return Err(AppError::Database(DbError::DuplicateKey(
                format!("email '{}'", new_email),
            )));
        }

        self.users.get_mut(&id).unwrap().email = new_email;
        Ok(())
    }
}

// ─── Result Combinators ───────────────────────────────────────────────────────

fn demonstrate_combinators() {
    println!("\n── Result Combinators ──");

    // map: transform Ok value
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|x| x * 2);
    println!("map:         {:?}", doubled);

    // map_err: transform Err value
    let err: Result<i32, i32> = Err(404);
    let mapped_err = err.map_err(|code| format!("HTTP error {}", code));
    println!("map_err:     {:?}", mapped_err);

    // and_then: chain fallible operations
    let chained = "42"
        .parse::<i32>()
        .and_then(|n| if n > 0 { Ok(n * 2) } else { Err("must be positive".parse::<i32>().unwrap_err()) });
    println!("and_then:    {:?}", chained);

    // or_else: fallback on error
    let fallback: Result<i32, &str> = Err("oops")
        .or_else(|_| Ok::<i32, &str>(99));
    println!("or_else:     {:?}", fallback);

    // unwrap_or / unwrap_or_else
    let val = Err::<i32, &str>("fail").unwrap_or(0);
    println!("unwrap_or:   {}", val);

    let val2 = Err::<i32, &str>("fail").unwrap_or_else(|_| 42);
    println!("unwrap_or_else: {}", val2);

    // ok() converts Result<T,E> to Option<T>
    let opt: Option<i32> = Ok::<i32, &str>(7).ok();
    println!("ok():        {:?}", opt);

    // Collecting results — stops at first error
    let strings = vec!["1", "2", "3", "4"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("collect Ok:  {:?}", numbers);

    let bad = vec!["1", "oops", "3"];
    let numbers2: Result<Vec<i32>, _> = bad.iter().map(|s| s.parse::<i32>()).collect();
    println!("collect Err: {:?}", numbers2);
}

// ─── Option chaining ─────────────────────────────────────────────────────────

#[derive(Debug)]
struct Address {
    city: Option<String>,
    zip: Option<String>,
}

#[derive(Debug)]
struct Profile {
    address: Option<Address>,
    bio: Option<String>,
}

fn get_city(profile: &Profile) -> Option<&str> {
    profile
        .address
        .as_ref()
        .and_then(|a| a.city.as_deref())
}

fn demonstrate_option_chaining() {
    println!("\n── Option Chaining ──");

    let profile = Profile {
        address: Some(Address {
            city: Some("Hyderabad".to_string()),
            zip: Some("500001".to_string()),
        }),
        bio: Some("Rust enthusiast".to_string()),
    };

    println!("City: {:?}", get_city(&profile));

    let empty = Profile { address: None, bio: None };
    println!("City (none): {:?}", get_city(&empty));

    // filter, flatten, zip, unzip
    let num: Option<i32> = Some(10);
    let filtered = num.filter(|&x| x > 5);
    println!("filter:    {:?}", filtered);

    let nested: Option<Option<i32>> = Some(Some(42));
    let flat = nested.flatten();
    println!("flatten:   {:?}", flat);

    let a = Some(1);
    let b = Some("hello");
    let zipped = a.zip(b);
    println!("zip:       {:?}", zipped);
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Real-World Error Handling ===\n");

    // Build config
    let mut config = Config::new();
    config.set("max_users", "5");
    config.set("debug", "true");
    config.set("timeout", "30");

    println!("── Config reads ──");
    println!("max_users:  {:?}", config.get_int("max_users"));
    println!("debug:      {:?}", config.get_bool("debug"));
    println!("missing:    {:?}", config.get("nonexistent"));
    println!("default:    {}", config.get_with_default("theme", "dark"));

    // Boot the service
    let mut service = match UserService::new(config) {
        Ok(s) => { println!("\nService started successfully"); s }
        Err(e) => { eprintln!("Failed to start: {}", e); return; }
    };

    println!("\n── Creating users ──");

    let create_and_report = |service: &mut UserService, name: &str, email: &str, age: u32| {
        match service.create_user(name.to_string(), email.to_string(), age) {
            Ok(u) => println!("Created: {:?}", u),
            Err(e) => println!("Error:   {}", e),
        }
    };

    create_and_report(&mut service, "Alice", "alice@example.com", 28);
    create_and_report(&mut service, "Bob", "bob@example.com", 34);
    create_and_report(&mut service, "", "bad", 999);         // validation fails
    create_and_report(&mut service, "Alice2", "alice@example.com", 22); // duplicate email

    println!("\n── Fetching users ──");
    match service.get_user(1) {
        Ok(u) => println!("Found: {:?}", u),
        Err(e) => println!("Error: {}", e),
    }
    match service.get_user(999) {
        Ok(u) => println!("Found: {:?}", u),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n── Updating email ──");
    match service.update_email(1, "alice_new@example.com".to_string()) {
        Ok(()) => println!("Email updated"),
        Err(e) => println!("Error: {}", e),
    }
    match service.update_email(1, "bob@example.com".to_string()) {
        Ok(()) => println!("Email updated"),
        Err(e) => println!("Error: {}", e),
    }

    demonstrate_combinators();
    demonstrate_option_chaining();

    println!("\n=== Done ===");
}