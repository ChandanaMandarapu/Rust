// File 18: Advanced Concurrency Patterns
// Atomics, lock-free data structures, custom channels, work-stealing,
// thread-local storage, and concurrent algorithms

use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex, Condvar, Barrier};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::ptr;
use std::cell::Cell;

// ─── Atomics Deep Dive ────────────────────────────────────────────────────────

fn demonstrate_atomics() {
    println!("── Atomics ──");

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // 8 threads all incrementing concurrently
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // SeqCst: strongest ordering, no reordering allowed
                c.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("8 threads × 1000 increments = {}", counter.load(Ordering::SeqCst));

    // Compare-and-swap based counter (CAS loop)
    let cas_counter = Arc::new(AtomicUsize::new(0));
    {
        let c = Arc::clone(&cas_counter);
        let h = thread::spawn(move || {
            for _ in 0..100 {
                let mut old = c.load(Ordering::Relaxed);
                loop {
                    // Only update if the value hasn't changed
                    match c.compare_exchange_weak(old, old + 1, Ordering::AcqRel, Ordering::Relaxed) {
                        Ok(_) => break,
                        Err(current) => old = current,
                    }
                }
            }
        });
        h.join().unwrap();
    }
    println!("CAS counter: {}", cas_counter.load(Ordering::SeqCst));

    // fetch_max / fetch_min
    let max = AtomicUsize::new(0);
    for n in [5, 2, 9, 1, 7] {
        max.fetch_max(n, Ordering::Relaxed);
    }
    println!("Atomic max: {}", max.load(Ordering::Relaxed));

    // Ordering demonstration
    let flag  = AtomicBool::new(false);
    let value = AtomicUsize::new(0);

    value.store(42, Ordering::Relaxed);
    // Release: all previous writes visible to any thread that Acquires this
    flag.store(true, Ordering::Release);

    // Acquire: ensures we see all writes before the Release
    if flag.load(Ordering::Acquire) {
        println!("Acquire saw value: {}", value.load(Ordering::Relaxed));
    }
}

// ─── Lock-Free Stack ──────────────────────────────────────────────────────────

struct Node<T> {
    value: T,
    next:  *mut Node<T>,
}

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

