// Implemented self referential structs using Pin and PhantomPinned, built a pinned coroutine style state machine, and explored safe vs unsafe pin projections. Added intrusive linked list with embedded nodes for scheduler style queues, plus a safe owned doubly linked list with cursors and iterators. Built raw pointer graph with BFS and DFS traversal including cycles and manual memory cleanup. Main demonstrates pin guarantees, coroutine resumption, intrusive scheduling, cursor based insertion, and graph traversal to explain why Pin exists and how low level Rust data structures work.

use std::pin::Pin;
use std::marker::{PhantomPinned, PhantomData};
use std::ptr::{self, NonNull};
use std::mem;
use std::fmt;
use std::cell::Cell;

// ─── Why Pin Exists ──────────────────────────────────────────────────────────
// Normally, every value in Rust can be moved (memcpy'd to a new address).
// This is fine unless a struct contains a pointer to itself.
// If moved, the internal pointer becomes dangling.
// Pin<P> promises: the value behind pointer P will NOT be moved.

// ─── Self-Referential Struct (the problem) ────────────────────────────────────

struct SelfRef {
    value: u64,
    // Points into `value` — if `self` is moved, this pointer is dangling!
    ptr_to_value: *const u64,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(v: u64) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfRef {
            value: v,
            ptr_to_value: ptr::null(),
            _pin: PhantomPinned,
        });
        // SAFETY: we're setting the internal pointer before any other access.
        // The Pin<Box<T>> guarantees this address will never change.
        unsafe {
            let raw = boxed.as_mut().get_unchecked_mut();
            raw.ptr_to_value = &raw.value as *const u64;
        }
        boxed
    }

    fn get_via_ptr(&self) -> u64 {
        // SAFETY: ptr_to_value points into self.value, which hasn't moved (Pin)
        unsafe { *self.ptr_to_value }
    }

    fn mutate(self: Pin<&mut Self>, new_val: u64) {
        // SAFETY: we update the pointer after changing value, maintaining invariant
        unsafe {
            let raw = self.get_unchecked_mut();
            raw.value = new_val;
            // Pointer still valid — same address, just different value at that address
            raw.ptr_to_value = &raw.value as *const u64;
        }
    }
}

impl fmt::Display for SelfRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelfRef {{ value: {}, via_ptr: {} }}", self.value, self.get_via_ptr())
    }
}

// ─── Coroutine State Machine (Pin'd) ─────────────────────────────────────────
// Async/await compiles to state machines that may contain references to
// their own fields across yield points. These MUST be pinned.

#[derive(Debug)]
enum CoroutineState { NotStarted, Yielded(i32), Finished(i32) }

struct Coroutine {
    // Pretend this holds a reference to `accumulator` across a yield point
    // (in real async code, this would be a &'self field)
    accumulator: i32,
    step: u32,
    // Internal pointer simulating a cross-yield borrow
    acc_ptr: *const i32,
    _pin: PhantomPinned,
}

impl Coroutine {
    fn new() -> Pin<Box<Self>> {
        let mut b = Box::pin(Coroutine {
            accumulator: 0,
            step: 0,
            acc_ptr: ptr::null(),
            _pin: PhantomPinned,
        });
        unsafe {
            let raw = b.as_mut().get_unchecked_mut();
            raw.acc_ptr = &raw.accumulator as *const i32;
        }
        b
    }

    fn resume(self: Pin<&mut Self>) -> CoroutineState {
        unsafe {
            let raw = self.get_unchecked_mut();
            raw.step += 1;
            match raw.step {
                1 => { raw.accumulator += 10; CoroutineState::Yielded(*raw.acc_ptr) }
                2 => { raw.accumulator += 20; CoroutineState::Yielded(*raw.acc_ptr) }
                3 => { raw.accumulator += 30; CoroutineState::Finished(*raw.acc_ptr) }
                _ => CoroutineState::Finished(*raw.acc_ptr),
            }
        }
    }
}

// ─── Intrusive Linked List ────────────────────────────────────────────────────
// Classic kernel-style: the list node is embedded IN the element,
// not separately heap-allocated. Zero extra allocation overhead.

// Each element that wants to be in a list embeds this node
struct ListNode {
    prev: Cell<*mut ListNode>,
    next: Cell<*mut ListNode>,
}

impl ListNode {
    fn new() -> Self {
        ListNode {
            prev: Cell::new(ptr::null_mut()),
            next: Cell::new(ptr::null_mut()),
        }
    }

    fn is_linked(&self) -> bool { !self.next.get().is_null() }
}

