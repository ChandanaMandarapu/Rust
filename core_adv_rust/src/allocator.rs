// File 21: Custom Allocators — Bump, Slab, Arena, Pool
// Building allocators from scratch: bump pointer, slab allocator,
// typed pool allocator, and a generational arena. All with safe wrappers.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::UnsafeCell;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::marker::PhantomData;
use std::fmt;

// ─── Bump Allocator ───────────────────────────────────────────────────────────
// Fastest possible allocator: just move a pointer forward.
// No individual deallocation — free everything at once ("reset").

const BUMP_SIZE: usize = 1024 * 64; // 64 KB

struct BumpAllocator {
    memory: UnsafeCell<[u8; BUMP_SIZE]>,
    offset: AtomicUsize,
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    const fn new() -> Self {
        BumpAllocator {
            memory: UnsafeCell::new([0u8; BUMP_SIZE]),
            offset: AtomicUsize::new(0),
        }
    }

    fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size  = layout.size();

        loop {
            let current = self.offset.load(Ordering::Relaxed);
            // Align up
            let aligned = (current + align - 1) & !(align - 1);
            let new_offset = aligned + size;

            if new_offset > BUMP_SIZE {
                return ptr::null_mut(); // OOM
            }

            match self.offset.compare_exchange_weak(
                current, new_offset,
                Ordering::SeqCst, Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let base = self.memory.get() as *mut u8;
                    return unsafe { base.add(aligned) };
                }
                Err(_) => continue, // retry
            }
        }
    }

    fn reset(&self) {
        self.offset.store(0, Ordering::SeqCst);
    }

    fn used(&self) -> usize {
        self.offset.load(Ordering::Relaxed)
    }

    fn remaining(&self) -> usize {
        BUMP_SIZE - self.used()
    }

    // Typed allocation
    fn alloc_one<T>(&self) -> Option<*mut T> {
        let layout = Layout::new::<T>();
        let ptr = self.alloc(layout);
        if ptr.is_null() { None } else { Some(ptr as *mut T) }
    }

    fn alloc_slice<T>(&self, count: usize) -> Option<*mut T> {
        let layout = Layout::array::<T>(count).ok()?;
        let ptr = self.alloc(layout);
        if ptr.is_null() { None } else { Some(ptr as *mut T) }
    }
}

// ─── Slab Allocator ───────────────────────────────────────────────────────────
// Pre-allocates fixed-size slots. O(1) alloc and free.
// Maintains a free list of available slots.

const SLAB_CAPACITY: usize = 256;

#[repr(C)]
union SlabSlot<T> {
    value: std::mem::ManuallyDrop<T>,
    next_free: usize, // index of next free slot (u8::MAX = none)
}

struct SlabAllocator<T> {
    slots:      Box<[SlabSlot<T>]>,
    free_head:  usize,
    allocated:  usize,
    capacity:   usize,
}

impl<T: fmt::Debug> SlabAllocator<T> {
    fn new(cap: usize) -> Self {
        let mut slots: Vec<SlabSlot<T>> = Vec::with_capacity(cap);
        // Initialize free list: slot i points to slot i+1
        for i in 0..cap {
            slots.push(SlabSlot { next_free: i + 1 });
        }
        SlabAllocator {
            slots: slots.into_boxed_slice(),
            free_head: 0,
            allocated: 0,
            capacity: cap,
        }
    }

    fn alloc(&mut self, value: T) -> Option<usize> {
        if self.free_head >= self.capacity {
            return None; // full
        }
        let idx = self.free_head;
        unsafe {
            // Advance free head
            self.free_head = self.slots[idx].next_free;
            // Write value into slot
            self.slots[idx].value = std::mem::ManuallyDrop::new(value);
        }
        self.allocated += 1;
        Some(idx)
    }

    fn free(&mut self, idx: usize) {
        assert!(idx < self.capacity, "index out of bounds");
        unsafe {
            // Drop the contained value
            std::mem::ManuallyDrop::drop(&mut self.slots[idx].value);
            // Re-link into free list
            self.slots[idx].next_free = self.free_head;
        }
        self.free_head = idx;
        self.allocated -= 1;
    }

    fn get(&self, idx: usize) -> &T {
        unsafe { &self.slots[idx].value }
    }

    fn get_mut(&mut self, idx: usize) -> &mut T {
        unsafe { &mut self.slots[idx].value }
    }

    fn len(&self) -> usize { self.allocated }
    fn is_full(&self) -> bool { self.free_head >= self.capacity }
}

