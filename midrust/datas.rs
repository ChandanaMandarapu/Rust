// File 13: Data Structures From Scratch
// Stack, Queue, Deque, Trie, LRU Cache, Graph, Min-Heap — all in safe Rust

use std::collections::HashMap;
use std::fmt;

// ─── Generic Stack ────────────────────────────────────────────────────────────

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { data: Vec::new() } }

    fn push(&mut self, item: T)     { self.data.push(item); }
    fn pop(&mut self) -> Option<T>  { self.data.pop() }
    fn peek(&self)    -> Option<&T> { self.data.last() }
    fn is_empty(&self) -> bool      { self.data.is_empty() }
    fn len(&self)      -> usize     { self.data.len() }
    fn clear(&mut self)             { self.data.clear(); }
}

impl<T: fmt::Display> Stack<T> {
    fn print(&self) {
        print!("Stack (top→): ");
        for item in self.data.iter().rev() {
            print!("{} ", item);
        }
        println!();
    }
}

// Balanced bracket checker using a stack
fn is_balanced(s: &str) -> bool {
    let mut stack = Stack::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; }
            ']' => if stack.pop() != Some('[') { return false; }
            '}' => if stack.pop() != Some('{') { return false; }
            _ => {}
        }
    }
    stack.is_empty()
}

// ─── Queue ────────────────────────────────────────────────────────────────────

struct Queue<T> {
    head: usize,
    data: Vec<Option<T>>,
}

impl<T> Queue<T> {
    fn new() -> Self { Queue { head: 0, data: Vec::new() } }

    fn enqueue(&mut self, item: T) { self.data.push(Some(item)); }

    fn dequeue(&mut self) -> Option<T> {
        while self.head < self.data.len() {
            if let Some(item) = self.data[self.head].take() {
                self.head += 1;
                return Some(item);
            }
            self.head += 1;
        }
        None
    }

    fn peek(&self) -> Option<&T> {
        self.data[self.head..].iter().find_map(|x| x.as_ref())
    }

    fn len(&self) -> usize {
        self.data[self.head..].iter().filter(|x| x.is_some()).count()
    }

    fn is_empty(&self) -> bool { self.len() == 0 }
}

// ─── Deque (Double-Ended Queue) ───────────────────────────────────────────────

struct Deque<T> {
    data: std::collections::VecDeque<T>,
}

impl<T: fmt::Debug> Deque<T> {
    fn new() -> Self { Deque { data: std::collections::VecDeque::new() } }

    fn push_front(&mut self, item: T) { self.data.push_front(item); }
    fn push_back (&mut self, item: T) { self.data.push_back(item); }
    fn pop_front (&mut self) -> Option<T> { self.data.pop_front() }
    fn pop_back  (&mut self) -> Option<T> { self.data.pop_back() }
    fn peek_front(&self) -> Option<&T>  { self.data.front() }
    fn peek_back (&self) -> Option<&T>  { self.data.back() }
    fn len        (&self) -> usize       { self.data.len() }
    fn is_empty   (&self) -> bool        { self.data.is_empty() }

    fn print(&self) { println!("Deque: {:?}", self.data); }
}

// Sliding window maximum using deque
fn sliding_window_max(arr: &[i32], k: usize) -> Vec<i32> {
    let mut dq: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    let mut result = Vec::new();

    for i in 0..arr.len() {
        // Remove elements outside window
        while dq.front().map_or(false, |&front| front + k <= i) {
            dq.pop_front();
        }
        // Remove smaller elements from back
        while dq.back().map_or(false, |&back| arr[back] <= arr[i]) {
            dq.pop_back();
        }
        dq.push_back(i);
        if i + 1 >= k {
            result.push(arr[*dq.front().unwrap()]);
        }
    }
    result
}

// ─── Trie ─────────────────────────────────────────────────────────────────────

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
    count: usize, // how many words pass through this node
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { children: HashMap::new(), is_end: false, count: 0 }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self { Trie { root: TrieNode::new() } }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
            node.count += 1;
        }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }

    fn count_words_with_prefix(&self, prefix: &str) -> usize {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return 0,
            }
        }
        node.count
    }

    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return vec![],
            }
        }
        let mut results = Vec::new();
        self.collect(node, &mut prefix.to_string(), &mut results);
        results
    }

    fn collect(&self, node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(current.clone());
        }
        for (ch, child) in &node.children {
            current.push(*ch);
            self.collect(child, current, results);
            current.pop();
        }
    }
}

