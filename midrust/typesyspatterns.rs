// File 15: Generics, Type System & Design Patterns
// Phantom types, type-state pattern, newtype idiom, RAII, builder, strategy,
// command, decorator, and more — all idiomatic Rust

use std::fmt;
use std::marker::PhantomData;
use std::collections::HashMap;

// ─── Phantom Types (compile-time type tagging) ────────────────────────────────

// Units that cannot be mixed up at compile time
struct Meters;
struct Feet;
struct Kilograms;
struct Pounds;

#[derive(Debug, Clone, Copy)]
struct Measurement<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Measurement<Unit> {
    fn new(value: f64) -> Self {
        Measurement { value, _unit: PhantomData }
    }

    fn value(&self) -> f64 { self.value }
}

impl Measurement<Meters> {
    fn to_feet(self) -> Measurement<Feet> {
        Measurement::new(self.value * 3.28084)
    }
}

impl Measurement<Feet> {
    fn to_meters(self) -> Measurement<Meters> {
        Measurement::new(self.value / 3.28084)
    }
}

impl Measurement<Kilograms> {
    fn to_pounds(self) -> Measurement<Pounds> {
        Measurement::new(self.value * 2.20462)
    }
}

impl<U> fmt::Display for Measurement<U>
where U: UnitLabel
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4} {}", self.value, U::label())
    }
}

trait UnitLabel { fn label() -> &'static str; }
impl UnitLabel for Meters    { fn label() -> &'static str { "m" } }
impl UnitLabel for Feet      { fn label() -> &'static str { "ft" } }
impl UnitLabel for Kilograms { fn label() -> &'static str { "kg" } }
impl UnitLabel for Pounds    { fn label() -> &'static str { "lb" } }

// ─── Type-State Pattern (state encoded in the type) ───────────────────────────

struct Draft;
struct Submitted;
struct Approved;
struct Rejected;

#[derive(Debug)]
struct Application<State> {
    id: u32,
    name: String,
    notes: Vec<String>,
    _state: PhantomData<State>,
}

impl Application<Draft> {
    fn new(id: u32, name: String) -> Self {
        Application { id, name, notes: Vec::new(), _state: PhantomData }
    }

    fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    // submit() consumes Draft, produces Submitted
    fn submit(self) -> Application<Submitted> {
        println!("Application {} submitted", self.id);
        Application { id: self.id, name: self.name, notes: self.notes, _state: PhantomData }
    }
}

impl Application<Submitted> {
    fn approve(self, reason: String) -> Application<Approved> {
        println!("Application {} approved: {}", self.id, reason);
        let mut notes = self.notes;
        notes.push(format!("Approved: {}", reason));
        Application { id: self.id, name: self.name, notes, _state: PhantomData }
    }

    fn reject(self, reason: String) -> Application<Rejected> {
        println!("Application {} rejected: {}", self.id, reason);
        let mut notes = self.notes;
        notes.push(format!("Rejected: {}", reason));
        Application { id: self.id, name: self.name, notes, _state: PhantomData }
    }
}

impl Application<Approved> {
    fn certificate(&self) -> String {
        format!("APPROVED CERTIFICATE — {} (ID: {})", self.name, self.id)
    }
}

impl Application<Rejected> {
    fn appeal_message(&self) -> String {
        format!("Appeal for {} (ID: {}): {:?}", self.name, self.id, self.notes)
    }
}

// ─── RAII — Resource Acquisition Is Initialization ───────────────────────────

struct ConnectionPool {
    name: String,
    max: usize,
    current: usize,
}

impl ConnectionPool {
    fn new(name: &str, max: usize) -> Self {
        println!("[Pool '{}'] opened (max={})", name, max);
        ConnectionPool { name: name.to_string(), max, current: 0 }
    }

    fn acquire(&mut self) -> Option<PoolGuard> {
        if self.current < self.max {
            self.current += 1;
            println!("[Pool '{}'] connection acquired ({}/{})", self.name, self.current, self.max);
            Some(PoolGuard { pool_name: self.name.clone(), id: self.current })
        } else {
            println!("[Pool '{}'] exhausted!", self.name);
            None
        }
    }