struct IntrusiveList {
    head: *mut ListNode,
    tail: *mut ListNode,
    len:  usize,
}

// SAFETY: IntrusiveList owns the nodes it points to
unsafe impl Send for IntrusiveList {}

impl IntrusiveList {
    fn new() -> Self { IntrusiveList { head: ptr::null_mut(), tail: ptr::null_mut(), len: 0 } }

    unsafe fn push_back(&mut self, node: *mut ListNode) {
        (*node).prev.set(self.tail);
        (*node).next.set(ptr::null_mut());
        if self.tail.is_null() {
            self.head = node;
        } else {
            (*self.tail).next.set(node);
        }
        self.tail = node;
        self.len += 1;
    }

    unsafe fn remove(&mut self, node: *mut ListNode) {
        let prev = (*node).prev.get();
        let next = (*node).next.get();
        if !prev.is_null() { (*prev).next.set(next); } else { self.head = next; }
        if !next.is_null() { (*next).prev.set(prev); } else { self.tail = prev; }
        (*node).prev.set(ptr::null_mut());
        (*node).next.set(ptr::null_mut());
        self.len -= 1;
    }

    unsafe fn pop_front(&mut self) -> *mut ListNode {
        if self.head.is_null() { return ptr::null_mut(); }
        let node = self.head;
        self.remove(node);
        node
    }

    fn is_empty(&self) -> bool { self.head.is_null() }
    fn len(&self) -> usize { self.len }

    // Iterate by walking next pointers
    unsafe fn iter_raw(&self) -> IntrusiveIter {
        IntrusiveIter { current: self.head }
    }
}

struct IntrusiveIter { current: *mut ListNode }
impl Iterator for IntrusiveIter {
    type Item = *mut ListNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() { return None; }
        let node = self.current;
        self.current = unsafe { (*node).next.get() };
        Some(node)
    }
}

// An element that embeds a list node
#[repr(C)] // ensure `node` is at a known offset
struct Task {
    node:     ListNode, // MUST be first field (or use offset_of!)
    id:       u32,
    priority: u8,
    name:     String,
}

impl Task {
    fn new(id: u32, priority: u8, name: &str) -> Box<Self> {
        Box::new(Task {
            node: ListNode::new(),
            id, priority,
            name: name.to_string(),
        })
    }

    // SAFETY: node must be the first field (repr(C))
    unsafe fn from_node(node: *mut ListNode) -> *mut Task {
        node as *mut Task
    }
}

// ─── Doubly-Linked List with Ownership (safe wrapper) ────────────────────────

struct Node<T> {
    value: T,
    prev:  Option<NonNull<Node<T>>>,
    next:  Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head:   Option<NonNull<Node<T>>>,
    tail:   Option<NonNull<Node<T>>>,
    length: usize,
    _own:   PhantomData<Box<Node<T>>>,
}

impl<T: fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, tail: None, length: 0, _own: PhantomData }
    }

    pub fn push_front(&mut self, value: T) {
        let node = Box::into_raw(Box::new(Node { value, prev: None, next: self.head }));
        let nn   = unsafe { NonNull::new_unchecked(node) };
        if let Some(old_head) = self.head {
            unsafe { (*old_head.as_ptr()).prev = Some(nn); }
        } else {
            self.tail = Some(nn);
        }
        self.head = Some(nn);
        self.length += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::into_raw(Box::new(Node { value, prev: self.tail, next: None }));
        let nn   = unsafe { NonNull::new_unchecked(node) };
        if let Some(old_tail) = self.tail {
            unsafe { (*old_tail.as_ptr()).next = Some(nn); }
        } else {
            self.head = Some(nn);
        }
        self.tail = Some(nn);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            unsafe {
                let boxed = Box::from_raw(node.as_ptr());
                self.head = boxed.next;
                if let Some(new_head) = self.head {
                    (*new_head.as_ptr()).prev = None;
                } else {
                    self.tail = None;
                }
                self.length -= 1;
                boxed.value
            }
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            unsafe {
                let boxed = Box::from_raw(node.as_ptr());
                self.tail = boxed.prev;
                if let Some(new_tail) = self.tail {
                    (*new_tail.as_ptr()).next = None;
                } else {
                    self.head = None;
                }
                self.length -= 1;
                boxed.value
            }
        })
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|n| unsafe { &(*n.as_ptr()).value })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|n| unsafe { &(*n.as_ptr()).value })
    }

    pub fn len(&self) -> usize { self.length }
    pub fn is_empty(&self) -> bool { self.length == 0 }

    pub fn iter(&self) -> ListIter<T> {
        ListIter { current: self.head, _marker: PhantomData }
    }

    // Cursor: O(n) move-to, O(1) insert/remove at position
    pub fn cursor_front(&mut self) -> Option<Cursor<T>> {
        self.head.map(|node| Cursor { node, list: self as *mut _ })
    }
}