// ─── LRU Cache ────────────────────────────────────────────────────────────────

struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: std::collections::VecDeque<K>,
}

impl<K: std::hash::Hash + Eq + Clone + fmt::Debug, V: Clone + fmt::Debug> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            order: std::collections::VecDeque::new(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_back() {
                self.map.remove(&oldest);
                println!("  Evicted: {:?}", oldest);
            }
        }
        self.map.insert(key.clone(), value);
        self.order.push_front(key);
    }

    fn len(&self) -> usize { self.map.len() }

    fn print_order(&self) {
        print!("  Order (MRU→LRU): ");
        for k in &self.order {
            print!("{:?} ", k);
        }
        println!();
    }
}

// ─── Min-Heap ─────────────────────────────────────────────────────────────────

struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord + fmt::Debug> MinHeap<T> {
    fn new() -> Self { MinHeap { data: Vec::new() } }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.sift_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() { return None; }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }

    fn peek(&self) -> Option<&T> { self.data.first() }
    fn len(&self)   -> usize     { self.data.len() }
    fn is_empty(&self) -> bool   { self.data.is_empty() }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let len = self.data.len();
        loop {
            let left  = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;

            if left  < len && self.data[left]  < self.data[smallest] { smallest = left; }
            if right < len && self.data[right] < self.data[smallest] { smallest = right; }

            if smallest == i { break; }
            self.data.swap(i, smallest);
            i = smallest;
        }
    }

    fn heap_sort(mut items: Vec<T>) -> Vec<T> {
        let mut heap = MinHeap::new();
        for item in items.drain(..) { heap.push(item); }
        let mut sorted = Vec::new();
        while let Some(item) = heap.pop() { sorted.push(item); }
        sorted
    }
}

// ─── Directed Graph ───────────────────────────────────────────────────────────

struct Graph {
    adj: HashMap<String, Vec<(String, u32)>>, // node -> [(neighbor, weight)]
}

impl Graph {
    fn new() -> Self { Graph { adj: HashMap::new() } }

    fn add_edge(&mut self, from: &str, to: &str, weight: u32) {
        self.adj.entry(from.to_string()).or_default().push((to.to_string(), weight));
        self.adj.entry(to.to_string()).or_default(); // ensure node exists
    }

    fn vertices(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.adj.keys().map(|s| s.as_str()).collect();
        v.sort();
        v
    }