    fn release(&mut self) {
        if self.current > 0 {
            self.current -= 1;
            println!("[Pool] connection released ({}/{})", self.current, self.max);
        }
    }
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        println!("[Pool '{}'] closed ({} leaked)", self.name, self.current);
    }
}

struct PoolGuard {
    pool_name: String,
    id: usize,
}

impl PoolGuard {
    fn query(&self, sql: &str) -> String {
        format!("[Conn {}@{}] result of: {}", self.id, self.pool_name, sql)
    }
}

impl Drop for PoolGuard {
    fn drop(&mut self) {
        println!("[Conn {}] returned to pool '{}'", self.id, self.pool_name);
    }
}

// ─── Decorator Pattern ────────────────────────────────────────────────────────

trait DataSource {
    fn read(&self) -> String;
    fn write(&mut self, data: &str);
}

struct FileDataSource {
    filename: String,
    content: String,
}

impl FileDataSource {
    fn new(filename: &str) -> Self {
        FileDataSource { filename: filename.to_string(), content: String::new() }
    }
}

impl DataSource for FileDataSource {
    fn read(&self) -> String {
        println!("  [File] reading from '{}'", self.filename);
        self.content.clone()
    }

    fn write(&mut self, data: &str) {
        println!("  [File] writing to '{}'", self.filename);
        self.content = data.to_string();
    }
}

struct EncryptionDecorator<T: DataSource> {
    inner: T,
}

impl<T: DataSource> EncryptionDecorator<T> {
    fn new(inner: T) -> Self { EncryptionDecorator { inner } }

    fn encrypt(data: &str) -> String {
        data.chars().map(|c| ((c as u8 ^ 42) as char)).collect()
    }

    fn decrypt(data: &str) -> String {
        Self::encrypt(data) // XOR is symmetric
    }
}

impl<T: DataSource> DataSource for EncryptionDecorator<T> {
    fn read(&self) -> String {
        println!("  [Encrypt] decrypting...");
        Self::decrypt(&self.inner.read())
    }

    fn write(&mut self, data: &str) {
        println!("  [Encrypt] encrypting...");
        self.inner.write(&Self::encrypt(data));
    }
}

struct CompressionDecorator<T: DataSource> {
    inner: T,
}

impl<T: DataSource> CompressionDecorator<T> {
    fn new(inner: T) -> Self { CompressionDecorator { inner } }

    fn compress(data: &str) -> String {
        // Fake compression: just mark it
        format!("COMPRESSED({})", data)
    }

    fn decompress(data: &str) -> String {
        data.trim_start_matches("COMPRESSED(").trim_end_matches(')').to_string()
    }
}

impl<T: DataSource> DataSource for CompressionDecorator<T> {
    fn read(&self) -> String {
        println!("  [Compress] decompressing...");
        Self::decompress(&self.inner.read())
    }

    fn write(&mut self, data: &str) {
        println!("  [Compress] compressing...");
        self.inner.write(&Self::compress(data));
    }
}

// ─── Command Pattern with Undo ────────────────────────────────────────────────

trait Command: fmt::Debug {
    fn execute(&mut self, state: &mut Vec<String>);
    fn undo(&mut self, state: &mut Vec<String>);
    fn name(&self) -> &str;
}

#[derive(Debug)]
struct AppendCommand { text: String }

#[derive(Debug)]
struct DeleteLastCommand { deleted: Option<String> }

#[derive(Debug)]
struct ClearCommand { backup: Vec<String> }

impl Command for AppendCommand {
    fn execute(&mut self, state: &mut Vec<String>) { state.push(self.text.clone()); }
    fn undo   (&mut self, state: &mut Vec<String>) { state.pop(); }
    fn name   (&self) -> &str { "Append" }
}

impl Command for DeleteLastCommand {
    fn execute(&mut self, state: &mut Vec<String>) { self.deleted = state.pop(); }
    fn undo   (&mut self, state: &mut Vec<String>) {
        if let Some(ref d) = self.deleted { state.push(d.clone()); }
    }
    fn name   (&self) -> &str { "DeleteLast" }
}

impl Command for ClearCommand {
    fn execute(&mut self, state: &mut Vec<String>) {
        self.backup = state.clone();
        state.clear();
    }
    fn undo(&mut self, state: &mut Vec<String>) { *state = self.backup.clone(); }
    fn name(&self) -> &str { "Clear" }
}

struct CommandHistory {
    state:   Vec<String>,
    history: Vec<Box<dyn Command>>,
}

impl CommandHistory {
    fn new() -> Self { CommandHistory { state: Vec::new(), history: Vec::new() } }

