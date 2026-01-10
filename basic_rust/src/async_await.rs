use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use futures::future::{join_all, select_all};
use tokio::time::sleep;

struct Delay {
    until: Instant,
    waker: Option<Waker>,
}

impl Future for Delay {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.until {
            Poll::Ready(())
        } else {
            self.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

async fn sleeper_ms(ms: u64) {
    Delay { until: Instant::now() + Duration::from_millis(ms), waker: None }.await
}

async fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1).await + fib(n - 2).await,
    }
}

async fn concurrent_map<F, Fut, T, U>(input: Vec<T>, f: F, limit: usize) -> Vec<U>
where
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = U> + Send,
    T: Send + 'static,
    U: Send + 'static,
{
    let f = Arc::new(f);
    let (tx, mut rx) = tokio::sync::mpsc::channel(limit);
    let handles: Vec<_> = input
        .into_iter()
        .map(|item| {
            let tx = tx.clone();
            let f = f.clone();
            tokio::spawn(async move {
                let res = f(item).await;
                let _ = tx.send(res).await;
            })
        })
        .collect();
    drop(tx);
    let mut out = Vec::new();
    while let Some(v) = rx.recv().await {
        out.push(v);
    }
    for h in handles {
        let _ = h.await;
    }
    out
}

#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..20).map(|i| tokio::spawn(async move { fib(i).await })).collect();
    let fibs: Vec<u64> = join_all(tasks).await.into_iter().map(|r| r.unwrap()).collect();
    let doubled = concurrent_map(fibs.clone(), |x| async move { x * 2 }, 4).await;
    let sum: u64 = doubled.iter().sum();
    println!("{}", sum);
}