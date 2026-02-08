use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::collections::{VecDeque, HashMap};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::os::unix::io::{AsRawFd, RawFd};

thread_local! {
    static REACTOR: RefCell<Option<Rc<Reactor>>> = RefCell::new(None);
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl Task {
    fn new(future: impl Future<Output = ()> + Send + 'static) -> Arc<Self> {
        Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
        })
    }

    fn poll(self: &Arc<Self>, cx: &mut Context) -> Poll<()> {
        let mut future = self.future.lock().unwrap();
        future.as_mut().poll(cx)
    }
}

struct Executor {
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
    reactor: Arc<Reactor>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            reactor: Arc::new(Reactor::new()),
        }
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Task::new(future);
        self.queue.lock().unwrap().push_back(task);
    }

    fn run(&self) {
        loop {
            let task = {
                let mut queue = self.queue.lock().unwrap();
                if queue.is_empty() {
                    break;
                }
                queue.pop_front()
            };

            if let Some(task) = task {
                let waker = self.create_waker(task.clone());
                let mut cx = Context::from_waker(&waker);
                
                match task.poll(&mut cx) {
                    Poll::Ready(()) => {},
                    Poll::Pending => {
                        self.reactor.register_task(task);
                    }
                }
            }

            self.reactor.poll_events();
        }
    }

    fn create_waker(&self, task: Arc<Task>) -> Waker {
        let queue = self.queue.clone();
        let raw_waker = {
            use std::task::RawWaker;
            use std::task::RawWakerVTable;
            
            unsafe fn clone(data: *const ()) -> RawWaker {
                Arc::increment_strong_count(data as *const Task);
                RawWaker::new(data, &VTABLE)
            }
            
            unsafe fn wake(data: *const ()) {
                let task = Arc::from_raw(data as *const Task);
                REACTOR.with(|r| {
                    if let Some(reactor) = r.borrow().as_ref() {
                        reactor.wake_task(task.clone());
                    }
                });
            }
            
            unsafe fn wake_by_ref(data: *const ()) {
                let task = Arc::from_raw(data as *const Task);
                REACTOR.with(|r| {
                    if let Some(reactor) = r.borrow().as_ref() {
                        reactor.wake_task(task.clone());
                    }
                });
                Arc::increment_strong_count(data as *const Task);
            }
            
            unsafe fn drop(data: *const ()) {
                Arc::from_raw(data as *const Task);
            }
            
            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
            
            let data = Arc::into_raw(task);
            RawWaker::new(data as *const (), &VTABLE)
        };
        
        unsafe { Waker::from_raw(raw_waker) }
    }
}

struct Reactor {
    timers: Mutex<Vec<(Instant, Arc<Task>)>>,
    io_sources: Mutex<HashMap<RawFd, Arc<Task>>>,
    ready_queue: Mutex<VecDeque<Arc<Task>>>,
}

impl Reactor {
    fn new() -> Self {
        Reactor {
            timers: Mutex::new(Vec::new()),
            io_sources: Mutex::new(HashMap::new()),
            ready_queue: Mutex::new(VecDeque::new()),
        }
    }

    fn register_task(&self, task: Arc<Task>) {
        self.ready_queue.lock().unwrap().push_back(task);
    }

    fn wake_task(&self, task: Arc<Task>) {
        self.ready_queue.lock().unwrap().push_back(task);
    }

    fn register_timer(&self, deadline: Instant, task: Arc<Task>) {
        let mut timers = self.timers.lock().unwrap();
        timers.push((deadline, task));
        timers.sort_by_key(|(d, _)| *d);
    }

    fn register_io(&self, fd: RawFd, task: Arc<Task>) {
        self.io_sources.lock().unwrap().insert(fd, task);
    }

    fn poll_events(&self) {
        let now = Instant::now();
        let mut timers = self.timers.lock().unwrap();
        
        while let Some((deadline, _)) = timers.first() {
            if *deadline <= now {
                if let Some((_, task)) = timers.remove(0) {
                    self.wake_task(task);
                }
            } else {
                break;
            }
        }
    }
}