impl<T> Drop for SlabAllocator<T> {
    fn drop(&mut self) {
        // No easy way to track which slots are live without a bitmap
        // In production code you'd maintain an occupancy bitset
    }
}

// ─── Generational Arena ───────────────────────────────────────────────────────
// Like a slab, but each slot has a "generation" counter.
// Stale handles (old generation) are automatically invalid — no use-after-free.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ArenaHandle {
    idx: u32,
    gen: u32,
}

impl fmt::Display for ArenaHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handle(idx={}, gen={})", self.idx, self.gen)
    }
}

struct ArenaEntry<T> {
    gen:   u32,
    value: Option<T>,
}

struct GenerationalArena<T> {
    entries:   Vec<ArenaEntry<T>>,
    free_list: Vec<u32>,
    len:       usize,
}

impl<T: fmt::Debug> GenerationalArena<T> {
    fn new(capacity: usize) -> Self {
        let mut entries = Vec::with_capacity(capacity);
        let mut free_list = Vec::with_capacity(capacity);
        for i in 0..capacity {
            entries.push(ArenaEntry { gen: 0, value: None });
            free_list.push(i as u32);
        }
        GenerationalArena { entries, free_list, len: 0 }
    }

    fn insert(&mut self, value: T) -> Option<ArenaHandle> {
        let idx = self.free_list.pop()? as usize;
        let gen = self.entries[idx].gen;
        self.entries[idx].value = Some(value);
        self.len += 1;
        Some(ArenaHandle { idx: idx as u32, gen })
    }

    fn remove(&mut self, handle: ArenaHandle) -> Option<T> {
        let e = self.entries.get_mut(handle.idx as usize)?;
        if e.gen != handle.gen || e.value.is_none() {
            return None; // stale handle
        }
        e.gen += 1; // invalidate all existing handles to this slot
        self.free_list.push(handle.idx);
        self.len -= 1;
        e.value.take()
    }

    fn get(&self, handle: ArenaHandle) -> Option<&T> {
        let e = self.entries.get(handle.idx as usize)?;
        if e.gen != handle.gen { return None; } // stale
        e.value.as_ref()
    }

    fn get_mut(&mut self, handle: ArenaHandle) -> Option<&mut T> {
        let e = self.entries.get_mut(handle.idx as usize)?;
        if e.gen != handle.gen { return None; }
        e.value.as_mut()
    }

    fn iter(&self) -> impl Iterator<Item = (ArenaHandle, &T)> {
        self.entries.iter().enumerate().filter_map(|(i, e)| {
            e.value.as_ref().map(|v| (ArenaHandle { idx: i as u32, gen: e.gen }, v))
        })
    }

    fn len(&self) -> usize { self.len }
    fn is_empty(&self) -> bool { self.len == 0 }
}

// ─── Pool Allocator (typed, fixed-size object pool) ──────────────────────────

struct PoolAllocator<T> {
    storage: Vec<mem::MaybeUninit<T>>,
    free:    Vec<usize>,
    used:    usize,
}

impl<T> PoolAllocator<T> {
    fn new(capacity: usize) -> Self {
        PoolAllocator {
            storage: (0..capacity).map(|_| mem::MaybeUninit::uninit()).collect(),
            free: (0..capacity).rev().collect(),
            used: 0,
        }
    }

    fn acquire(&mut self, init: T) -> Option<PoolRef<T>> {
        let idx = self.free.pop()?;
        unsafe { self.storage[idx].as_mut_ptr().write(init); }
        self.used += 1;
        Some(PoolRef { idx, _marker: PhantomData })
    }

    fn release(&mut self, r: PoolRef<T>) {
        unsafe { ptr::drop_in_place(self.storage[r.idx].as_mut_ptr()); }
        self.free.push(r.idx);
        self.used -= 1;
        mem::forget(r); // prevent double-free in Drop
    }

    fn get(&self, r: &PoolRef<T>) -> &T {
        unsafe { &*self.storage[r.idx].as_ptr() }
    }

    fn get_mut(&mut self, r: &PoolRef<T>) -> &mut T {
        unsafe { &mut *self.storage[r.idx].as_mut_ptr() }
    }

    fn capacity(&self) -> usize { self.storage.len() }
    fn used(&self) -> usize { self.used }
    fn available(&self) -> usize { self.free.len() }
}

impl<T> Drop for PoolAllocator<T> {
    fn drop(&mut self) {
        // Drop all live items (not in free list)
        let free_set: std::collections::HashSet<usize> = self.free.iter().cloned().collect();
        for (i, slot) in self.storage.iter_mut().enumerate() {
            if !free_set.contains(&i) {
                unsafe { ptr::drop_in_place(slot.as_mut_ptr()); }
            }
        }
    }
}

