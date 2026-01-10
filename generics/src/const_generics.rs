use std::ops::{Add, Sub, Mul};
use std::mem::MaybeUninit;

struct Matrix<T, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

impl<T: Copy + Default, const R: usize, const C: usize> Matrix<T, R, C> {
    fn new() -> Self {
        Self { data: [[T::default(); C]; R] }
    }
    fn from_rows(rows: [[T; C]; R]) -> Self {
        Self { data: rows }
    }
    fn transpose(self) -> Matrix<T, C, R> {
        let mut out = Matrix::<T, C, R>::new();
        for i in 0..R {
            for j in 0..C {
                out.data[j][i] = self.data[i][j];
            }
        }
        out
    }
}

impl<T: Add<Output = T> + Copy, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut out = Self::new();
        for i in 0..R {
            for j in 0..C {
                out.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        out
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + Default, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, K>;
    fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
        let mut out = Matrix::<T, R, K>::new();
        for i in 0..R {
            for j in 0..K {
                let mut sum = T::default();
                for k in 0..C {
                    sum = sum + self.data[i][k] * other.data[k][j];
                }
                out.data[i][j] = sum;
            }
        }
        out
    }
}

struct ArrayVec<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    fn new() -> Self {
        Self {
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }
    fn push(&mut self, val: T) {
        assert!(self.len < N);
        self.buf[self.len].write(val);
        self.len += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 { None } else {
            self.len -= 1;
            unsafe { Some(self.buf[self.len].assume_init_read()) }
        }
    }
    fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.buf.as_ptr() as *const T, self.len) }
    }
}

fn main() {
    let a = Matrix::<i32, 3, 4>::from_rows([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let b = a.transpose();
    let c = a * b;
    let mut av = ArrayVec::<i32, 64>::new();
    for i in 0..64 { av.push(i * i); }
    let sum: i32 = av.as_slice().iter().sum();
    println!("{}", sum);
}