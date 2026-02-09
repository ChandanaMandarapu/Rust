//Advanced Patterns, Testing, and Best Practices


use std::collections::HashMap;
use std::fmt;

/// The newtype pattern for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct UserId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProductId(u32);

#[derive(Debug, Clone, Copy)]
struct Price(u32); // Price in cents

impl UserId {
    fn new(id: u32) -> Self {
        UserId(id)
    }

    fn value(&self) -> u32 {
        self.0
    }
}

impl ProductId {
    fn new(id: u32) -> Self {
        ProductId(id)
    }

    fn value(&self) -> u32 {
        self.0
    }
}

impl Price {
    fn new(cents: u32) -> Self {
        Price(cents)
    }

    fn in_dollars(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    fn add(&self, other: Price) -> Price {
        Price(self.0 + other.0)
    }
}

/// The builder pattern with type states
mod builder_pattern {
    pub struct EmailBuilder<S> {
        from: Option<String>,
        to: Option<String>,
        subject: Option<String>,
        body: Option<String>,
        state: S,
    }

    pub struct Empty;
    pub struct WithFrom;
    pub struct WithTo;
    pub struct Complete;

    impl EmailBuilder<Empty> {
        pub fn new() -> EmailBuilder<Empty> {
            EmailBuilder {
                from: None,
                to: None,
                subject: None,
                body: None,
                state: Empty,
            }
        }

        pub fn from(mut self, from: String) -> EmailBuilder<WithFrom> {
            self.from = Some(from);
            EmailBuilder {
                from: self.from,
                to: self.to,
                subject: self.subject,
                body: self.body,
                state: WithFrom,
            }
        }
    }

    impl EmailBuilder<WithFrom> {
        pub fn to(mut self, to: String) -> EmailBuilder<WithTo> {
            self.to = Some(to);
            EmailBuilder {
                from: self.from,
                to: self.to,
                subject: self.subject,
                body: self.body,
                state: WithTo,
            }
        }
    }

    impl EmailBuilder<WithTo> {
        pub fn subject(mut self, subject: String) -> Self {
            self.subject = Some(subject);
            self
        }

        pub fn body(mut self, body: String) -> EmailBuilder<Complete> {
            self.body = Some(body);
            EmailBuilder {
                from: self.from,
                to: self.to,
                subject: self.subject,
                body: self.body,
                state: Complete,
            }
        }
    }

    impl EmailBuilder<Complete> {
        pub fn build(self) -> Email {
            Email {
                from: self.from.unwrap(),
                to: self.to.unwrap(),
                subject: self.subject.unwrap_or_default(),
                body: self.body.unwrap(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Email {
        from: String,
        to: String,
        subject: String,
        body: String,
    }

    impl Email {
        pub fn send(&self) {
            println!("Sending email from {} to {}", self.from, self.to);
            println!("Subject: {}", self.subject);
            println!("Body: {}", self.body);
        }
    }
}

/// The strategy pattern with trait objects
trait PaymentStrategy {
    fn pay(&self, amount: u32) -> Result<String, String>;
    fn name(&self) -> &str;
}

struct CreditCardPayment {
    card_number: String,
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: u32) -> Result<String, String> {
        println!("Processing credit card payment of ${:.2}", amount as f64 / 100.0);
        Ok(format!("Charged ${:.2} to card ending in {}", 
                   amount as f64 / 100.0, 
                   &self.card_number[self.card_number.len()-4..]))
    }

    fn name(&self) -> &str {
        "Credit Card"
    }
}

struct PayPalPayment {
    email: String,
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: u32) -> Result<String, String> {
        println!("Processing PayPal payment of ${:.2}", amount as f64 / 100.0);
        Ok(format!("Charged ${:.2} to PayPal account {}", 
                   amount as f64 / 100.0, 
                   self.email))
    }

    fn name(&self) -> &str {
        "PayPal"
    }
}

struct PaymentProcessor {
    strategy: Box<dyn PaymentStrategy>,
}

impl PaymentProcessor {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        PaymentProcessor { strategy }
    }

    fn process(&self, amount: u32) -> Result<String, String> {
        self.strategy.pay(amount)
    }

    fn change_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = strategy;
    }
}

/// The state pattern
trait State {
    fn handle(&self) -> Box<dyn State>;
    fn name(&self) -> &str;
}

struct Idle;
struct Running;
struct Paused;

impl State for Idle {
    fn handle(&self) -> Box<dyn State> {
        println!("Transitioning from Idle to Running");
        Box::new(Running)
    }

    fn name(&self) -> &str {
        "Idle"
    }
}

impl State for Running {
    fn handle(&self) -> Box<dyn State> {
        println!("Transitioning from Running to Paused");
        Box::new(Paused)
    }

    fn name(&self) -> &str {
        "Running"
    }
}

impl State for Paused {
    fn handle(&self) -> Box<dyn State> {
        println!("Transitioning from Paused to Running");
        Box::new(Running)
    }

    fn name(&self) -> &str {
        "Paused"
    }
}

struct StateMachine {
    state: Box<dyn State>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: Box::new(Idle),
        }
    }

    fn transition(&mut self) {
        self.state = self.state.handle();
    }

    fn current_state(&self) -> &str {
        self.state.name()
    }
}