struct PoolRef<T> {
    idx:     usize,
    _marker: PhantomData<T>,
}

// ─── Tracking Allocator (wraps System, counts bytes) ─────────────────────────

struct TrackingAllocator {
    allocated: AtomicUsize,
    freed:     AtomicUsize,
    calls:     AtomicUsize,
}

impl TrackingAllocator {
    const fn new() -> Self {
        TrackingAllocator {
            allocated: AtomicUsize::new(0),
            freed:     AtomicUsize::new(0),
            calls:     AtomicUsize::new(0),
        }
    }

    fn stats(&self) -> (usize, usize, usize) {
        (
            self.allocated.load(Ordering::Relaxed),
            self.freed.load(Ordering::Relaxed),
            self.calls.load(Ordering::Relaxed),
        )
    }

    fn live_bytes(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
            .saturating_sub(self.freed.load(Ordering::Relaxed))
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
        self.calls.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.freed.fetch_add(layout.size(), Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            self.allocated.fetch_add(new_size - layout.size(), Ordering::Relaxed);
        } else {
            self.freed.fetch_add(layout.size() - new_size, Ordering::Relaxed);
        }
        System.realloc(ptr, layout, new_size)
    }
}

// Uncomment to use as global allocator (only one allowed per binary):
// #[global_allocator]
// static GLOBAL: TrackingAllocator = TrackingAllocator::new();

// ─── ECS-style Component Storage ─────────────────────────────────────────────
// An archetypal storage pattern: entities identified by ArenaHandle,
// components stored in parallel vecs for cache locality.

#[derive(Debug, Clone)]
struct Position { x: f32, y: f32 }

#[derive(Debug, Clone)]
struct Velocity { dx: f32, dy: f32 }

#[derive(Debug, Clone)]
struct Health { hp: i32, max: i32 }

struct World {
    positions:  GenerationalArena<Position>,
    velocities: Vec<Option<(ArenaHandle, Velocity)>>,
    health:     Vec<Option<(ArenaHandle, Health)>>,
    next_vel_slot: usize,
    next_hp_slot:  usize,
}

impl World {
    fn new(cap: usize) -> Self {
        World {
            positions:     GenerationalArena::new(cap),
            velocities:    vec![None; cap],
            health:        vec![None; cap],
            next_vel_slot: 0,
            next_hp_slot:  0,
        }
    }

    fn spawn(&mut self, pos: Position) -> Option<ArenaHandle> {
        self.positions.insert(pos)
    }

    fn add_velocity(&mut self, handle: ArenaHandle, vel: Velocity) {
        let slot = self.next_vel_slot;
        self.next_vel_slot += 1;
        if slot < self.velocities.len() {
            self.velocities[slot] = Some((handle, vel));
        }
    }

    fn add_health(&mut self, handle: ArenaHandle, hp: Health) {
        let slot = self.next_hp_slot;
        self.next_hp_slot += 1;
        if slot < self.health.len() {
            self.health[slot] = Some((handle, hp));
        }
    }

    fn update_physics(&mut self, dt: f32) {
        for vel_slot in &self.velocities {
            if let Some((handle, vel)) = vel_slot {
                if let Some(pos) = self.positions.get_mut(*handle) {
                    pos.x += vel.dx * dt;
                    pos.y += vel.dy * dt;
                }
            }
        }
    }

    fn apply_damage(&mut self, handle: ArenaHandle, dmg: i32) {
        for hp_slot in &mut self.health {
            if let Some((h, hp)) = hp_slot {
                if *h == handle {
                    hp.hp = (hp.hp - dmg).max(0);
                }
            }
        }
    }

    fn entity_count(&self) -> usize { self.positions.len() }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Custom Allocators ===\n");

    // Bump allocator
    println!("── Bump Allocator ──");
    let bump = BumpAllocator::new();

    if let Some(p) = bump.alloc_one::<u64>() {
        unsafe { ptr::write(p, 0xDEAD_BEEF_u64); }
        unsafe { println!("  u64 @ {:p} = 0x{:X}", p, ptr::read(p)); }
    }

    if let Some(arr) = bump.alloc_slice::<f32>(8) {
        for i in 0..8usize {
            unsafe { ptr::write(arr.add(i), i as f32 * 1.5); }
        }
        let slice = unsafe { std::slice::from_raw_parts(arr, 8) };
        println!("  f32 slice: {:?}", slice);
    }

