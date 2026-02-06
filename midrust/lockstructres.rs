use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::ptr::{self, NonNull};
use std::mem::{self, ManuallyDrop};
use std::cell::UnsafeCell;
use std::marker::PhantomData;

struct Node<T> {
    value: ManuallyDrop<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> *mut Self {
        Box::into_raw(Box::new(Node {
            value: ManuallyDrop::new(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }))
    }
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
    count: AtomicUsize,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
            count: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, value: T) {
        let new_node = Node::new(value);

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next.store(head, Ordering::Release);
            }

            if self.head.compare_exchange(
                head,
                new_node,
                Ordering::Release,
                Ordering::Acquire,
            ).is_ok() {
                self.count.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);

            if head.is_null() {
                return None;
            }

            unsafe {
                let next = (*head).next.load(Ordering::Acquire);

                if self.head.compare_exchange(
                    head,
                    next,
                    Ordering::Release,
                    Ordering::Acquire,
                ).is_ok() {
                    self.count.fetch_sub(1, Ordering::Relaxed);
                    let value = ManuallyDrop::take(&mut (*head).value);
                    drop(Box::from_raw(head));
                    return Some(value);
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
    count: AtomicUsize,
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        let dummy = Node::new(unsafe { mem::zeroed() });
        LockFreeQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
            count: AtomicUsize::new(0),
        }
    }

    pub fn enqueue(&self, value: T) {
        let new_node = Node::new(value);

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if tail == self.tail.load(Ordering::Acquire) {
                if next.is_null() {
                    if unsafe {
                        (*tail).next.compare_exchange(
                            next,
                            new_node,
                            Ordering::Release,
                            Ordering::Acquire,
                        ).is_ok()
                    } {
                        self.tail.compare_exchange(
                            tail,
                            new_node,
                            Ordering::Release,
                            Ordering::Acquire,
                        ).ok();
                        self.count.fetch_add(1, Ordering::Relaxed);
                        break;
                    }
                } else {
                    self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Acquire,
                    ).ok();
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == self.head.load(Ordering::Acquire) {
                if head == tail {
                    if next.is_null() {
                        return None;
                    }
                    self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Acquire,
                    ).ok();
                } else {
                    if !next.is_null() {
                        let value = unsafe { ManuallyDrop::take(&mut (*next).value) };
                        
                        if self.head.compare_exchange(
                            head,
                            next,
                            Ordering::Release,
                            Ordering::Acquire,
                        ).is_ok() {
                            self.count.fetch_sub(1, Ordering::Relaxed);
                            unsafe { drop(Box::from_raw(head)); }
                            return Some(value);
                        }
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        unsafe {
            let head = self.head.load(Ordering::Acquire);
            if !head.is_null() {
                drop(Box::from_raw(head));
            }
        }
    }
}

struct HashNode<K, V> {
    key: K,
    value: UnsafeCell<V>,
    hash: u64,
    next: AtomicPtr<HashNode<K, V>>,
}

impl<K, V> HashNode<K, V> {
    fn new(key: K, value: V, hash: u64) -> *mut Self {
        Box::into_raw(Box::new(HashNode {
            key,
            value: UnsafeCell::new(value),
            hash,
            next: AtomicPtr::new(ptr::null_mut()),
        }))
    }
}

pub struct LockFreeHashMap<K, V> {
    buckets: Vec<AtomicPtr<HashNode<K, V>>>,
    count: AtomicUsize,
    capacity: usize,
}

impl<K, V> LockFreeHashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(AtomicPtr::new(ptr::null_mut()));
        }

        LockFreeHashMap {
            buckets,
            count: AtomicUsize::new(0),
            capacity,
        }
    }

