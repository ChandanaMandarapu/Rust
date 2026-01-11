use std::collections::HashMap;
use std::marker::PhantomData;

pub trait EventHandler<'a, E> {
    fn handle(&mut self, event: &'a E);
}

pub struct ClosureHandler<'a, F, E> 
where F: FnMut(&'a E)
{
    callback: F,
    _marker: PhantomData<&'a E>,
}

impl<'a, F, E> EventHandler<'a, E> for ClosureHandler<'a, F, E> 
where F: FnMut(&'a E)
{
    fn handle(&mut self, event: &'a E) {
        (self.callback)(event);
    }
}

pub struct EventBus<'a, E> {
    handlers: Vec<Box<dyn EventHandler<'a, E> + 'a>>, 
}

impl<'a, E> EventBus<'a, E> {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn subscribe<H>(&mut self, handler: H) 
    where H: EventHandler<'a, E> + 'a
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn subscribe_fn<F>(&mut self, f: F) 
    where F: FnMut(&'a E) + 'a 
    {
        self.handlers.push(Box::new(ClosureHandler {
            callback: f,
            _marker: PhantomData,
        }));
    }

    pub fn publish(&mut self, event: &'a E) {
        for handler in &mut self.handlers {
            handler.handle(event);
        }
    }
}

pub struct TypedEvent<'a> {
    pub name: &'a str,
    pub payload: &'a [u8],
}

pub struct Subscriber<'a> {
    pub name: &'a str,
    pub received_count: usize,
}

impl<'a> EventHandler<'a, TypedEvent<'a>> for Subscriber<'a> {
    fn handle(&mut self, event: &'a TypedEvent<'a>) {
        if event.name == self.name {
            self.received_count += 1;
        }
    }
}

pub struct SystemContext<'a> {
    pub config: &'a HashMap<String, String>,
}

pub struct SystemEventHandler<'a, 'b> {
    context: &'b SystemContext<'a>,
}

impl<'a, 'b> EventHandler<'b, TypedEvent<'b>> for SystemEventHandler<'a, 'b> {
    fn handle(&mut self, _event: &'b TypedEvent<'b>) {
    }
}

pub struct FilteredBus<'a, E, P> {
    parent: &'a mut EventBus<'a, E>,
    predicate: P,
}

impl<'a, E, P> FilteredBus<'a, E, P> 
where P: Fn(&E) -> bool
{
    pub fn publish_if(&mut self, event: &'a E) {
        if (self.predicate)(event) {
            self.parent.publish(event);
        }
    }
}

pub struct EventQueue<'a, E> {
    events: Vec<&'a E>,
}

impl<'a, E> EventQueue<'a, E> {
    pub fn push(&mut self, event: &'a E) {
        self.events.push(event);
    }
    
    pub fn drain<H>(&mut self, handler: &mut H) 
    where H: EventHandler<'a, E>
    {
        for e in self.events.drain(..) {
            handler.handle(e);
        }
    }
}

pub struct DeferredEvent<'a, E> {
    event: &'a E,
    delay: u32,
}

pub struct Scheduler<'a, E> {
    bus: &'a mut EventBus<'a, E>,
    deferred: Vec<DeferredEvent<'a, E>>,
}

impl<'a, E> Scheduler<'a, E> {
    pub fn schedule(&mut self, event: &'a E, delay: u32) {
        self.deferred.push(DeferredEvent { event, delay });
    }
    
    pub fn tick(&mut self) {
        let mut ready = Vec::new();
        let mut i = 0;
        while i < self.deferred.len() {
             if self.deferred[i].delay == 0 {
                 ready.push(self.deferred.remove(i));
             } else {
                 self.deferred[i].delay -= 1;
                 i += 1;
             }
        }
        
        for e in ready {
            self.bus.publish(e.event);
        }
    }
}

pub trait Message<'a> {
    fn content(&self) -> &'a str;
}

pub struct SimpleMessage<'a>(&'a str);
impl<'a> Message<'a> for SimpleMessage<'a> {
    fn content(&self) -> &'a str { self.0 }
}

pub struct MessageBroadcaster<'a, M: Message<'a> + 'a> {
    receivers: Vec<&'a mut dyn FnMut(&M)>,
    _marker: PhantomData<&'a M>,
}

impl<'a, M: Message<'a> + 'a> MessageBroadcaster<'a, M> {
    pub fn broadcast(&mut self, msg: &M) {
        for r in &mut self.receivers {
            r(msg);
        }
    }
}

pub struct Observer<'a, T> {
    state: &'a T,
}

pub struct Subject<'a, T> {
    observers: Vec<Observer<'a, T>>,
    value: T,
}

pub struct ExternalSubject<'a, T> {
    observers: Vec<&'a mut dyn FnMut(&'a T)>,
    value: &'a T,
}

impl<'a, T> ExternalSubject<'a, T> {
    pub fn notify(&mut self) {
        for obs in &mut self.observers {
            obs(self.value);
        }
    }
}

pub struct ListenerHandle<'a> {
    id: usize,
    _marker: PhantomData<&'a ()>,
}

pub struct Registration<'a> {
    bus_name: &'a str,
}

pub struct MultiBus<'a, E> {
    channels: HashMap<&'a str, EventBus<'a, E>>,
}

impl<'a, E> MultiBus<'a, E> {
    pub fn publish_to(&mut self, channel: &str, event: &'a E) {
        if let Some(bus) = self.channels.get_mut(channel) {
            bus.publish(event);
        }
    }
}

pub struct WildcardHandler<'a> {
    pattern: &'a str,
}

impl<'a> EventHandler<'a, TypedEvent<'a>> for WildcardHandler<'a> {
    fn handle(&mut self, event: &'a TypedEvent<'a>) {
        if event.name.contains(self.pattern) {
        }
    }
}

pub enum BusError<'a> {
    ChannelNotFound(&'a str),
    HandlerFailed,
}

pub fn route_event<'a, E>(
    source: &'a EventBus<'a, E>, 
    target: &'a mut EventBus<'a, E>, 
    transformer: impl Fn(&'a E) -> &'a E
) {
}

pub struct A1<'a>(&'a i32);
pub struct A2<'a>(&'a i32);
pub struct A3<'a>(&'a i32);
pub struct A4<'a>(&'a i32);

pub struct TupleContainer<'a> {
    t: (A1<'a>, A2<'a>, A3<'a>, A4<'a>),
}

fn main() {
    let mut bus = EventBus::<i32>::new();
    let x = 10;
    bus.publish(&x);
}
