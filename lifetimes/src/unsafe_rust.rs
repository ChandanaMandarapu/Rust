// File 17: Unsafe Rust
// Raw pointers, manual memory management, union types, inline assembly concepts,
// building safe abstractions over unsafe code, and FFI patterns
//
// Every unsafe block is documented with SAFETY comments explaining the invariants

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::fmt;
use std::mem;
use std::ptr;

// ─── Raw Pointers ─────────────────────────────────────────────────────────────

fn demonstrate_raw_pointers() {
    println!("── Raw Pointers ──");

    let x = 42i32;

    // Creating raw pointers — safe, no dereference yet
    let ptr_const: *const i32 = &x;
    let ptr_mut: *mut i32 = &x as *const i32 as *mut i32;

    // Dereferencing is unsafe
    unsafe {
        // SAFETY: ptr_const was created from a valid &x, which is still live
        println!("*ptr_const = {}", *ptr_const);

        // SAFETY: ptr_mut points to stack memory we own; we must not alias
        // In practice mutating through *const cast is UB — demo only
        // *ptr_mut = 99;  // would be UB
        let _ = ptr_mut;
    }

    // Null pointer check
    let null: *const i32 = ptr::null();
    println!("null.is_null() = {}", null.is_null());

    // Pointer arithmetic
    let arr = [10i32, 20, 30, 40, 50];
    let base: *const i32 = arr.as_ptr();
    unsafe {
        // SAFETY: arr has 5 elements, offsets 0..4 are valid
        for i in 0..5 {
            let val = *base.add(i);
            print!("{} ", val);
        }
    }
    println!();

    // offset_from
    let first = arr.as_ptr();
    let third = unsafe { first.add(2) };
    unsafe {
        // SAFETY: both pointers into the same slice
        let diff = third.offset_from(first);
        println!("offset_from: {}", diff);
    }
}

// ─── Manual Heap Allocation ───────────────────────────────────────────────────

fn demonstrate_manual_alloc() {
    println!("\n── Manual Alloc ──");

    let layout = Layout::new::<i32>();

    unsafe {
        // SAFETY: layout is non-zero sized
        let ptr = alloc(layout) as *mut i32;
        if ptr.is_null() { panic!("allocation failed"); }

        // SAFETY: ptr is valid, aligned, and we just allocated it
        ptr::write(ptr, 777);
        println!("manually allocated: {}", ptr::read(ptr));

        // SAFETY: ptr was allocated with this exact layout
        dealloc(ptr as *mut u8, layout);
    }
}

// ─── Safe Vec Wrapper (unsafe internals, safe API) ────────────────────────────