pub struct ListIter<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.current.map(|node| {
            let n = unsafe { &*node.as_ptr() };
            self.current = n.next;
            &n.value
        })
    }
}

pub struct Cursor<T> {
    node: NonNull<Node<T>>,
    list: *mut LinkedList<T>,
}

impl<T: fmt::Debug> Cursor<T> {
    pub fn current(&self) -> &T {
        unsafe { &(*self.node.as_ptr()).value }
    }

    pub fn move_next(&mut self) -> bool {
        unsafe {
            if let Some(next) = (*self.node.as_ptr()).next {
                self.node = next;
                true
            } else {
                false
            }
        }
    }

    pub fn move_prev(&mut self) -> bool {
        unsafe {
            if let Some(prev) = (*self.node.as_ptr()).prev {
                self.node = prev;
                true
            } else {
                false
            }
        }
    }

    // Insert after current position
    pub fn insert_after(&mut self, value: T) {
        unsafe {
            let list   = &mut *self.list;
            let cur    = self.node.as_ptr();
            let next   = (*cur).next;
            let new_nn = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node { value, prev: Some(self.node), next }
            )));
            (*cur).next = Some(new_nn);
            if let Some(n) = next {
                (*n.as_ptr()).prev = Some(new_nn);
            } else {
                list.tail = Some(new_nn);
            }
            list.length += 1;
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// ─── Raw-Pointer Graph ────────────────────────────────────────────────────────
// A directed graph where nodes are heap-allocated and edges are raw pointers.
// Demonstrates manual memory management for cyclic structures.

struct GraphNode {
    id:        u32,
    label:     String,
    neighbors: Vec<*mut GraphNode>,
}

struct RawGraph {
    nodes: Vec<*mut GraphNode>,
}

impl RawGraph {
    fn new() -> Self { RawGraph { nodes: vec![] } }

    fn add_node(&mut self, id: u32, label: &str) -> *mut GraphNode {
        let node = Box::into_raw(Box::new(GraphNode {
            id,
            label: label.to_string(),
            neighbors: vec![],
        }));
        self.nodes.push(node);
        node
    }

    unsafe fn add_edge(&mut self, from: *mut GraphNode, to: *mut GraphNode) {
        (*from).neighbors.push(to);
    }

    // BFS from a root node
    unsafe fn bfs(&self, root: *mut GraphNode) -> Vec<u32> {
        let mut visited = std::collections::HashSet::new();
        let mut queue   = std::collections::VecDeque::new();
        let mut order   = vec![];

        queue.push_back(root);
        visited.insert((*root).id);

        while let Some(node) = queue.pop_front() {
            order.push((*node).id);
            for &neighbor in &(*node).neighbors {
                if visited.insert((*neighbor).id) {
                    queue.push_back(neighbor);
                }
            }
        }
        order
    }

    // DFS with cycle detection
    unsafe fn dfs(&self, root: *mut GraphNode, visited: &mut std::collections::HashSet<u32>, order: &mut Vec<u32>) {
        if !visited.insert((*root).id) { return; }
        order.push((*root).id);
        for &neighbor in &(*root).neighbors {
            self.dfs(neighbor, visited, order);
        }
    }

    unsafe fn print_node(node: *const GraphNode) {
        let n = &*node;
        let neighbors: Vec<u32> = n.neighbors.iter().map(|&&mut ref np| (*np).id).collect();
        println!("  Node {} '{}' → {:?}", n.id, n.label, neighbors);
    }
}

impl Drop for RawGraph {
    fn drop(&mut self) {
        for &node in &self.nodes {
            unsafe { drop(Box::from_raw(node)); }
        }
    }
}

// ─── Pin Projection ───────────────────────────────────────────────────────────
// Projecting a Pin<&mut Struct> to Pin<&mut Field> safely

struct TwoFields {
    pinned_field:   u64,
    unpinned_field: String,
    _pin: PhantomPinned,
}

impl TwoFields {
    fn new(n: u64, s: &str) -> Pin<Box<Self>> {
        Box::pin(TwoFields {
            pinned_field: n,
            unpinned_field: s.to_string(),
            _pin: PhantomPinned,
        })
    }

    // Safe projection to pinned field (field is !Unpin, so keep Pin)
    fn pinned_field(self: Pin<&mut Self>) -> Pin<&mut u64> {
        // SAFETY: pinned_field doesn't implement Drop that moves things,
        // and we never move it out
        unsafe { self.map_unchecked_mut(|s| &mut s.pinned_field) }
    }