    println!("  used: {} / {} bytes", bump.used(), BUMP_SIZE);
    bump.reset();
    println!("  after reset: {} bytes used", bump.used());

    // Slab allocator
    println!("\n── Slab Allocator ──");
    let mut slab: SlabAllocator<String> = SlabAllocator::new(8);

    let i0 = slab.alloc("hello".to_string()).unwrap();
    let i1 = slab.alloc("world".to_string()).unwrap();
    let i2 = slab.alloc("rust".to_string()).unwrap();
    println!("  allocated slots: {} {} {}", i0, i1, i2);
    println!("  [{}] = {}", i0, slab.get(i0));
    println!("  [{}] = {}", i2, slab.get(i2));

    slab.free(i1);
    println!("  after free slot {}: len={}", i1, slab.len());

    let i3 = slab.alloc("reused".to_string()).unwrap();
    println!("  reallocated at slot {}: {}", i3, slab.get(i3));

    // Generational arena
    println!("\n── Generational Arena ──");
    let mut arena: GenerationalArena<String> = GenerationalArena::new(16);

    let h0 = arena.insert("Alice".to_string()).unwrap();
    let h1 = arena.insert("Bob".to_string()).unwrap();
    let h2 = arena.insert("Charlie".to_string()).unwrap();
    println!("  h0={}, h1={}, h2={}", h0, h1, h2);
    println!("  arena[h1] = {:?}", arena.get(h1));

    let removed = arena.remove(h1).unwrap();
    println!("  removed: {}", removed);

    // h1 is now stale — generation mismatch
    println!("  stale get h1: {:?}", arena.get(h1));

    let h3 = arena.insert("Dave".to_string()).unwrap();
    println!("  new handle h3={}, value={:?}", h3, arena.get(h3));

    println!("  all entries:");
    for (h, v) in arena.iter() {
        println!("    {} → {}", h, v);
    }

    // Pool allocator
    println!("\n── Pool Allocator ──");
    let mut pool: PoolAllocator<Vec<u8>> = PoolAllocator::new(4);
    println!("  cap={}, avail={}", pool.capacity(), pool.available());

    let r0 = pool.acquire(vec![1, 2, 3]).unwrap();
    let r1 = pool.acquire(vec![4, 5, 6]).unwrap();
    println!("  r0: {:?}", pool.get(&r0));
    println!("  r1: {:?}", pool.get(&r1));
    println!("  used={}, avail={}", pool.used(), pool.available());

    pool.release(r0);
    println!("  after release: used={}", pool.used());

    let r2 = pool.acquire(vec![7, 8, 9]).unwrap();
    println!("  r2 (reused slot): {:?}", pool.get(&r2));
    pool.release(r1);
    pool.release(r2);

    // Tracking allocator
    println!("\n── Tracking Allocator ──");
    let tracker = TrackingAllocator::new();
    // Simulate some allocations going through tracker
    println!("  (tracking allocator ready — use as #[global_allocator] for real tracking)");
    println!("  stats: allocated={}, freed={}, calls={}",
        tracker.stats().0, tracker.stats().1, tracker.stats().2);

    // ECS world
    println!("\n── ECS World (Generational Arena + Components) ──");
    let mut world = World::new(32);

    let player = world.spawn(Position { x: 0.0, y: 0.0 }).unwrap();
    let enemy  = world.spawn(Position { x: 10.0, y: 5.0 }).unwrap();
    let bullet = world.spawn(Position { x: 1.0, y: 0.0 }).unwrap();

    world.add_velocity(player, Velocity { dx: 1.0, dy: 0.5 });
    world.add_velocity(bullet, Velocity { dx: 5.0, dy: 0.0 });
    world.add_health(player, Health { hp: 100, max: 100 });
    world.add_health(enemy,  Health { hp: 50,  max: 50 });

    println!("  entities: {}", world.entity_count());
    println!("  player pos before: {:?}", world.positions.get(player));
    world.update_physics(0.1);
    println!("  player pos after dt=0.1: {:?}", world.positions.get(player));

    world.apply_damage(player, 30);
    for hp_slot in &world.health {
        if let Some((h, hp)) = hp_slot {
            if *h == player {
                println!("  player health: {}/{}", hp.hp, hp.max);
            }
        }
    }

    // Remove an entity — handle becomes stale
    world.positions.remove(bullet);
    println!("  bullet after remove: {:?}", world.positions.get(bullet));

    println!("\n=== Done ===");
}