struct RawVec<T> {
    ptr: ptr::NonNull<T>,
    cap: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        // Zero-sized types — no allocation needed
        let cap = if mem::size_of::<T>() == 0 { usize::MAX } else { 0 };
        RawVec {
            ptr: ptr::NonNull::dangling(),
            cap,
            _marker: std::marker::PhantomData,
        }
    }

    fn with_capacity(cap: usize) -> Self {
        if mem::size_of::<T>() == 0 || cap == 0 {
            return Self::new();
        }
        let layout = Layout::array::<T>(cap).expect("layout overflow");
        unsafe {
            // SAFETY: layout is valid and non-zero
            let raw = alloc(layout) as *mut T;
            let ptr = ptr::NonNull::new(raw).expect("allocation failed");
            RawVec { ptr, cap, _marker: std::marker::PhantomData }
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        unsafe {
            let new_ptr = if self.cap == 0 {
                // SAFETY: new_layout is valid and non-zero
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                // SAFETY: ptr was allocated with old_layout
                realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
            };
            self.ptr = ptr::NonNull::new(new_ptr as *mut T).expect("realloc failed");
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 && mem::size_of::<T>() != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                // SAFETY: ptr was allocated with this layout
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

pub struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self { MyVec { buf: RawVec::new(), len: 0 } }

    pub fn push(&mut self, value: T) {
        if self.len == self.buf.cap { self.buf.grow(); }
        unsafe {
            // SAFETY: self.len < cap after grow; slot is uninitialized
            ptr::write(self.buf.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        unsafe {
            // SAFETY: self.len was valid, element is initialized
            Some(ptr::read(self.buf.ptr.as_ptr().add(self.len)))
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len { return None; }
        unsafe {
            // SAFETY: index < self.len, which are all initialized
            Some(&*self.buf.ptr.as_ptr().add(index))
        }
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn capacity(&self) -> usize { self.buf.cap }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // Drop each element in order
        while self.pop().is_some() {}
        // RawVec's Drop then frees the memory
    }
}

impl<T: fmt::Debug> fmt::Display for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyVec[")?;
        for i in 0..self.len {
            if i > 0 { write!(f, ", ")?; }
            unsafe {
                // SAFETY: i < self.len, all initialized
                write!(f, "{:?}", &*self.buf.ptr.as_ptr().add(i))?;
            }
        }
        write!(f, "]")
    }
}

// ─── Union Types ──────────────────────────────────────────────────────────────

#[repr(C)]
union FloatBits {
    f: f32,
    bits: u32,
}

fn float_to_bits(f: f32) -> u32 {
    unsafe {
        // SAFETY: f32 and u32 are the same size; any bit pattern is valid for u32
        FloatBits { f }.bits
    }
}

fn bits_to_float(bits: u32) -> f32 {
    unsafe {
        // SAFETY: any bit pattern is a valid f32 (may be NaN/Inf, but not UB)
        FloatBits { bits }.f
    }
}

// Fast inverse square root (the famous Quake algorithm)
fn fast_inverse_sqrt(x: f32) -> f32 {
    let x2 = x * 0.5;
    let bits = float_to_bits(x);
    let bits = 0x5f3759df_u32.wrapping_sub(bits >> 1);
    let y = bits_to_float(bits);
    // One Newton–Raphson iteration
    y * (1.5 - x2 * y * y)
}

// Tagged union for a dynamic value type
#[repr(u8)]
#[derive(Clone, Copy)]
enum Tag { Int = 0, Float = 1, Bool = 2 }

#[repr(C)]
union Payload {
    int_val:   i64,
    float_val: f64,
    bool_val:  bool,
}

struct DynValue {
    tag: Tag,
    payload: Payload,
}

impl DynValue {
    fn int(n: i64) -> Self { DynValue { tag: Tag::Int, payload: Payload { int_val: n } } }
    fn float(f: f64) -> Self { DynValue { tag: Tag::Float, payload: Payload { float_val: f } } }
    fn bool(b: bool) -> Self { DynValue { tag: Tag::Bool, payload: Payload { bool_val: b } } }

    fn as_int(&self) -> Option<i64> {
        match self.tag {
            Tag::Int => Some(unsafe { self.payload.int_val }),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match self.tag {
            Tag::Float => Some(unsafe { self.payload.float_val }),
            _ => None,
        }
    }
}

impl fmt::Display for DynValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self.tag {
                Tag::Int   => write!(f, "Int({})",   self.payload.int_val),
                Tag::Float => write!(f, "Float({})", self.payload.float_val),
                Tag::Bool  => write!(f, "Bool({})",  self.payload.bool_val),
            }
        }
    }
}

// ─── mem::transmute ───────────────────────────────────────────────────────────

fn demonstrate_transmute() {
    println!("\n── transmute ──");

    // Reinterpret bytes of one type as another (same size)
    let n: u64 = 0x4065_8000_0000_0000;
    let f: f64 = unsafe {
        // SAFETY: f64 and u64 are both 8 bytes; any u64 is a valid f64 bit pattern
        mem::transmute(n)
    };
    println!("u64 0x{:016X} as f64 = {}", n, f);

    // Casting a function pointer
    fn add_one(x: i32) -> i32 { x + 1 }
    let fn_ptr: fn(i32) -> i32 = add_one;
    let fn_as_ptr: *const () = fn_ptr as *const ();
    let back: fn(i32) -> i32 = unsafe {
        // SAFETY: same function pointer type round-trip
        mem::transmute(fn_as_ptr)
    };
    println!("fn ptr round-trip: add_one(41) = {}", back(41));

    // Extending a lifetime (DANGEROUS — demo only)
    // This is the canonical example of why transmute is dangerous
    let local = String::from("temporary");
    let extended: &'static str = unsafe {
        // SAFETY: We're lying to the compiler here — local will be dropped.
        // This is UNSOUND in general; we keep it alive for this scope.
        let r: &str = &local;
        mem::transmute::<&str, &'static str>(r)
    };
    // Use before local is dropped — still valid HERE
    println!("transmuted str: {}", extended);
    drop(local);
    // extended is dangling now — do NOT use it after this point
}

// ─── ptr::read / write / copy ─────────────────────────────────────────────────

fn demonstrate_ptr_ops() {
    println!("\n── ptr read/write/copy ──");

    let src = vec![1u32, 2, 3, 4, 5];
    let mut dst = vec![0u32; 5];

    unsafe {
        // SAFETY: src and dst are non-overlapping slices of length 5
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), 5);
    }
    println!("copy_nonoverlapping: {:?}", dst);

    // ptr::swap
    let mut a = 100i32;
    let mut b = 200i32;
    unsafe {
        // SAFETY: both are valid, aligned, non-overlapping i32 locations
        ptr::swap(&mut a, &mut b);
    }
    println!("after swap: a={}, b={}", a, b);

    // ptr::replace
    let mut val = String::from("old");
    let old = unsafe {
        // SAFETY: val is valid and we immediately put a new value in
        ptr::replace(&mut val, String::from("new"))
    };
    println!("replaced '{}' with '{}'", old, val);
}

