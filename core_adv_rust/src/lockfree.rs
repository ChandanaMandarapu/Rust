// today practice session 3
// Built a deep dive into atomic memory ordering including acquire release relaxed seqcst semantics and fence usage with documented happens before reasoning. Implemented SeqLock with optimistic reads, Michael Scott lock free queue with CAS loops, tagged pointer to prevent ABA, simplified epoch based reclamation for deferred memory safety, and a wait free counter with per thread slots to avoid contention. Also added ticket lock for fair synchronization and multiple threaded stress tests in main to validate correctness under concurrency.

use std::sync::atomic::{
    AtomicUsize, AtomicBool, AtomicPtr, AtomicU64,
    Ordering::{self, Acquire, Release, Relaxed, SeqCst, AcqRel},
    fence,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::mem;
use std::time::Duration;
use std::cell::UnsafeCell;

// ─── Ordering Proofs (documented) ────────────────────────────────────────────

// Relaxed: no ordering, just atomicity. Use for counters with no deps.
// Acquire: all reads/writes AFTER this load cannot move BEFORE it.
// Release: all reads/writes BEFORE this store cannot move AFTER it.
// AcqRel:  both Acquire and Release in one operation (for RMW ops).
// SeqCst:  total order, most expensive. Use when you need global ordering.
//
// Pairs: Release store → Acquire load forms a happens-before edge.
// fence(SeqCst) creates a full memory barrier.

fn demonstrate_ordering() {
    println!("── Memory Ordering ──");

    // Classic acquire/release handshake
    let data  = AtomicU64::new(0);
    let ready = AtomicBool::new(false);

    let data_ref  = &data  as *const _ as usize;
    let ready_ref = &ready as *const _ as usize;

    // Producer: write data, then signal
    unsafe {
        let d = &*(data_ref as *const AtomicU64);
        let r = &*(ready_ref as *const AtomicBool);
        d.store(0xCAFE_BABE, Relaxed);    // no ordering needed — guarded by ready
        r.store(true, Release);           // Release: data write is visible before this
    }

    // Consumer: spin on ready, then read data
    unsafe {
        let d = &*(data_ref as *const AtomicU64);
        let r = &*(ready_ref as *const AtomicBool);
        while !r.load(Acquire) {}        // Acquire: all writes before Release are visible
        let val = d.load(Relaxed);       // safe to read Relaxed now
        println!("  producer/consumer: 0x{:X}", val);
    }

    // Explicit fence usage
    let x = AtomicUsize::new(0);
    let y = AtomicUsize::new(0);
    x.store(1, Relaxed);
    fence(Release);                      // equivalent to release store
    y.store(1, Relaxed);
    println!("  fence demo: x={}, y={}", x.load(Acquire), y.load(Relaxed));

    // fetch_add / fetch_sub / fetch_and / fetch_or / fetch_xor
    let bits = AtomicUsize::new(0b0000_1111);
    bits.fetch_and(0b1010_1010, AcqRel);
    println!("  after AND: 0b{:08b}", bits.load(Relaxed));
    bits.fetch_or(0b0101_0000, AcqRel);
    println!("  after OR:  0b{:08b}", bits.load(Relaxed));
    bits.fetch_xor(0b1111_0000, AcqRel);
    println!("  after XOR: 0b{:08b}", bits.load(Relaxed));
}

// ─── SeqLock (sequence lock) ──────────────────────────────────────────────────
// Optimistic read: readers don't lock, but retry if a write happened.
// Writers increment a sequence number (odd = writing, even = done).

struct SeqLock<T: Copy> {
    seq:  AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Copy + Send> Send for SeqLock<T> {}
unsafe impl<T: Copy + Send> Sync for SeqLock<T> {}

impl<T: Copy> SeqLock<T> {
    fn new(v: T) -> Self {
        SeqLock { seq: AtomicUsize::new(0), data: UnsafeCell::new(v) }
    }

    fn read(&self) -> T {
        loop {
            let s1 = self.seq.load(Acquire);
            if s1 & 1 != 0 { continue; } // writer in progress

            let val = unsafe { ptr::read_volatile(self.data.get()) };

            fence(Acquire);
            let s2 = self.seq.load(Relaxed);

            if s1 == s2 { return val; } // no write happened — value is consistent
        }
    }

    fn write(&self, v: T) {
        let old = self.seq.fetch_add(1, Release); // odd: writing
        fence(Release);
        unsafe { ptr::write_volatile(self.data.get(), v); }
        self.seq.store(old + 2, Release);         // even: done
    }
}

// ─── Michael-Scott Lock-Free Queue ────────────────────────────────────────────
// Classic 2-pointer FIFO queue. Uses sentinel node.
// Both enqueue and dequeue are non-blocking.

struct QNode<T> {
    value: Option<T>,
    next:  AtomicPtr<QNode<T>>,
}

impl<T> QNode<T> {
    fn sentinel() -> *mut Self {
        Box::into_raw(Box::new(QNode { value: None, next: AtomicPtr::new(ptr::null_mut()) }))
    }

    fn new(v: T) -> *mut Self {
        Box::into_raw(Box::new(QNode { value: Some(v), next: AtomicPtr::new(ptr::null_mut()) }))
    }
}

struct MsQueue<T> {
    head: AtomicPtr<QNode<T>>,
    tail: AtomicPtr<QNode<T>>,
}

unsafe impl<T: Send> Send for MsQueue<T> {}
unsafe impl<T: Send> Sync for MsQueue<T> {}

impl<T> MsQueue<T> {
    fn new() -> Self {
        let sentinel = QNode::sentinel();
        MsQueue {
            head: AtomicPtr::new(sentinel),
            tail: AtomicPtr::new(sentinel),
        }
    }

    fn enqueue(&self, value: T) {
        let node = QNode::new(value);
        loop {
            let tail = self.tail.load(Acquire);
            let next = unsafe { (*tail).next.load(Acquire) };

            if tail == self.tail.load(Relaxed) {
                if next.is_null() {
                    // Try to link node at the end
                    match unsafe { (*tail).next.compare_exchange(ptr::null_mut(), node, Release, Relaxed) } {
                        Ok(_) => {
                            // Try to advance tail (okay if it fails — others will help)
                            let _ = self.tail.compare_exchange(tail, node, Release, Relaxed);
                            return;
                        }
                        Err(_) => {}
                    }
                } else {
                    // Tail is lagging — help advance it
                    let _ = self.tail.compare_exchange(tail, next, Release, Relaxed);
                }
            }
        }
    }

    fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Acquire);
            let tail = self.tail.load(Acquire);
            let next = unsafe { (*head).next.load(Acquire) };

            if head == self.head.load(Relaxed) {
                if head == tail {
                    if next.is_null() { return None; } // empty
                    let _ = self.tail.compare_exchange(tail, next, Release, Relaxed);
                } else {
                    let value = unsafe { (*next).value.take() };
                    match self.head.compare_exchange(head, next, Release, Relaxed) {
                        Ok(_) => {
                            unsafe { drop(Box::from_raw(head)); }
                            return value;
                        }
                        Err(_) => {
                            // Put value back (we lost the race)
                            unsafe { (*next).value = value; }
                        }
                    }
                }
            }
        }
    }
}

