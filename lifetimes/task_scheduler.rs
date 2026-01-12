use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::marker::PhantomData;

pub struct Task<'a> {
    future: Pin<Box<dyn Future<Output = ()> + 'a>>,
    id: u64,
}

pub struct Scheduler<'a> {
    tasks: Vec<Task<'a>>,
    queue: std::collections::VecDeque<u64>,
}

impl<'a> Scheduler<'a> {
    pub fn new() -> Self {
        Self { tasks: Vec::new(), queue: std::collections::VecDeque::new() }
    }

    pub fn spawn<F>(&mut self, future: F) 
    where F: Future<Output = ()> + 'a 
    {
        let id = self.tasks.len() as u64;
        self.tasks.push(Task {
            future: Box::pin(future),
            id,
        });
        self.queue.push_back(id);
    }

    pub fn run_one(&mut self) -> bool {
         if let Some(id) = self.queue.pop_front() {
             if let Some(task) = self.tasks.get_mut(id as usize) {
                 let waker = unsafe { Waker::from_raw(std::task::RawWaker::new(std::ptr::null(), &VTABLE)) };
                 let mut ctx = Context::from_waker(&waker);
                 if let Poll::Pending = task.future.as_mut().poll(&mut ctx) {
                     self.queue.push_back(id);
                 }
             }
             true
         } else {
             false
         }
    }
}

static VTABLE: std::task::RawWakerVTable = std::task::RawWakerVTable::new(
    |_| std::task::RawWaker::new(std::ptr::null(), &VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

pub struct Sleeper<'a> {
    until: std::time::Instant,
    marker: PhantomData<&'a ()>,
}

impl<'a> Future for Sleeper<'a> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if std::time::Instant::now() >= self.until {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct ScopedTask<'a, T> {
    data: &'a mut T,
}

impl<'a, T> Future for ScopedTask<'a, T> {
    type Output = &'a T; 
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

pub struct AsyncChannel<'a, T> {
    buffer: Vec<T>,
    _marker: PhantomData<&'a T>,
}

pub struct Receiver<'a, T> {
    chan: &'a AsyncChannel<'a, T>,
}

pub struct Sender<'a, T> {
    chan: &'a AsyncChannel<'a, T>,
}

impl<'a, T> Future for Receiver<'a, T> {
    type Output = Option<&'a T>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(val) = self.chan.buffer.first() {
            Poll::Ready(Some(val))
        } else {
            Poll::Pending
        }
    }
}

pub struct Executor<'a> {
    ready: Vec<Task<'a>>,
}

pub struct YieldNow<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> Future for YieldNow<'a> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }
}

pub struct JoinHandle<'a, T> {
    result: Option<T>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T> Future for JoinHandle<'a, T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Pending
    }
}

pub struct Select<'a, A, B> {
    a: A,
    b: B,
    _marker: PhantomData<&'a ()>,
}

impl<'a, A: Future + Unpin, B: Future + Unpin> Future for Select<'a, A, B> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(_) = Pin::new(&mut self.a).poll(cx) {
            Poll::Ready(())
        } else {
            Pin::new(&mut self.b).poll(cx).map(|_| ())
        }
    }
}

pub struct StreamProcessor<'a, S> {
    stream: S,
    _marker: PhantomData<&'a ()>,
}

pub trait AsyncVisitor<'a> {
    fn visit(&mut self) -> Pin<Box<dyn Future<Output = ()> + 'a>>;
}

pub struct RecursiveAsync<'a> {
    depth: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> RecursiveAsync<'a> {
    pub fn run(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async move {
            if self.depth > 0 {
            }
        })
    }
}

pub struct Y<'a>(&'a i32);
pub struct Z<'a>(&'a i32);

fn main() {
    println!("Scheduler");
}
