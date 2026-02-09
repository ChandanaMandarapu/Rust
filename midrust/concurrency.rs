//concurrency and Threading

use std::sync::{Arc, Mutex, RwLock, Barrier, Condvar, mpsc};
use std::thread;
use std::time::Duration;

/// Demonstrates basic thread creation and joining
fn demonstrate_basic_threads() {
    println!("=== Basic Threading ===");
    
    // Spawning a simple thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: count {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Main thread also doing work
    for i in 1..=5 {
        println!("Main: count {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    // Wait for spawned thread to finish
    handle.join().unwrap();
    println!("Thread completed");
}

/// Demonstrates thread with return values
fn demonstrate_thread_return() {
    println!("\n=== Thread Return Values ===");
    
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum
    });
    
    let result = handle.join().unwrap();
    println!("Sum from thread: {}", result);
}

/// Demonstrates moving data into threads
fn demonstrate_move_closure() {
    println!("\n=== Moving Data into Threads ===");
    
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        println!("Thread received data: {:?}", data);
        let sum: i32 = data.iter().sum();
        sum
    });
    
    let sum = handle.join().unwrap();
    println!("Sum computed in thread: {}", sum);
}

/// Demonstrates multiple threads
fn demonstrate_multiple_threads() {
    println!("\n=== Multiple Threads ===");
    
    let mut handles = vec![];
    
    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(100 * i as u64));
            println!("Thread {} finishing", i);
            i * i
        });
        handles.push(handle);
    }
    
    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    println!("Results: {:?}", results);
}

/// Demonstrates Mutex for shared mutable state
fn demonstrate_mutex() {
    println!("\n=== Mutex ===");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}

/// A thread-safe bank account using Mutex
struct BankAccount {
    balance: Arc<Mutex<f64>>,
}

impl BankAccount {
    /// Creates a new bank account
    fn new(initial_balance: f64) -> Self {
        BankAccount {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    /// Deposits money
    fn deposit(&self, amount: f64) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        println!("Deposited ${:.2}, new balance: ${:.2}", amount, *balance);
    }

    /// Withdraws money
    fn withdraw(&self, amount: f64) -> bool {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            println!("Withdrew ${:.2}, new balance: ${:.2}", amount, *balance);
            true
        } else {
            println!("Insufficient funds for ${:.2} withdrawal", amount);
            false
        }
    }

    /// Returns the current balance
    fn get_balance(&self) -> f64 {
        *self.balance.lock().unwrap()
    }

    /// Clones the account reference
    fn clone_ref(&self) -> Self {
        BankAccount {
            balance: Arc::clone(&self.balance),
        }
    }
}

/// Demonstrates RwLock for read-write locks
fn demonstrate_rwlock() {
    println!("\n=== RwLock ===");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Spawn reader threads
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_data = data_clone.read().unwrap();
            println!("Reader {}: {:?}", i, *read_data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // Spawn writer thread
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut write_data = data_clone.write().unwrap();
        write_data.push(4);
        println!("Writer: Added element");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", *data.read().unwrap());
}

/// Demonstrates message passing with channels
fn demonstrate_channels() {
    println!("\n=== Channels (Message Passing) ===");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("Received: {}", received);
    }
}

/// Demonstrates multiple producers with channels
fn demonstrate_multiple_producers() {
    println!("\n=== Multiple Producers ===");
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    for i in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("Producer {} sends message {}", i, j);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    drop(tx); // Drop original sender
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    for received in rx {
        println!("Received: {}", received);
    }
}

/// A thread pool implementation
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            
            match job {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// Creates a new thread pool
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Executes a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.clone());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Demonstrates thread pool usage
fn demonstrate_thread_pool() {
    println!("\n=== Thread Pool ===");
    
    let pool = ThreadPool::new(3);
    
    for i in 0..7 {
        pool.execute(move || {
            println!("Task {} processing", i);
            thread::sleep(Duration::from_millis(200));
            println!("Task {} completed", i);
        });
    }
    
    thread::sleep(Duration::from_secs(2));
    println!("All tasks submitted");
}

/// Demonstrates barrier synchronization
fn demonstrate_barrier() {
    println!("\n=== Barrier Synchronization ===");
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} working...", i);
            thread::sleep(Duration::from_millis(100 * i as u64));
            println!("Thread {} waiting at barrier", i);
            barrier_clone.wait();
            println!("Thread {} passed barrier", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// Demonstrates condition variable
fn demonstrate_condvar() {
    println!("\n=== Condition Variable ===");
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        let (lock, cvar) = &*pair_clone;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        println!("Thread: Data is ready");
        cvar.notify_one();
    });
    
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        println!("Main: Waiting for data...");
        ready = cvar.wait(ready).unwrap();
    }
    println!("Main: Data received!");
}

/// A producer-consumer pattern implementation
struct ProducerConsumer {
    queue: Arc<Mutex<Vec<i32>>>,
    not_empty: Arc<Condvar>,
    not_full: Arc<Condvar>,
    capacity: usize,
}