impl<T> Drop for MsQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        let sentinel = self.head.load(Relaxed);
        unsafe { drop(Box::from_raw(sentinel)); }
    }
}

// ─── Epoch-Based Reclamation (simplified) ────────────────────────────────────
// Deferred deallocation: only free when no thread holds an old epoch.

struct Epoch {
    global: AtomicUsize,
    thread_epoch: Mutex<std::collections::HashMap<thread::ThreadId, usize>>,
    pending: Mutex<Vec<(usize, Box<dyn FnOnce() + Send>)>>, // (epoch, drop_fn)
}

impl Epoch {
    fn new() -> Arc<Self> {
        Arc::new(Epoch {
            global: AtomicUsize::new(0),
            thread_epoch: Mutex::new(std::collections::HashMap::new()),
            pending: Mutex::new(vec![]),
        })
    }

    // Register this thread as active in the current epoch
    fn pin(&self) -> usize {
        let e = self.global.load(SeqCst);
        self.thread_epoch.lock().unwrap().insert(thread::current().id(), e);
        fence(SeqCst);
        e
    }

    fn unpin(&self) {
        self.thread_epoch.lock().unwrap().remove(&thread::current().id());
    }

    fn advance(&self) {
        self.global.fetch_add(1, SeqCst);
        self.collect_garbage();
    }