    // BFS — shortest path by hops
    fn bfs(&self, start: &str) -> HashMap<String, usize> {
        let mut dist = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        dist.insert(start.to_string(), 0usize);
        queue.push_back(start.to_string());

        while let Some(node) = queue.pop_front() {
            let d = dist[&node];
            if let Some(neighbors) = self.adj.get(&node) {
                for (neighbor, _) in neighbors {
                    if !dist.contains_key(neighbor) {
                        dist.insert(neighbor.clone(), d + 1);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        dist
    }

    // DFS — topological order (no cycle detection)
    fn dfs_order(&self, start: &str) -> Vec<String> {
        let mut visited = std::collections::HashSet::new();
        let mut order = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut order);
        order
    }

    fn dfs_recursive(&self, node: &str, visited: &mut std::collections::HashSet<String>, order: &mut Vec<String>) {
        if !visited.insert(node.to_string()) { return; }
        order.push(node.to_string());
        if let Some(neighbors) = self.adj.get(node) {
            for (neighbor, _) in neighbors {
                self.dfs_recursive(neighbor, visited, order);
            }
        }
    }

    // Dijkstra — shortest weighted path
    fn dijkstra(&self, start: &str) -> HashMap<String, u32> {
        let mut dist: HashMap<String, u32> = self.adj.keys().map(|k| (k.clone(), u32::MAX)).collect();
        dist.insert(start.to_string(), 0);

        let mut heap = MinHeap::new();
        heap.push(std::cmp::Reverse((0u32, start.to_string())));

        while let Some(std::cmp::Reverse((d, node))) = heap.pop() {
            if d > *dist.get(&node).unwrap_or(&u32::MAX) { continue; }

            if let Some(neighbors) = self.adj.get(&node) {
                for (neighbor, weight) in neighbors {
                    let new_dist = d + weight;
                    if new_dist < *dist.get(neighbor).unwrap_or(&u32::MAX) {
                        dist.insert(neighbor.clone(), new_dist);
                        heap.push(std::cmp::Reverse((new_dist, neighbor.clone())));
                    }
                }
            }
        }
        dist.into_iter().filter(|(_, v)| *v != u32::MAX).collect()
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Data Structures ===\n");

    // Stack
    println!("── Stack ──");
    let mut stack = Stack::new();
    for i in [10, 20, 30, 40] { stack.push(i); }
    stack.print();
    println!("peek: {:?}", stack.peek());
    println!("pop:  {:?}", stack.pop());
    stack.print();

    println!("\nBracket checker:");
    for s in ["({[]})", "([)]", "{[]}", "(((" ] {
        println!("  {:12} → {}", s, if is_balanced(s) { "balanced" } else { "UNBALANCED" });
    }

    // Queue
    println!("\n── Queue ──");
    let mut q: Queue<&str> = Queue::new();
    q.enqueue("first"); q.enqueue("second"); q.enqueue("third");
    println!("peek: {:?}", q.peek());
    println!("dequeue: {:?}", q.dequeue());
    println!("dequeue: {:?}", q.dequeue());
    println!("len: {}", q.len());

    // Deque + sliding window max
    println!("\n── Deque + Sliding Window Max ──");
    let mut dq = Deque::new();
    for v in [1, 3, -1, -3, 5, 3, 6] { dq.push_back(v); }
    dq.print();
    let arr = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let maxes = sliding_window_max(&arr, 3);
    println!("Window maxes (k=3): {:?}", maxes);

    // Trie
    println!("\n── Trie ──");
    let mut trie = Trie::new();
    for word in ["rust", "rusty", "rustacean", "rune", "run", "running", "python"] {
        trie.insert(word);
    }
    println!("search 'rust':        {}", trie.search("rust"));
    println!("search 'ruster':      {}", trie.search("ruster"));
    println!("starts_with 'run':    {}", trie.starts_with("run"));
    println!("words with 'ru':      {}", trie.count_words_with_prefix("ru"));
    println!("autocomplete 'run':   {:?}", trie.autocomplete("run"));
    println!("autocomplete 'rust':  {:?}", trie.autocomplete("rust"));

    // LRU Cache
    println!("\n── LRU Cache (capacity 3) ──");
    let mut cache: LruCache<&str, i32> = LruCache::new(3);
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    cache.print_order();
    println!("get 'a': {:?}", cache.get(&"a")); // a moves to front
    cache.print_order();
    cache.put("d", 4); // evicts LRU
    cache.print_order();
    println!("get 'b': {:?}", cache.get(&"b")); // b was evicted

    // Min-Heap
    println!("\n── Min-Heap ──");
    let mut heap = MinHeap::new();
    for n in [5, 2, 8, 1, 9, 3] { heap.push(n); }
    println!("peek (min): {:?}", heap.peek());
    let mut sorted = Vec::new();
    while let Some(n) = heap.pop() { sorted.push(n); }
    println!("heap sort output: {:?}", sorted);

    println!("\nHeap sort via static method:");
    let items = vec!["banana", "apple", "cherry", "date", "elderberry"];
    println!("{:?}", MinHeap::heap_sort(items));

    // Graph
    println!("\n── Graph ──");
    let mut g = Graph::new();
    g.add_edge("A", "B", 4);
    g.add_edge("A", "C", 2);
    g.add_edge("B", "D", 3);
    g.add_edge("C", "B", 1);
    g.add_edge("C", "D", 5);
    g.add_edge("D", "E", 1);

    println!("Vertices: {:?}", g.vertices());

    let bfs = g.bfs("A");
    let mut bfs_sorted: Vec<_> = bfs.iter().collect();
    bfs_sorted.sort_by_key(|(k, _)| k.as_str());
    println!("BFS from A (hops): {:?}", bfs_sorted);

    println!("DFS order from A: {:?}", g.dfs_order("A"));

    let dijkstra = g.dijkstra("A");
    let mut dijk_sorted: Vec<_> = dijkstra.iter().collect();
    dijk_sorted.sort_by_key(|(k, _)| k.as_str());
    println!("Dijkstra from A:  {:?}", dijk_sorted);

    println!("\n=== Done ===");
}