impl ProducerConsumer {
    /// Creates a new producer-consumer
    fn new(capacity: usize) -> Self {
        ProducerConsumer {
            queue: Arc::new(Mutex::new(Vec::new())),
            not_empty: Arc::new(Condvar::new()),
            not_full: Arc::new(Condvar::new()),
            capacity,
        }
    }

    /// Produces an item
    fn produce(&self, item: i32) {
        let mut queue = self.queue.lock().unwrap();
        
        while queue.len() >= self.capacity {
            queue = self.not_full.wait(queue).unwrap();
        }
        
        queue.push(item);
        println!("Produced: {}", item);
        self.not_empty.notify_one();
    }

    /// Consumes an item
    fn consume(&self) -> i32 {
        let mut queue = self.queue.lock().unwrap();
        
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        
        let item = queue.remove(0);
        println!("Consumed: {}", item);
        self.not_full.notify_one();
        item
    }

    /// Clones the reference
    fn clone_ref(&self) -> Self {
        ProducerConsumer {
            queue: Arc::clone(&self.queue),
            not_empty: Arc::clone(&self.not_empty),
            not_full: Arc::clone(&self.not_full),
            capacity: self.capacity,
        }
    }
}

/// Demonstrates scoped threads
fn demonstrate_scoped_threads() {
    println!("\n=== Scoped Threads ===");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread 1 reading data: {:?}", data);
        });
        
        s.spawn(|| {
            println!("Thread 2 reading data: {:?}", data);
        });
        
        s.spawn(|| {
            data.push(6);
            println!("Thread 3 modified data: {:?}", data);
        });
    });
    
    println!("After scoped threads: {:?}", data);
}

/// A parallel computation example
fn parallel_sum(numbers: Vec<i32>, num_threads: usize) -> i32 {
    let chunk_size = (numbers.len() + num_threads - 1) / num_threads;
    let numbers = Arc::new(numbers);
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let numbers_clone = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(numbers_clone.len());
            
            if start < numbers_clone.len() {
                let sum: i32 = numbers_clone[start..end].iter().sum();
                sum
            } else {
                0
            }
        });
        handles.push(handle);
    }
    
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// Demonstrates parallel computation
fn demonstrate_parallel_computation() {
    println!("\n=== Parallel Computation ===");
    
    let numbers: Vec<i32> = (1..=1000).collect();
    let result = parallel_sum(numbers, 4);
    println!("Parallel sum: {}", result);
}

/// A task scheduler using channels
struct TaskScheduler {
    tx: mpsc::Sender<Task>,
}

enum Task {
    Execute(Box<dyn FnOnce() + Send>),
    Shutdown,
}

impl TaskScheduler {
    /// Creates a new task scheduler
    fn new(num_workers: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        
        for i in 0..num_workers {
            let rx_clone = Arc::clone(&rx);
            thread::spawn(move || {
                loop {
                    let task = rx_clone.lock().unwrap().recv();
                    
                    match task {
                        Ok(Task::Execute(f)) => {
                            println!("Worker {} executing task", i);
                            f();
                        }
                        Ok(Task::Shutdown) | Err(_) => {
                            println!("Worker {} shutting down", i);
                            break;
                        }
                    }
                }
            });
        }
        
        TaskScheduler { tx }
    }

    /// Schedules a task
    fn schedule<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.send(Task::Execute(Box::new(f))).unwrap();
    }

    /// Shuts down the scheduler
    fn shutdown(&self) {
        self.tx.send(Task::Shutdown).unwrap();
    }
}

/// Main function demonstrating all concurrency concepts
fn main() {
    println!("=== Rust Concurrency Demo ===\n");

    demonstrate_basic_threads();
    demonstrate_thread_return();
    demonstrate_move_closure();
    demonstrate_multiple_threads();
    demonstrate_mutex();
    demonstrate_rwlock();
    demonstrate_channels();
    demonstrate_multiple_producers();
    demonstrate_barrier();
    demonstrate_condvar();
    demonstrate_scoped_threads();
    demonstrate_parallel_computation();

    // Bank account example
    println!("\n=== Bank Account Example ===");
    let account = BankAccount::new(1000.0);
    let mut handles = vec![];
    
    for _ in 0..3 {
        let account_ref = account.clone_ref();
        let handle = thread::spawn(move || {
            account_ref.deposit(100.0);
        });
        handles.push(handle);
    }
    
    for _ in 0..2 {
        let account_ref = account.clone_ref();
        let handle = thread::spawn(move || {
            account_ref.withdraw(150.0);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final balance: ${:.2}", account.get_balance());

    // Producer-consumer example
    println!("\n=== Producer-Consumer Example ===");
    let pc = ProducerConsumer::new(3);
    let mut handles = vec![];
    
    for i in 0..5 {
        let pc_clone = pc.clone_ref();
        let handle = thread::spawn(move || {
            pc_clone.produce(i);
        });
        handles.push(handle);
    }
    
    for _ in 0..5 {
        let pc_clone = pc.clone_ref();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            pc_clone.consume();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    // Thread pool example
    demonstrate_thread_pool();

    println!("\n=== Demo Complete ===");
}