// SAFETY: LockFreeStack is safe to share across threads because
// all operations use atomic CAS to prevent data races
unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        LockFreeStack { head: AtomicPtr::new(ptr::null_mut()) }
    }

    fn push(&self, value: T) {
        let node = Box::into_raw(Box::new(Node { value, next: ptr::null_mut() }));
        loop {
            let old_head = self.head.load(Ordering::Relaxed);
            unsafe {
                // SAFETY: node is freshly allocated and exclusively owned
                (*node).next = old_head;
            }
            // CAS: set head to new node only if it hasn't changed
            match self.head.compare_exchange_weak(old_head, node, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => break,
                Err(_) => { /* retry */ }
            }
        }
    }

    fn pop(&self) -> Option<T> {
        loop {
            let old_head = self.head.load(Ordering::Acquire);
            if old_head.is_null() { return None; }
            unsafe {
                // SAFETY: old_head was loaded under Acquire, valid Node
                let next = (*old_head).next;
                match self.head.compare_exchange_weak(old_head, next, Ordering::Release, Ordering::Relaxed) {
                    Ok(_) => {
                        let node = Box::from_raw(old_head);
                        return Some(node.value);
                    }
                    Err(_) => { /* retry */ }
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed).is_null()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

// ─── Bounded SPSC Queue (Single Producer, Single Consumer) ───────────────────

struct RingBuffer<T, const N: usize> {
    data:  std::mem::MaybeUninit<[T; N]>,
    head:  AtomicUsize,  // consumer reads from here
    tail:  AtomicUsize,  // producer writes here
}

// SAFETY: we ensure exclusive access through atomic head/tail
unsafe impl<T: Send, const N: usize> Send for RingBuffer<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for RingBuffer<T, N> {}

impl<T, const N: usize> RingBuffer<T, N> {
    fn new() -> Self {
        RingBuffer {
            data: std::mem::MaybeUninit::uninit(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    fn push(&self, value: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % N;
        if next_tail == self.head.load(Ordering::Acquire) {
            return false; // full
        }
        unsafe {
            // SAFETY: tail slot is not occupied (verified above)
            let slot = (self.data.as_ptr() as *mut T).add(tail);
            slot.write(value);
        }
        self.tail.store(next_tail, Ordering::Release);
        true
    }

    fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        if head == self.tail.load(Ordering::Acquire) {
            return None; // empty
        }
        let value = unsafe {
            // SAFETY: head slot was written by producer (verified via tail)
            let slot = (self.data.as_ptr() as *mut T).add(head);
            slot.read()
        };
        self.head.store((head + 1) % N, Ordering::Release);
        Some(value)
    }

    fn len(&self) -> usize {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        (tail + N - head) % N
    }
}

impl<T, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

// ─── Custom Channel ───────────────────────────────────────────────────────────

struct Channel<T> {
    queue:  Arc<Mutex<std::collections::VecDeque<T>>>,
    cv:     Arc<Condvar>,
    closed: Arc<AtomicBool>,
}

struct Sender<T>   { inner: Channel<T> }
struct Receiver<T> { inner: Channel<T> }

fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let ch = Channel {
        queue:  Arc::new(Mutex::new(std::collections::VecDeque::new())),
        cv:     Arc::new(Condvar::new()),
        closed: Arc::new(AtomicBool::new(false)),
    };
    let rx = Channel {
        queue:  Arc::clone(&ch.queue),
        cv:     Arc::clone(&ch.cv),
        closed: Arc::clone(&ch.closed),
    };
    (Sender { inner: ch }, Receiver { inner: rx })
}

impl<T> Sender<T> {
    fn send(&self, value: T) -> bool {
        if self.inner.closed.load(Ordering::SeqCst) { return false; }
        self.inner.queue.lock().unwrap().push_back(value);
        self.inner.cv.notify_one();
        true
    }

    fn close(&self) {
        self.inner.closed.store(true, Ordering::SeqCst);
        self.inner.cv.notify_all();
    }
}

impl<T> Receiver<T> {
    fn recv(&self) -> Option<T> {
        let mut q = self.inner.queue.lock().unwrap();
        loop {
            if let Some(v) = q.pop_front() { return Some(v); }
            if self.inner.closed.load(Ordering::SeqCst) { return None; }
            q = self.inner.cv.wait(q).unwrap();
        }
    }

    fn try_recv(&self) -> Option<T> {
        self.inner.queue.lock().unwrap().pop_front()
    }
}

// ─── Work-Stealing Thread Pool ────────────────────────────────────────────────

type Task = Box<dyn FnOnce() + Send + 'static>;

struct WorkStealer {
    queues: Arc<Vec<Mutex<std::collections::VecDeque<Task>>>>,
}

impl WorkStealer {
    fn new(workers: usize) -> (Self, Vec<thread::JoinHandle<()>>) {
        let queues = Arc::new((0..workers).map(|_| Mutex::new(std::collections::VecDeque::new())).collect::<Vec<_>>());
        let done   = Arc::new(AtomicBool::new(false));
        let mut handles = vec![];

        for id in 0..workers {
            let qs   = Arc::clone(&queues);
            let done = Arc::clone(&done);

            let h = thread::spawn(move || {
                while !done.load(Ordering::Relaxed) {
                    // Try own queue first
                    let task = qs[id].lock().unwrap().pop_front();
                    if let Some(t) = task { t(); continue; }

                    // Steal from others
                    let stolen = (0..qs.len()).filter(|&i| i != id).find_map(|i| {
                        qs[i].lock().unwrap().pop_back() // steal from back
                    });
                    if let Some(t) = stolen { t(); continue; }

                    // Nothing to do
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(h);
        }

        (WorkStealer { queues }, handles)
    }

    fn submit(&self, worker: usize, task: Task) {
        let idx = worker % self.queues.len();
        self.queues[idx].lock().unwrap().push_back(task);
    }
}

// ─── Thread-Local Storage ────────────────────────────────────────────────────

thread_local! {
    static THREAD_ID: Cell<u32>     = Cell::new(0);
    static CALL_COUNT: Cell<u32>    = Cell::new(0);
}

fn register_thread(id: u32) {
    THREAD_ID.with(|tid| tid.set(id));
}

fn increment_calls() -> u32 {
    CALL_COUNT.with(|c| { c.set(c.get() + 1); c.get() })
}

fn thread_stats() -> (u32, u32) {
    (
        THREAD_ID.with(|t| t.get()),
        CALL_COUNT.with(|c| c.get()),
    )
}

fn demonstrate_thread_local() {
    println!("\n── Thread-Local Storage ──");

    let mut handles = vec![];
    for id in 0..4 {
        let h = thread::spawn(move || {
            register_thread(id);
            for _ in 0..5 { increment_calls(); }
            let (tid, calls) = thread_stats();
            println!("  thread_id={}, calls={}", tid, calls);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
}

// ─── Barrier + Phase Synchronization ─────────────────────────────────────────

fn demonstrate_barrier_phases() {
    println!("\n── Barrier Phase Sync ──");

    let n = 4;
    let barrier   = Arc::new(Barrier::new(n));
    let results   = Arc::new(Mutex::new(vec![0u32; n]));
    let mut handles = vec![];

    for id in 0..n {
        let b = Arc::clone(&barrier);
        let r = Arc::clone(&results);

        let h = thread::spawn(move || {
            // Phase 1: compute
            let val = (id as u32 + 1) * 10;
            println!("  [{}] computed {}", id, val);
            r.lock().unwrap()[id] = val;

            b.wait(); // sync — all finish phase 1 before phase 2

            // Phase 2: read others' results (all results are now visible)
            let sum: u32 = r.lock().unwrap().iter().sum();
            println!("  [{}] sees total = {}", id, sum);

            b.wait(); // sync again
            println!("  [{}] done", id);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
}

// ─── Pipeline Pattern ─────────────────────────────────────────────────────────

fn pipeline_stage<T, U, F>(rx: mpsc::Receiver<T>, tx: mpsc::Sender<U>, f: F)
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    thread::spawn(move || {
        for item in rx { let _ = tx.send(f(item)); }
    });
}

fn demonstrate_pipeline() {
    println!("\n── Thread Pipeline ──");

    let (tx0, rx0) = mpsc::channel::<i32>();
    let (tx1, rx1) = mpsc::channel::<i32>();
    let (tx2, rx2) = mpsc::channel::<String>();

    // Stage 1: double
    pipeline_stage(rx0, tx1, |x| x * 2);
    // Stage 2: stringify
    pipeline_stage(rx1, tx2, |x| format!("val={}", x + 1));

    for i in 1..=5 { tx0.send(i).unwrap(); }
    drop(tx0);

    for result in rx2 {
        println!("  pipeline: {}", result);
    }
}

// ─── Reader-Writer with priority ─────────────────────────────────────────────

struct PriorityRwLock<T> {
    inner:     Arc<Mutex<T>>,
    readers:   Arc<AtomicUsize>,
    write_req: Arc<AtomicBool>,
    cv:        Arc<Condvar>,
}

impl<T: Send + 'static> PriorityRwLock<T> {
    fn new(data: T) -> Self {
        PriorityRwLock {
            inner:     Arc::new(Mutex::new(data)),
            readers:   Arc::new(AtomicUsize::new(0)),
            write_req: Arc::new(AtomicBool::new(false)),
            cv:        Arc::new(Condvar::new()),
        }
    }

    fn read_with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        // Respect write requests — don't starve writers
        while self.write_req.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(1));
        }
        self.readers.fetch_add(1, Ordering::SeqCst);
        let result = f(&*self.inner.lock().unwrap());
        self.readers.fetch_sub(1, Ordering::SeqCst);
        self.cv.notify_all();
        result
    }

    fn write_with<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        self.write_req.store(true, Ordering::SeqCst);
        // Wait for readers to finish
        while self.readers.load(Ordering::SeqCst) > 0 {
            thread::sleep(Duration::from_millis(1));
        }
        let result = f(&mut *self.inner.lock().unwrap());
        self.write_req.store(false, Ordering::SeqCst);
        self.cv.notify_all();
        result
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Advanced Concurrency ===\n");

    demonstrate_atomics();

    // Lock-free stack
    println!("\n── Lock-Free Stack ──");
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];

    for i in 0..5 {
        let s = Arc::clone(&stack);
        handles.push(thread::spawn(move || { s.push(i * 10); }));
    }
    for h in handles { h.join().unwrap(); }

    println!("popped:");
    while let Some(v) = stack.pop() { print!("{} ", v); }
    println!();

    // Ring buffer SPSC
    println!("\n── SPSC Ring Buffer ──");
    let buf = Arc::new(RingBuffer::<i32, 8>::new());
    let buf2 = Arc::clone(&buf);

    let producer = thread::spawn(move || {
        for i in 0..6 {
            while !buf2.push(i * 11) { thread::yield_now(); }
        }
    });
    producer.join().unwrap();

    println!("len: {}", buf.len());
    while let Some(v) = buf.pop() { print!("{} ", v); }
    println!();

    // Custom channel
    println!("\n── Custom Channel ──");
    let (tx, rx) = channel::<String>();

    let producer = thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("message {}", i));
            thread::sleep(Duration::from_millis(10));
        }
        tx.close();
    });

    while let Some(msg) = rx.recv() {
        println!("  recv: {}", msg);
    }
    producer.join().unwrap();

    // Work-stealing pool
    println!("\n── Work-Stealing Pool ──");
    let results = Arc::new(Mutex::new(vec![]));
    let done    = Arc::new(AtomicBool::new(false));

    {
        let (pool, handles) = WorkStealer::new(3);
        for i in 0..9 {
            let r = Arc::clone(&results);
            pool.submit(i % 3, Box::new(move || {
                let val = i * i;
                r.lock().unwrap().push(val);
            }));
        }
        // Let workers drain
        thread::sleep(Duration::from_millis(100));
        done.store(true, Ordering::Relaxed);
        for h in handles { let _ = h.join(); }
    }

    let mut r = results.lock().unwrap();
    r.sort();
    println!("  results: {:?}", *r);

    demonstrate_thread_local();
    demonstrate_barrier_phases();
    demonstrate_pipeline();

    // Priority RwLock
    println!("\n── Priority RwLock ──");
    let lock = Arc::new(PriorityRwLock::new(0i32));
    {
        let l = Arc::clone(&lock);
        let rh = thread::spawn(move || {
            l.read_with(|v| println!("  reader sees: {}", v))
        });
        lock.write_with(|v| { *v = 42; println!("  writer set: {}", *v); });
        rh.join().unwrap();
    }
    lock.read_with(|v| println!("  final: {}", v));

    println!("\n=== Done ===");
}