    fn hash(&self, key: &K) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn bucket_index(&self, hash: u64) -> usize {
        (hash % self.capacity as u64) as usize
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let hash = self.hash(&key);
        let bucket_idx = self.bucket_index(hash);
        let new_node = HashNode::new(key, value, hash);

        loop {
            let head = self.buckets[bucket_idx].load(Ordering::Acquire);
            let mut current = head;

            while !current.is_null() {
                unsafe {
                    if (*current).hash == hash && (*current).key == (*new_node).key {
                        let old_value = ptr::read((*current).value.get());
                        ptr::write((*current).value.get(), ptr::read((*new_node).value.get()));
                        drop(Box::from_raw(new_node));
                        return Some(old_value);
                    }
                    current = (*current).next.load(Ordering::Acquire);
                }
            }

            unsafe {
                (*new_node).next.store(head, Ordering::Release);
            }

            if self.buckets[bucket_idx].compare_exchange(
                head,
                new_node,
                Ordering::Release,
                Ordering::Acquire,
            ).is_ok() {
                self.count.fetch_add(1, Ordering::Relaxed);
                return None;
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Copy,
    {
        let hash = self.hash(key);
        let bucket_idx = self.bucket_index(hash);
        let mut current = self.buckets[bucket_idx].load(Ordering::Acquire);

        while !current.is_null() {
            unsafe {
                if (*current).hash == hash && (*current).key == *key {
                    return Some(ptr::read((*current).value.get()));
                }
                current = (*current).next.load(Ordering::Acquire);
            }
        }

        None
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let bucket_idx = self.bucket_index(hash);

        loop {
            let head = self.buckets[bucket_idx].load(Ordering::Acquire);
            let mut current = head;
            let mut prev: *mut HashNode<K, V> = ptr::null_mut();

            while !current.is_null() {
                unsafe {
                    if (*current).hash == hash && (*current).key == *key {
                        let next = (*current).next.load(Ordering::Acquire);

                        if prev.is_null() {
                            if self.buckets[bucket_idx].compare_exchange(
                                current,
                                next,
                                Ordering::Release,
                                Ordering::Acquire,
                            ).is_ok() {
                                self.count.fetch_sub(1, Ordering::Relaxed);
                                let value = ptr::read((*current).value.get());
                                drop(Box::from_raw(current));
                                return Some(value);
                            } else {
                                break;
                            }
                        } else {
                            (*prev).next.store(next, Ordering::Release);
                            self.count.fetch_sub(1, Ordering::Relaxed);
                            let value = ptr::read((*current).value.get());
                            drop(Box::from_raw(current));
                            return Some(value);
                        }
                    }

                    prev = current;
                    current = (*current).next.load(Ordering::Acquire);
                }
            }

            if current.is_null() {
                return None;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

impl<K, V> Drop for LockFreeHashMap<K, V> {
    fn drop(&mut self) {
        for bucket in &self.buckets {
            let mut current = bucket.load(Ordering::Acquire);
            while !current.is_null() {
                unsafe {
                    let next = (*current).next.load(Ordering::Acquire);
                    drop(Box::from_raw(current));
                    current = next;
                }
            }
        }
    }
}

struct SkipNode<T> {
    value: ManuallyDrop<T>,
    level: usize,
    next: Vec<AtomicPtr<SkipNode<T>>>,
}

impl<T> SkipNode<T> {
    fn new(value: T, level: usize) -> *mut Self {
        let mut next = Vec::with_capacity(level + 1);
        for _ in 0..=level {
            next.push(AtomicPtr::new(ptr::null_mut()));
        }

        Box::into_raw(Box::new(SkipNode {
            value: ManuallyDrop::new(value),
            level,
            next,
        }))
    }
}

pub struct LockFreeSkipList<T> {
    head: AtomicPtr<SkipNode<T>>,
    max_level: usize,
    count: AtomicUsize,
}

impl<T: Ord> LockFreeSkipList<T> {
    pub fn new(max_level: usize) -> Self {
        let head = SkipNode::new(unsafe { mem::zeroed() }, max_level);

        LockFreeSkipList {
            head: AtomicPtr::new(head),
            max_level,
            count: AtomicUsize::new(0),
        }
    }

    fn random_level(&self) -> usize {
        let mut level = 0;
        let mut rng = rand::random::<u32>();
        
        while level < self.max_level && (rng & 1) == 0 {
            level += 1;
            rng >>= 1;
        }
        
        level
    }

    pub fn insert(&self, value: T) -> bool {
        let level = self.random_level();
        let new_node = SkipNode::new(value, level);
        let head = self.head.load(Ordering::Acquire);

        let mut update: Vec<*mut SkipNode<T>> = vec![ptr::null_mut(); self.max_level + 1];
        let mut current = head;

        for i in (0..=self.max_level).rev() {
            unsafe {
                while !(*current).next[i].load(Ordering::Acquire).is_null() {
                    let next = (*current).next[i].load(Ordering::Acquire);
                    if (*next).value >= (*new_node).value {
                        break;
                    }
                    current = next;
                }
                update[i] = current;
            }
        }

        for i in 0..=level {
            loop {
                let next = unsafe { (*update[i]).next[i].load(Ordering::Acquire) };
                unsafe {
                    (*new_node).next[i].store(next, Ordering::Release);
                }

                if unsafe {
                    (*update[i]).next[i].compare_exchange(
                        next,
                        new_node,
                        Ordering::Release,
                        Ordering::Acquire,
                    ).is_ok()
                } {
                    break;
                }
            }
        }

        self.count.fetch_add(1, Ordering::Relaxed);
        true
    }

    pub fn contains(&self, value: &T) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let mut current = head;

        for i in (0..=self.max_level).rev() {
            unsafe {
                while !(*current).next[i].load(Ordering::Acquire).is_null() {
                    let next = (*current).next[i].load(Ordering::Acquire);
                    if &*(*next).value > value {
                        break;
                    }
                    if &*(*next).value == value {
                        return true;
                    }
                    current = next;
                }
            }
        }

        false
    }

    pub fn len(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

impl<T> Drop for LockFreeSkipList<T> {
    fn drop(&mut self) {
        let head = self.head.load(Ordering::Acquire);
        let mut current = unsafe { (*head).next[0].load(Ordering::Acquire) };

        unsafe {
            drop(Box::from_raw(head));
        }

        while !current.is_null() {
            unsafe {
                let next = (*current).next[0].load(Ordering::Acquire);
                drop(Box::from_raw(current));
                current = next;
            }
        }
    }
}

struct RingNode<T> {
    value: UnsafeCell<Option<T>>,
    sequence: AtomicUsize,
}

pub struct LockFreeRingBuffer<T> {
    buffer: Vec<RingNode<T>>,
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T> LockFreeRingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for i in 0..capacity {
            buffer.push(RingNode {
                value: UnsafeCell::new(None),
                sequence: AtomicUsize::new(i),
            });
        }

        LockFreeRingBuffer {
            buffer,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let node = &self.buffer[tail % self.capacity];
            let seq = node.sequence.load(Ordering::Acquire);

            if seq == tail {
                if self.tail.compare_exchange(
                    tail,
                    tail + 1,
                    Ordering::Release,
                    Ordering::Acquire,
                ).is_ok() {
                    unsafe {
                        *node.value.get() = Some(value);
                    }
                    node.sequence.store(tail + 1, Ordering::Release);
                    return Ok(());
                }
            } else if seq < tail {
                return Err(value);
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let node = &self.buffer[head % self.capacity];
            let seq = node.sequence.load(Ordering::Acquire);

            if seq == head + 1 {
                if self.head.compare_exchange(
                    head,
                    head + 1,
                    Ordering::Release,
                    Ordering::Acquire,
                ).is_ok() {
                    let value = unsafe { (*node.value.get()).take() };
                    node.sequence.store(head + self.capacity, Ordering::Release);
                    return value;
                }
            } else if seq < head + 1 {
                return None;
            }
        }
    }
}

mod rand {
    pub fn random<T>() -> T
    where
        T: Default,
    {
        T::default()
    }
}