    fn execute(&mut self, mut cmd: Box<dyn Command>) {
        println!("  exec: {}", cmd.name());
        cmd.execute(&mut self.state);
        self.history.push(cmd);
    }

    fn undo(&mut self) {
        if let Some(mut cmd) = self.history.pop() {
            println!("  undo: {}", cmd.name());
            cmd.undo(&mut self.state);
        }
    }

    fn state(&self) -> &[String] { &self.state }
}

// ─── Generic Event System ─────────────────────────────────────────────────────

trait EventHandler<E>: fmt::Debug {
    fn handle(&mut self, event: &E);
}

struct EventBus<E> {
    handlers: Vec<Box<dyn EventHandler<E>>>,
}

impl<E: fmt::Debug> EventBus<E> {
    fn new() -> Self { EventBus { handlers: Vec::new() } }

    fn subscribe(&mut self, handler: Box<dyn EventHandler<E>>) {
        self.handlers.push(handler);
    }

    fn publish(&mut self, event: E) {
        println!("  Event: {:?}", event);
        for h in &mut self.handlers {
            h.handle(&event);
        }
    }
}

#[derive(Debug)]
enum UserEvent {
    Created { username: String },
    Deleted { id: u32 },
    LoggedIn { username: String },
}

#[derive(Debug)]
struct AuditLogger { log: Vec<String> }

#[derive(Debug)]
struct EmailNotifier { sent: Vec<String> }

impl AuditLogger {
    fn new() -> Self { AuditLogger { log: Vec::new() } }
}

impl EmailNotifier {
    fn new() -> Self { EmailNotifier { sent: Vec::new() } }
}

impl EventHandler<UserEvent> for AuditLogger {
    fn handle(&mut self, event: &UserEvent) {
        let entry = format!("AUDIT: {:?}", event);
        println!("    [AuditLogger] {}", entry);
        self.log.push(entry);
    }
}

impl EventHandler<UserEvent> for EmailNotifier {
    fn handle(&mut self, event: &UserEvent) {
        match event {
            UserEvent::Created { username } => {
                let msg = format!("Welcome email → {}", username);
                println!("    [EmailNotifier] {}", msg);
                self.sent.push(msg);
            }
            _ => {}
        }
    }
}

// ─── Generic Repository with Query ───────────────────────────────────────────

trait Entity {
    fn id(&self) -> u32;
}

struct InMemoryRepo<T: Entity + Clone> {
    data: HashMap<u32, T>,
    next_id: u32,
}

impl<T: Entity + Clone + fmt::Debug> InMemoryRepo<T> {
    fn new() -> Self { InMemoryRepo { data: HashMap::new(), next_id: 1 } }

    fn insert(&mut self, mut_fn: impl FnOnce(u32) -> T) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let entity = mut_fn(id);
        self.data.insert(id, entity);
        id
    }

    fn get(&self, id: u32) -> Option<&T> { self.data.get(&id) }

    fn find_all<F: Fn(&T) -> bool>(&self, predicate: F) -> Vec<&T> {
        let mut results: Vec<&T> = self.data.values().filter(|e| predicate(e)).collect();
        results.sort_by_key(|e| e.id());
        results
    }

    fn update<F: FnOnce(&mut T)>(&mut self, id: u32, f: F) -> bool {
        if let Some(entity) = self.data.get_mut(&id) {
            f(entity);
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: u32) -> Option<T> { self.data.remove(&id) }
    fn count(&self) -> usize { self.data.len() }
}

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    in_stock: bool,
}

impl Entity for Product {
    fn id(&self) -> u32 { self.id }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Generics, Type System & Design Patterns ===\n");