struct AsyncTimer {
    deadline: Instant,
    registered: bool,
}

impl AsyncTimer {
    fn new(duration: Duration) -> Self {
        AsyncTimer {
            deadline: Instant::now() + duration,
            registered: false,
        }
    }
}

impl Future for AsyncTimer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();
        
        if now >= self.deadline {
            Poll::Ready(())
        } else {
            if !self.registered {
                REACTOR.with(|r| {
                    if let Some(reactor) = r.borrow().as_ref() {
                    }
                });
                self.registered = true;
            }
            Poll::Pending
        }
    }
}

struct AsyncTcpListener {
    listener: TcpListener,
}

impl AsyncTcpListener {
    fn bind(addr: SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(AsyncTcpListener { listener })
    }

    async fn accept(&self) -> io::Result<(AsyncTcpStream, SocketAddr)> {
        AcceptFuture {
            listener: &self.listener,
            registered: false,
        }.await
    }
}

struct AcceptFuture<'a> {
    listener: &'a TcpListener,
    registered: bool,
}

impl<'a> Future for AcceptFuture<'a> {
    type Output = io::Result<(AsyncTcpStream, SocketAddr)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true).ok();
                Poll::Ready(Ok((AsyncTcpStream { stream }, addr)))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if !self.registered {
                    let fd = self.listener.as_raw_fd();
                    REACTOR.with(|r| {
                        if let Some(reactor) = r.borrow().as_ref() {
                        }
                    });
                    self.registered = true;
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

struct AsyncTcpStream {
    stream: TcpStream,
}

impl AsyncTcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ReadFuture {
            stream: &mut self.stream,
            buf,
            registered: false,
        }.await
    }

    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        WriteFuture {
            stream: &mut self.stream,
            buf,
            registered: false,
        }.await
    }
}

struct ReadFuture<'a> {
    stream: &'a mut TcpStream,
    buf: &'a mut [u8],
    registered: bool,
}

impl<'a> Future for ReadFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.stream.read(self.buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if !self.registered {
                    let fd = self.stream.as_raw_fd();
                    REACTOR.with(|r| {
                        if let Some(reactor) = r.borrow().as_ref() {
                        }
                    });
                    self.registered = true;
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

struct WriteFuture<'a> {
    stream: &'a mut TcpStream,
    buf: &'a [u8],
    registered: bool,
}

impl<'a> Future for WriteFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.stream.write(self.buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if !self.registered {
                    let fd = self.stream.as_raw_fd();
                    REACTOR.with(|r| {
                        if let Some(reactor) = r.borrow().as_ref() {
                        }
                    });
                    self.registered = true;
                }
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

struct Channel<T> {
    inner: Arc<Mutex<ChannelInner<T>>>,
}

struct ChannelInner<T> {
    queue: VecDeque<T>,
    wakers: Vec<Waker>,
    closed: bool,
}

impl<T> Channel<T> {
    fn new() -> (Sender<T>, Receiver<T>) {
        let inner = Arc::new(Mutex::new(ChannelInner {
            queue: VecDeque::new(),
            wakers: Vec::new(),
            closed: false,
        }));
        
        (
            Sender { inner: inner.clone() },
            Receiver { inner }
        )
    }
}

struct Sender<T> {
    inner: Arc<Mutex<ChannelInner<T>>>,
}

impl<T> Sender<T> {
    fn send(&self, value: T) -> Result<(), T> {
        let mut inner = self.inner.lock().unwrap();
        
        if inner.closed {
            return Err(value);
        }
        
        inner.queue.push_back(value);
        
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }
        
        Ok(())
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.closed = true;
        
        for waker in inner.wakers.drain(..) {
            waker.wake();
        }
    }
}

struct Receiver<T> {
    inner: Arc<Mutex<ChannelInner<T>>>,
}