// ─── Slice from raw parts ─────────────────────────────────────────────────────

fn demonstrate_slice_from_raw() {
    println!("\n── slice_from_raw_parts ──");

    let data = vec![10u8, 20, 30, 40, 50];
    let ptr = data.as_ptr();
    let len = data.len();

    let slice = unsafe {
        // SAFETY: ptr is valid for len elements, properly aligned, initialized
        std::slice::from_raw_parts(ptr, len)
    };
    println!("slice: {:?}", slice);

    // Building a str from bytes
    let bytes = b"hello unsafe world";
    let s = unsafe {
        // SAFETY: bytes is valid UTF-8
        std::str::from_utf8_unchecked(bytes)
    };
    println!("str from raw: {}", s);
}

// ─── Building a Safe Spinlock over Unsafe ────────────────────────────────────

use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;

struct SpinLock<T> {
    locked: AtomicBool,
    data:   UnsafeCell<T>,
}

// SAFETY: SpinLock protects T with a spin-lock; access is always exclusive
unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    fn new(data: T) -> Self {
        SpinLock { locked: AtomicBool::new(false), data: UnsafeCell::new(data) }
    }

    fn lock(&self) -> SpinGuard<T> {
        // Spin until we acquire
        while self.locked.compare_exchange_weak(
            false, true,
            Ordering::Acquire, Ordering::Relaxed,
        ).is_err() {
            std::hint::spin_loop();
        }
        SpinGuard { lock: self }
    }
}

impl<T> std::ops::Deref for SpinGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            // SAFETY: We hold the lock, so we have exclusive access
            &*self.lock.data.get()
        }
    }
}

impl<T> std::ops::DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            // SAFETY: We hold the lock exclusively
            &mut *self.lock.data.get()
        }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

// ─── MaybeUninit ──────────────────────────────────────────────────────────────

fn demonstrate_maybeuninit() {
    println!("\n── MaybeUninit ──");
    use std::mem::MaybeUninit;

    // Initialize an array without requiring Default
    let mut arr: [MaybeUninit<String>; 3] = unsafe {
        // SAFETY: MaybeUninit doesn't require initialization
        MaybeUninit::uninit().assume_init()
    };

    arr[0].write(String::from("first"));
    arr[1].write(String::from("second"));
    arr[2].write(String::from("third"));

    let initialized = unsafe {
        // SAFETY: all elements were written above
        arr.map(|x| x.assume_init())
    };
    println!("MaybeUninit array: {:?}", initialized);

    // Single uninit value used as out-parameter
    let mut result = MaybeUninit::<i32>::uninit();
    unsafe {
        result.as_mut_ptr().write(42);
        println!("uninit i32: {}", result.assume_init());
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Unsafe Rust ===\n");

    demonstrate_raw_pointers();
    demonstrate_manual_alloc();

    // MyVec
    println!("\n── MyVec ──");
    let mut v: MyVec<i32> = MyVec::new();
    for i in 0..8 { v.push(i * 10); }
    println!("{}", v);
    println!("len={}, cap={}", v.len(), v.capacity());
    println!("get(3): {:?}", v.get(3));
    println!("pop:    {:?}", v.pop());
    println!("{}", v);

    // Unions
    println!("\n── Union & float bits ──");
    let pi = std::f32::consts::PI;
    let bits = float_to_bits(pi);
    let back = bits_to_float(bits);
    println!("PI bits: 0x{:08X}, back: {}", bits, back);
    println!("fast_inv_sqrt(4.0) = {:.6}  (expected ≈ 0.5)", fast_inverse_sqrt(4.0));
    println!("fast_inv_sqrt(9.0) = {:.6}  (expected ≈ 0.333)", fast_inverse_sqrt(9.0));

    // DynValue union
    let values = vec![
        DynValue::int(42),
        DynValue::float(3.14),
        DynValue::bool(true),
    ];
    for v in &values { println!("  {}", v); }
    println!("as_int:  {:?}", values[0].as_int());
    println!("as_float:{:?}", values[1].as_float());

    demonstrate_transmute();
    demonstrate_ptr_ops();
    demonstrate_slice_from_raw();
    demonstrate_maybeuninit();

    // SpinLock
    println!("\n── SpinLock ──");
    let lock = SpinLock::new(vec![1, 2, 3]);
    {
        let mut guard = lock.lock();
        guard.push(4);
        println!("inside lock: {:?}", *guard);
    } // guard drops, lock released
    {
        let guard = lock.lock();
        println!("after release: {:?}", *guard);
    }

    println!("\n=== Done ===");
}