    // Safe projection to unpinned field (String: Unpin, so we can take &mut)
    fn unpinned_field(self: Pin<&mut Self>) -> &mut String {
        // SAFETY: String is Unpin — moving it is fine
        unsafe { &mut self.get_unchecked_mut().unpinned_field }
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Pin, Self-Referential & Intrusive ===\n");

    // Self-referential struct
    println!("── Self-Referential ──");
    let mut sr = SelfRef::new(42);
    println!("  {}", sr.as_ref().get_ref());
    sr.as_mut().mutate(999);
    println!("  after mutate: {}", sr.as_ref().get_ref());

    // Coroutine state machine
    println!("\n── Pinned Coroutine ──");
    let mut co = Coroutine::new();
    loop {
        match co.as_mut().resume() {
            CoroutineState::Yielded(v)  => println!("  yielded: {}", v),
            CoroutineState::NotStarted  => {}
            CoroutineState::Finished(v) => { println!("  finished: {}", v); break; }
        }
    }

    // Intrusive linked list (task scheduler style)
    println!("\n── Intrusive Linked List ──");
    let mut t0 = Task::new(0, 5, "idle");
    let mut t1 = Task::new(1, 10, "io-handler");
    let mut t2 = Task::new(2, 8, "worker");
    let mut t3 = Task::new(3, 15, "timer");

    let mut runqueue = IntrusiveList::new();
    unsafe {
        runqueue.push_back(&mut t0.node as *mut ListNode);
        runqueue.push_back(&mut t1.node as *mut ListNode);
        runqueue.push_back(&mut t2.node as *mut ListNode);
        runqueue.push_back(&mut t3.node as *mut ListNode);
    }

    println!("  runqueue len: {}", runqueue.len());
    println!("  traversal:");
    unsafe {
        for node_ptr in runqueue.iter_raw() {
            let task = Task::from_node(node_ptr);
            println!("    Task[{}] '{}' priority={}", (*task).id, (*task).name, (*task).priority);
        }
    }

    // Pop highest priority (id=3 = timer, manually here)
    unsafe {
        runqueue.remove(&mut t3.node as *mut ListNode);
        println!("  removed timer, len={}", runqueue.len());
    }

    // Safe doubly-linked list
    println!("\n── Safe Doubly-Linked List ──");
    let mut list: LinkedList<i32> = LinkedList::new();
    for x in [10, 20, 30, 40, 50] { list.push_back(x); }
    list.push_front(5);

    print!("  iter: ");
    for v in list.iter() { print!("{} ", v); }
    println!();
    println!("  front={:?}, back={:?}, len={}", list.front(), list.back(), list.len());
    println!("  pop_front={:?}", list.pop_front());
    println!("  pop_back ={:?}", list.pop_back());

    // Cursor insert
    if let Some(mut cursor) = list.cursor_front() {
        cursor.move_next();
        cursor.insert_after(999);
    }
    print!("  after cursor insert: ");
    for v in list.iter() { print!("{} ", v); }
    println!();

    // Raw pointer graph
    println!("\n── Raw-Pointer Graph ──");
    let mut g = RawGraph::new();
    unsafe {
        let a = g.add_node(0, "A");
        let b = g.add_node(1, "B");
        let c = g.add_node(2, "C");
        let d = g.add_node(3, "D");
        let e = g.add_node(4, "E");

        g.add_edge(a, b); g.add_edge(a, c);
        g.add_edge(b, d); g.add_edge(c, d);
        g.add_edge(d, e); g.add_edge(e, a); // cycle!

        for &node in &g.nodes { RawGraph::print_node(node); }

        let bfs_order = g.bfs(a);
        println!("  BFS from A: {:?}", bfs_order);

        let mut visited = std::collections::HashSet::new();
        let mut dfs_order = vec![];
        g.dfs(a, &mut visited, &mut dfs_order);
        println!("  DFS from A: {:?}", dfs_order);
    }

    // Pin projection
    println!("\n── Pin Projection ──");
    let mut tf = TwoFields::new(100, "hello");
    {
        let unpinned = tf.as_mut().unpinned_field();
        unpinned.push_str(" world");
    }
    {
        let mut pinned = tf.as_mut().pinned_field();
        unsafe { *pinned.as_mut().get_unchecked_mut() += 1; }
    }
    println!("  pinned_field={}, unpinned={}", tf.pinned_field, tf.unpinned_field);

    println!("\n=== Done ===");
}