/// The visitor pattern
trait Visitor {
    fn visit_number(&mut self, n: i32);
    fn visit_string(&mut self, s: &str);
    fn visit_bool(&mut self, b: bool);
}

trait Visitable {
    fn accept(&self, visitor: &mut dyn Visitor);
}

enum Value {
    Number(i32),
    Text(String),
    Boolean(bool),
}

impl Visitable for Value {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Value::Number(n) => visitor.visit_number(*n),
            Value::Text(s) => visitor.visit_string(s),
            Value::Boolean(b) => visitor.visit_bool(*b),
        }
    }
}

struct PrintVisitor;

impl Visitor for PrintVisitor {
    fn visit_number(&mut self, n: i32) {
        println!("Number: {}", n);
    }

    fn visit_string(&mut self, s: &str) {
        println!("String: {}", s);
    }

    fn visit_bool(&mut self, b: bool) {
        println!("Boolean: {}", b);
    }
}

struct CountVisitor {
    count: usize,
}

impl CountVisitor {
    fn new() -> Self {
        CountVisitor { count: 0 }
    }

    fn total(&self) -> usize {
        self.count
    }
}

impl Visitor for CountVisitor {
    fn visit_number(&mut self, _: i32) {
        self.count += 1;
    }

    fn visit_string(&mut self, _: &str) {
        self.count += 1;
    }

    fn visit_bool(&mut self, _: bool) {
        self.count += 1;
    }
}

/// The command pattern
trait Command {
    fn execute(&mut self) -> Result<(), String>;
    fn undo(&mut self) -> Result<(), String>;
    fn description(&self) -> &str;
}

struct AddCommand {
    value: i32,
    stack: Vec<i32>,
}

impl AddCommand {
    fn new(value: i32) -> Self {
        AddCommand {
            value,
            stack: Vec::new(),
        }
    }
}

impl Command for AddCommand {
    fn execute(&mut self) -> Result<(), String> {
        self.stack.push(self.value);
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        if self.stack.pop().is_some() {
            Ok(())
        } else {
            Err("Stack is empty".to_string())
        }
    }

    fn description(&self) -> &str {
        "Add value to stack"
    }
}

/// The chain of responsibility pattern
trait Handler {
    fn handle(&self, request: &str) -> Option<String>;
    fn set_next(&mut self, handler: Box<dyn Handler>);
}

struct AuthHandler {
    next: Option<Box<dyn Handler>>,
}

impl AuthHandler {
    fn new() -> Self {
        AuthHandler { next: None }
    }
}

impl Handler for AuthHandler {
    fn handle(&self, request: &str) -> Option<String> {
        if request.contains("auth") {
            Some("Authentication handled".to_string())
        } else if let Some(ref next) = self.next {
            next.handle(request)
        } else {
            None
        }
    }

    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.next = Some(handler);
    }
}

struct LogHandler {
    next: Option<Box<dyn Handler>>,
}

impl LogHandler {
    fn new() -> Self {
        LogHandler { next: None }
    }
}

impl Handler for LogHandler {
    fn handle(&self, request: &str) -> Option<String> {
        if request.contains("log") {
            Some("Logging handled".to_string())
        } else if let Some(ref next) = self.next {
            next.handle(request)
        } else {
            None
        }
    }

    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.next = Some(handler);
    }
}

/// The observer pattern
trait Observer {
    fn update(&mut self, message: &str);
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
    state: String,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
            state: String::new(),
        }
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn set_state(&mut self, state: String) {
        self.state = state.clone();
        self.notify();
    }

    fn notify(&mut self) {
        for observer in &mut self.observers {
            observer.update(&self.state);
        }
    }
}

struct ConcreteObserver {
    name: String,
}

impl ConcreteObserver {
    fn new(name: String) -> Self {
        ConcreteObserver { name }
    }
}

impl Observer for ConcreteObserver {
    fn update(&mut self, message: &str) {
        println!("Observer {} received: {}", self.name, message);
    }
}