    // Schedule something to be freed after all threads leave current epoch
    fn defer_free(&self, current_epoch: usize, f: Box<dyn FnOnce() + Send>) {
        self.pending.lock().unwrap().push((current_epoch, f));
    }

    fn collect_garbage(&self) {
        let min_epoch = {
            let epochs = self.thread_epoch.lock().unwrap();
            *epochs.values().min().unwrap_or(&usize::MAX)
        };
        let global = self.global.load(SeqCst);
        let safe_epoch = global.saturating_sub(2); // safe to free 2 epochs ago

        let mut pending = self.pending.lock().unwrap();
        let mut keep = vec![];
        for (epoch, f) in pending.drain(..) {
            if epoch <= safe_epoch.min(min_epoch) {
                f(); // safe to run
            } else {
                keep.push((epoch, f));
            }
        }
        *pending = keep;
    }
}

// ─── ABA Problem Demonstration ────────────────────────────────────────────────
// Without version tags, a CAS can succeed even though the value changed and
// changed back — making it appear nothing happened.

#[derive(Debug, Clone, Copy)]
struct TaggedPtr {
    ptr: usize,
    tag: usize, // incremented on every modification
}

struct TaggedAtomic {
    data: AtomicU64, // packs ptr (lower 48 bits) + tag (upper 16 bits)
}

impl TaggedAtomic {
    fn new(ptr: usize) -> Self {
        TaggedAtomic { data: AtomicU64::new(ptr as u64) }
    }

    fn load(&self) -> TaggedPtr {
        let v = self.data.load(Acquire);
        TaggedPtr { ptr: (v & 0x0000_FFFF_FFFF_FFFF) as usize, tag: (v >> 48) as usize }
    }

    fn cas(&self, expected: TaggedPtr, new_ptr: usize) -> Result<TaggedPtr, TaggedPtr> {
        let new_tag = expected.tag.wrapping_add(1);
        let new_val = (new_tag as u64) << 48 | (new_ptr as u64 & 0x0000_FFFF_FFFF_FFFF);
        let old_val = (expected.tag as u64) << 48 | (expected.ptr as u64);

        self.data.compare_exchange(old_val, new_val, AcqRel, Relaxed)
            .map(|_| TaggedPtr { ptr: new_ptr, tag: new_tag })
            .map_err(|v| TaggedPtr {
                ptr: (v & 0x0000_FFFF_FFFF_FFFF) as usize,
                tag: (v >> 48) as usize,
            })
    }
}

// ─── Wait-Free Counter with Combining ────────────────────────────────────────
// Each thread has its own slot — readers sum all slots.
// Completely wait-free for writers.

const MAX_THREADS: usize = 16;

#[repr(align(64))] // cache-line align to prevent false sharing
struct CachePadded<T>(T, [u8; 64 - std::mem::size_of::<AtomicUsize>() % 64]);

struct WaitFreeCounter {
    slots: [AtomicUsize; MAX_THREADS],
}

impl WaitFreeCounter {
    fn new() -> Self {
        WaitFreeCounter { slots: [const { AtomicUsize::new(0) }; MAX_THREADS] }
    }

    fn increment(&self, thread_id: usize) {
        self.slots[thread_id % MAX_THREADS].fetch_add(1, Relaxed);
    }

    fn decrement(&self, thread_id: usize) {
        self.slots[thread_id % MAX_THREADS].fetch_sub(1, Relaxed);
    }

    fn read(&self) -> usize {
        // Sum all slots — may race with writes, but gives a consistent snapshot
        fence(SeqCst);
        self.slots.iter().map(|s| s.load(Relaxed)).sum()
    }
}

// ─── Ticket Lock ──────────────────────────────────────────────────────────────
// Fair mutex: threads take a ticket, wait for their number to be called.

struct TicketLock {
    next_ticket: AtomicUsize,
    now_serving: AtomicUsize,
}

impl TicketLock {
    fn new() -> Self {
        TicketLock {
            next_ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
        }
    }

