use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::io::{self, Write};

struct Grid<T, const W: usize, const H: usize> {
    buf: [T; W * H],
}

impl<T: Clone + Default, const W: usize, const H: usize> Grid<T, W, H> {
    fn new() -> Self {
        Self { buf: [T::default(); W * H] }
    }
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < W && y < H { Some(&self.buf[y * W + x]) } else { None }
    }
    fn set(&mut self, x: usize, y: usize, val: T) {
        if x < W && y < H {
            self.buf[y * W + x] = val;
        }
    }
    fn row(&self, y: usize) -> Option<&[T]> {
        if y < H { Some(&self.buf[y * W..(y + 1) * W]) } else { None }
    }
    fn col(&self, x: usize) -> Vec<T> {
        if x >= W { return Vec::new(); }
        (0..H).map(|y| self.buf[y * W + x].clone()).collect()
    }
    fn map<U: Clone + Default, F: Fn(&T) -> U>(&self, f: F) -> Grid<U, W, H> {
        let mut out = Grid::<U, W, H>::new();
        for i in 0..W * H {
            out.buf[i] = f(&self.buf[i]);
        }
        out
    }
}

impl<T, const W: usize, const H: usize> Index<(usize, usize)> for Grid<T, W, H> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.buf[y * W + x]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<(usize, usize)> for Grid<T, W, H> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.buf[y * W + x]
    }
}

struct Id<T> {
    id: usize,
    _pd: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    fn new(id: usize) -> Self {
        Self { id, _pd: PhantomData }
    }
    fn index(self) -> usize {
        self.id
    }
}

struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn insert(&mut self, item: T) -> Id<T> {
        let id = self.items.len();
        self.items.push(item);
        Id::new(id)
    }
    fn get(&self, id: Id<T>) -> &T {
        &self.items[id.index()]
    }
    fn get_mut(&mut self, id: Id<T>) -> &mut T {
        &mut self.items[id.index()]
    }
}

fn zip_with<T, U, V, F>(a: &[T], b: &[U], mut f: F) -> Vec<V>
where
    F: FnMut(&T, &U) -> V,
{
    a.iter().zip(b).map(|(x, y)| f(x, y)).collect()
}

fn reduce<T, F>(slice: &[T], mut f: F) -> Option<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> T,
{
    if slice.is_empty() {
        None
    } else {
        Some(slice[1..].iter().fold(slice[0].clone(), |acc, x| f(&acc, x)))
    }
}

fn main() {
    let mut g: Grid<u32, 40, 30> = Grid::new();
    for y in 0..30 {
        for x in 0..40 {
            g[(x, y)] = (x + y) as u32;
        }
    }
    let doubled = g.map(|&v| v * 2);
    let mut arena = Arena::new();
    let ids: Vec<_> = (0..100).map(|i| arena.insert(i * i)).collect();
    let sum: i32 = ids.iter().map(|&id| *arena.get(id)).sum();
    let a: Vec<_> = (0..50).collect();
    let b: Vec<_> = (0..50).map(|x| x * 2).collect();
    let c = zip_with(&a, &b, |&x, &y| x + y);
    let max = reduce(&c, |x, y| if x > y { x } else { y }).unwrap_or(0);
    io::stdout().write_all(format!("{} {} {}\n", doubled[(10, 10)], sum, max).as_bytes()).unwrap();
}