/// The repository pattern
trait Repository<T> {
    fn find_by_id(&self, id: u32) -> Option<&T>;
    fn find_all(&self) -> Vec<&T>;
    fn save(&mut self, entity: T) -> u32;
    fn delete(&mut self, id: u32) -> bool;
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct UserRepository {
    data: HashMap<u32, User>,
    next_id: u32,
}

impl UserRepository {
    fn new() -> Self {
        UserRepository {
            data: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Repository<User> for UserRepository {
    fn find_by_id(&self, id: u32) -> Option<&User> {
        self.data.get(&id)
    }

    fn find_all(&self) -> Vec<&User> {
        self.data.values().collect()
    }

    fn save(&mut self, mut entity: User) -> u32 {
        if entity.id == 0 {
            entity.id = self.next_id;
            self.next_id += 1;
        }
        let id = entity.id;
        self.data.insert(id, entity);
        id
    }

    fn delete(&mut self, id: u32) -> bool {
        self.data.remove(&id).is_some()
    }
}

/// Unit testing examples
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation() {
        let id = UserId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_price_addition() {
        let price1 = Price::new(1000); // $10.00
        let price2 = Price::new(500);  // $5.00
        let total = price1.add(price2);
        assert_eq!(total.in_dollars(), 15.0);
    }

    #[test]
    fn test_user_repository_save() {
        let mut repo = UserRepository::new();
        let user = User {
            id: 0,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        
        let id = repo.save(user);
        assert_eq!(id, 1);
        assert!(repo.find_by_id(id).is_some());
    }

    #[test]
    fn test_user_repository_delete() {
        let mut repo = UserRepository::new();
        let user = User {
            id: 0,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        
        let id = repo.save(user);
        assert!(repo.delete(id));
        assert!(repo.find_by_id(id).is_none());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_panic_example() {
        assert_eq!(1, 2);
    }

    #[test]
    fn test_result_ok() {
        let result: Result<i32, String> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let result: Result<i32, String> = Err("error".to_string());
        assert!(result.is_err());
    }
}

/// Demonstrates newtype pattern
fn demonstrate_newtype() {
    println!("=== Newtype Pattern ===");
    
    let user_id = UserId::new(123);
    let product_id = ProductId::new(456);
    let price = Price::new(2999); // $29.99
    
    println!("User ID: {:?}", user_id);
    println!("Product ID: {:?}", product_id);
    println!("Price: ${:.2}", price.in_dollars());
    
    // Type safety prevents mixing up IDs
    // This would not compile:
    // let mixed: UserId = product_id;
}

/// Demonstrates builder pattern with type states
fn demonstrate_builder_pattern() {
    println!("\n=== Builder Pattern with Type States ===");
    
    let email = builder_pattern::EmailBuilder::new()
        .from("sender@example.com".to_string())
        .to("receiver@example.com".to_string())
        .subject("Hello".to_string())
        .body("This is a test email".to_string())
        .build();
    
    email.send();
}

/// Demonstrates strategy pattern
fn demonstrate_strategy_pattern() {
    println!("\n=== Strategy Pattern ===");
    
    let credit_card = Box::new(CreditCardPayment {
        card_number: "1234567890123456".to_string(),
    });
    
    let mut processor = PaymentProcessor::new(credit_card);
    
    match processor.process(5000) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
    
    let paypal = Box::new(PayPalPayment {
        email: "user@example.com".to_string(),
    });
    
    processor.change_strategy(paypal);
    
    match processor.process(3000) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
}

/// Demonstrates state pattern
fn demonstrate_state_pattern() {
    println!("\n=== State Pattern ===");
    
    let mut machine = StateMachine::new();
    println!("Current state: {}", machine.current_state());
    
    machine.transition();
    println!("Current state: {}", machine.current_state());
    
    machine.transition();
    println!("Current state: {}", machine.current_state());
    
    machine.transition();
    println!("Current state: {}", machine.current_state());
}

/// Demonstrates visitor pattern
fn demonstrate_visitor_pattern() {
    println!("\n=== Visitor Pattern ===");
    
    let values = vec![
        Value::Number(42),
        Value::Text("Hello".to_string()),
        Value::Boolean(true),
        Value::Number(100),
    ];
    
    let mut print_visitor = PrintVisitor;
    println!("Printing values:");
    for value in &values {
        value.accept(&mut print_visitor);
    }
    
    let mut count_visitor = CountVisitor::new();
    for value in &values {
        value.accept(&mut count_visitor);
    }
    println!("Total values: {}", count_visitor.total());
}

/// Demonstrates observer pattern
fn demonstrate_observer_pattern() {
    println!("\n=== Observer Pattern ===");
    
    let mut subject = Subject::new();
    
    subject.attach(Box::new(ConcreteObserver::new("Observer1".to_string())));
    subject.attach(Box::new(ConcreteObserver::new("Observer2".to_string())));
    
    subject.set_state("New state!".to_string());
}

/// Demonstrates repository pattern
fn demonstrate_repository_pattern() {
    println!("\n=== Repository Pattern ===");
    
    let mut repo = UserRepository::new();
    
    let user1 = User {
        id: 0,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    let user2 = User {
        id: 0,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };
    
    let id1 = repo.save(user1);
    let id2 = repo.save(user2);
    
    println!("Saved users with IDs: {} and {}", id1, id2);
    
    if let Some(user) = repo.find_by_id(id1) {
        println!("Found user: {:?}", user);
    }
    
    println!("All users: {}", repo.find_all().len());
    
    repo.delete(id1);
    println!("After deletion: {} users", repo.find_all().len());
}

/// Main function demonstrating all patterns
fn main() {
    println!("=== Rust Advanced Patterns Demo ===\n");

    demonstrate_newtype();
    demonstrate_builder_pattern();
    demonstrate_strategy_pattern();
    demonstrate_state_pattern();
    demonstrate_visitor_pattern();
    demonstrate_observer_pattern();
    demonstrate_repository_pattern();

    println!("\n=== Running Tests ===");
    println!("Run 'cargo test' or 'rustc --test' to execute unit tests");

    println!("\n=== Demo Complete ===");
}