    // Phantom types
    println!("── Phantom Types ──");
    let height = Measurement::<Meters>::new(1.85);
    let weight = Measurement::<Kilograms>::new(80.0);
    println!("Height: {}", height);
    println!("       = {}", height.to_feet());
    println!("Weight: {}", weight);
    println!("       = {}", weight.to_pounds());
    // Measurement<Meters> and Measurement<Feet> are different types — can't mix them

    // Type-state pattern
    println!("\n── Type-State Pattern ──");
    let mut app = Application::<Draft>::new(1001, "Alice's Application".to_string());
    app.add_note("Submitted via online portal".to_string());
    let submitted = app.submit();
    let approved  = submitted.approve("Strong candidate".to_string());
    println!("{}", approved.certificate());

    let app2 = Application::<Draft>::new(1002, "Bob's Application".to_string());
    let sub2  = app2.submit();
    let rej   = sub2.reject("Insufficient experience".to_string());
    println!("{}", rej.appeal_message());

    // RAII
    println!("\n── RAII (ConnectionPool) ──");
    {
        let mut pool = ConnectionPool::new("main-db", 2);
        let conn1 = pool.acquire();
        let conn2 = pool.acquire();
        let conn3 = pool.acquire(); // should fail
        if let Some(ref c) = conn1 {
            println!("{}", c.query("SELECT 1"));
        }
        drop(conn3); drop(conn2); drop(conn1);
        // pool.release is called via PoolGuard::drop
    } // pool drops here

    // Decorator
    println!("\n── Decorator Pattern ──");
    let file   = FileDataSource::new("data.bin");
    let enc    = EncryptionDecorator::new(file);
    let mut decorated = CompressionDecorator::new(enc);

    decorated.write("Hello, layered world!");
    let read_back = decorated.read();
    println!("Read back: '{}'", read_back);

    // Command + Undo
    println!("\n── Command Pattern (with Undo) ──");
    let mut history = CommandHistory::new();
    history.execute(Box::new(AppendCommand { text: "first".to_string() }));
    history.execute(Box::new(AppendCommand { text: "second".to_string() }));
    history.execute(Box::new(AppendCommand { text: "third".to_string() }));
    println!("  state: {:?}", history.state());
    history.undo();
    println!("  after undo: {:?}", history.state());
    history.execute(Box::new(ClearCommand { backup: vec![] }));
    println!("  after clear: {:?}", history.state());
    history.undo();
    println!("  after undo clear: {:?}", history.state());

    // Event bus
    println!("\n── Event Bus ──");
    let mut bus: EventBus<UserEvent> = EventBus::new();
    bus.subscribe(Box::new(AuditLogger::new()));
    bus.subscribe(Box::new(EmailNotifier::new()));
    bus.publish(UserEvent::Created { username: "alice".to_string() });
    bus.publish(UserEvent::LoggedIn { username: "alice".to_string() });
    bus.publish(UserEvent::Deleted { id: 42 });

    // Generic repository
    println!("\n── Generic Repository ──");
    let mut repo: InMemoryRepo<Product> = InMemoryRepo::new();

    repo.insert(|id| Product { id, name: "Keyboard".to_string(),  price: 79.99,  in_stock: true  });
    repo.insert(|id| Product { id, name: "Monitor".to_string(),   price: 399.99, in_stock: false });
    repo.insert(|id| Product { id, name: "Mouse".to_string(),     price: 49.99,  in_stock: true  });
    repo.insert(|id| Product { id, name: "Headphones".to_string(),price: 199.99, in_stock: true  });

    println!("Total products: {}", repo.count());

    let in_stock = repo.find_all(|p| p.in_stock);
    println!("In-stock:");
    for p in &in_stock { println!("  {:?}", p); }

    let expensive = repo.find_all(|p| p.price > 100.0);
    println!("Expensive (>$100):");
    for p in &expensive { println!("  {} — ${:.2}", p.name, p.price); }

    repo.update(1, |p| p.price = 69.99);
    println!("Updated keyboard: {:?}", repo.get(1));

    repo.delete(2);
    println!("After delete, count: {}", repo.count());

    println!("\n=== Done ===");
}