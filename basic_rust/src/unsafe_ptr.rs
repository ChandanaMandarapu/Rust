use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::slice;

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "ZST");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };
        let new_ptr = unsafe { alloc(new_layout) as *mut T };
        if new_ptr.is_null() { std::alloc::handle_alloc_layout(new_layout) }
        unsafe {
            ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr, self.cap);
            if self.cap != 0 {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.ptr.as_ptr() as *mut u8, old_layout);
            }
        }
        self.ptr = unsafe { NonNull::new_unchecked(new_ptr) };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout) }
        }
    }
}

struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }
    fn push(&mut self, elem: T) {
        if self.len == self.buf.cap { self.buf.grow(); }
        unsafe {
            ptr::write(self.buf.ptr.as_ptr().add(self.len), elem);
        }
        self.len += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 { None } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.buf.ptr.as_ptr().add(self.len))) }
        }
    }
    fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len) }
    }
    fn into_raw_parts(self) -> (*mut T, usize, usize) {
        let ptr = self.buf.ptr.as_ptr();
        let len = self.len;
        let cap = self.buf.cap;
        std::mem::forget(self);
        (ptr, len, cap)
    }
    unsafe fn from_raw_parts(ptr: *mut T, len: usize, cap: usize) -> Self {
        Self {
            buf: RawVec { ptr: NonNull::new_unchecked(ptr), cap },
            len,
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len));
        }
    }
}

fn main() {
    let mut v = Vec::new();
    for i in 0..5000 { v.push(i); }
    let sum: i32 = v.as_slice().iter().sum();
    let (ptr, len, cap) = v.into_raw_parts();
    let rebuilt = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("{}", sum);
    std::mem::drop(rebuilt);
}