impl<T> Receiver<T> {
    async fn recv(&self) -> Option<T> {
        RecvFuture {
            inner: self.inner.clone(),
        }.await
    }
}

struct RecvFuture<T> {
    inner: Arc<Mutex<ChannelInner<T>>>,
}

impl<T> Future for RecvFuture<T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        
        if let Some(value) = inner.queue.pop_front() {
            return Poll::Ready(Some(value));
        }
        
        if inner.closed {
            return Poll::Ready(None);
        }
        
        inner.wakers.push(cx.waker().clone());
        Poll::Pending
    }
}

struct JoinHandle<T> {
    result: Arc<Mutex<Option<T>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl<T> Future for JoinHandle<T>
where
    T: Clone,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = self.result.lock().unwrap();
        
        if let Some(value) = result.as_ref() {
            Poll::Ready(value.clone())
        } else {
            *self.waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn spawn<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Clone + Send + 'static,
{
    let result = Arc::new(Mutex::new(None));
    let waker = Arc::new(Mutex::new(None));
    
    let result_clone = result.clone();
    let waker_clone = waker.clone();
    
    REACTOR.with(|r| {
        if let Some(reactor) = r.borrow().as_ref() {
        }
    });
    
    JoinHandle { result, waker }
}

struct Semaphore {
    count: Arc<Mutex<SemaphoreInner>>,
}

struct SemaphoreInner {
    permits: usize,
    wakers: VecDeque<Waker>,
}

impl Semaphore {
    fn new(permits: usize) -> Self {
        Semaphore {
            count: Arc::new(Mutex::new(SemaphoreInner {
                permits,
                wakers: VecDeque::new(),
            })),
        }
    }

    async fn acquire(&self) -> SemaphorePermit {
        AcquireFuture {
            inner: self.count.clone(),
        }.await;
        
        SemaphorePermit {
            inner: self.count.clone(),
        }
    }
}

struct AcquireFuture {
    inner: Arc<Mutex<SemaphoreInner>>,
}

impl Future for AcquireFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        
        if inner.permits > 0 {
            inner.permits -= 1;
            Poll::Ready(())
        } else {
            inner.wakers.push_back(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct SemaphorePermit {
    inner: Arc<Mutex<SemaphoreInner>>,
}

impl Drop for SemaphorePermit {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.permits += 1;
        
        if let Some(waker) = inner.wakers.pop_front() {
            waker.wake();
        }
    }
}

struct Barrier {
    state: Arc<Mutex<BarrierState>>,
    count: usize,
}

struct BarrierState {
    arrived: usize,
    generation: usize,
    wakers: Vec<Waker>,
}

impl Barrier {
    fn new(count: usize) -> Self {
        Barrier {
            state: Arc::new(Mutex::new(BarrierState {
                arrived: 0,
                generation: 0,
                wakers: Vec::new(),
            })),
            count,
        }
    }

    async fn wait(&self) -> BarrierWaitResult {
        WaitFuture {
            state: self.state.clone(),
            count: self.count,
            generation: None,
        }.await
    }
}

struct WaitFuture {
    state: Arc<Mutex<BarrierState>>,
    count: usize,
    generation: Option<usize>,
}

impl Future for WaitFuture {
    type Output = BarrierWaitResult;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        let generation = self.generation.unwrap_or_else(|| {
            let gen = state.generation;
            state.arrived += 1;
            self.generation = Some(gen);
            gen
        });
        
        if state.generation > generation {
            return Poll::Ready(BarrierWaitResult { is_leader: false });
        }
        
        if state.arrived >= self.count {
            state.arrived = 0;
            state.generation += 1;
            
            for waker in state.wakers.drain(..) {
                waker.wake();
            }
            
            Poll::Ready(BarrierWaitResult { is_leader: true })
        } else {
            state.wakers.push(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct BarrierWaitResult {
    is_leader: bool,
}

impl BarrierWaitResult {
    fn is_leader(&self) -> bool {
        self.is_leader
    }
}