    fn lock(&self) -> TicketGuard {
        let ticket = self.next_ticket.fetch_add(1, SeqCst);
        while self.now_serving.load(Acquire) != ticket {
            std::hint::spin_loop();
        }
        TicketGuard { lock: self }
    }
}

struct TicketGuard<'a> {
    lock: &'a TicketLock,
}

impl<'a> Drop for TicketGuard<'a> {
    fn drop(&mut self) {
        self.lock.now_serving.fetch_add(1, Release);
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Lock-Free Algorithms & Memory Ordering ===\n");

    demonstrate_ordering();

    // SeqLock
    println!("\n── SeqLock ──");
    let lock = Arc::new(SeqLock::new((0u64, 0u64)));
    let mut handles = vec![];

    for _ in 0..4 {
        let l = Arc::clone(&lock);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let (a, b) = l.read();
                assert_eq!(a, b, "torn read! a={} b={}", a, b);
            }
        }));
    }
    for i in 0u64..50 {
        lock.write((i, i));
    }
    for h in handles { h.join().unwrap(); }
    println!("  SeqLock: no torn reads detected");

    // MS Queue
    println!("\n── Michael-Scott Lock-Free Queue ──");
    let q = Arc::new(MsQueue::new());
    let mut handles = vec![];

    let producers = 4;
    let items_each = 25;

    for p in 0..producers {
        let q2 = Arc::clone(&q);
        handles.push(thread::spawn(move || {
            for i in 0..items_each {
                q2.enqueue(p * 100 + i);
            }
        }));
    }

    let consumed = Arc::new(AtomicUsize::new(0));
    for _ in 0..2 {
        let q2 = Arc::clone(&q);
        let c  = Arc::clone(&consumed);
        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                if q2.dequeue().is_some() { c.fetch_add(1, Relaxed); }
                thread::sleep(Duration::from_micros(10));
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    // Drain remaining
    let mut remaining = 0;
    while q.dequeue().is_some() { remaining += 1; }
    println!("  produced: {}, consumed by threads: {}, remaining: {}",
        producers * items_each, consumed.load(Relaxed), remaining);

    // Epoch-based reclamation
    println!("\n── Epoch-Based Reclamation ──");
    let epoch = Epoch::new();
    {
        let e = Arc::clone(&epoch);
        let t = thread::spawn(move || {
            let pinned = e.pin();
            thread::sleep(Duration::from_millis(50));
            e.defer_free(pinned, Box::new(|| println!("  deferred drop ran!")));
            e.unpin();
            e.advance();
            e.collect_garbage();
        });
        t.join().unwrap();
    }

    // Tagged pointer (ABA prevention)
    println!("\n── Tagged Pointer (ABA prevention) ──");
    let tagged = TaggedAtomic::new(0x1000);
    let t1 = tagged.load();
    println!("  initial: ptr=0x{:x}, tag={}", t1.ptr, t1.tag);

    let r1 = tagged.cas(t1, 0x2000);
    println!("  after cas to 0x2000: {:?}", r1);

    let t2 = tagged.load();
    let r2 = tagged.cas(t2, 0x3000);
    println!("  after cas to 0x3000: {:?}", r2);

    // Old t1 CAS now fails (tag mismatch) — no ABA
    let r3 = tagged.cas(t1, 0x1000);
    println!("  stale cas (should fail): {:?}", r3.is_err());

    // Wait-free counter
    println!("\n── Wait-Free Counter ──");
    let counter = Arc::new(WaitFreeCounter::new());
    let mut handles = vec![];
    for tid in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 { c.increment(tid); }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("  8 threads × 1000 increments = {}", counter.read());

    // Ticket lock
    println!("\n── Ticket Lock (fair ordering) ──");
    let ticket = Arc::new(TicketLock::new());
    let log = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for id in 0..5 {
        let t = Arc::clone(&ticket);
        let l = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            let _guard = t.lock();
            l.lock().unwrap().push(id);
            thread::sleep(Duration::from_millis(5));
        }));
    }
    for h in handles { h.join().unwrap(); }
    // Ticket lock guarantees FIFO, but thread scheduling may vary
    println!("  execution order: {:?}", log.lock().unwrap());

    println!("\n=== Done ===");
}