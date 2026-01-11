use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Entity(u32);

pub trait Component: 'static {}
impl Component for Position {}
impl Component for Velocity {}

pub struct Position { x: f32, y: f32 }
pub struct Velocity { dx: f32, dy: f32 }

pub struct World {
    positions: Vec<Option<Position>>,
    velocities: Vec<Option<Velocity>>,
    entities: Vec<Entity>,
}

pub struct SystemContext<'a> {
    world: &'a World,
}

pub struct QueryPosition<'a> {
    iter: std::slice::Iter<'a, Option<Position>>,
}

impl<'a> Iterator for QueryPosition<'a> {
    type Item = &'a Position;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Some(pos)) => return Some(pos),
                Some(None) => continue,
                None => return None,
            }
        }
    }
}

pub struct QueryVelocity<'a> {
    iter: std::slice::Iter<'a, Option<Velocity>>,
}

impl<'a> Iterator for QueryVelocity<'a> {
    type Item = &'a Velocity;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Some(vel)) => return Some(vel),
                Some(None) => continue,
                None => return None,
            }
        }
    }
}


pub struct JoinQuery<'a> {
    pos_iter: std::slice::Iter<'a, Option<Position>>,
    vel_iter: std::slice::Iter<'a, Option<Velocity>>,
}

impl<'a> Iterator for JoinQuery<'a> {
    type Item = (&'a Position, &'a Velocity);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.pos_iter.next()?;
            let v = self.vel_iter.next()?;
            match (p, v) {
                (Some(pos), Some(vel)) => return Some((pos, vel)),
                _ => continue,
            }
        }
    }
}

pub struct System<'a> {
    name: &'a str,
    run: Box<dyn Fn(&'a World) + 'a>, 
}

impl<'a> System<'a> {
    pub fn new<F>(name: &'a str, f: F) -> Self 
    where F: Fn(&'a World) + 'a 
    {
        Self { name, run: Box::new(f) }
    }
    
    pub fn execute(&self, world: &'a World) {
        (self.run)(world);
    }
}

pub struct MutableWorld<'a> {
    positions: &'a mut [Option<Position>],
    velocities: &'a mut [Option<Velocity>],
}

impl<'a> MutableWorld<'a> {
    pub fn get_mut_pos(&mut self, entity: usize) -> Option<&mut Position> {
        if entity < self.positions.len() {
            self.positions[entity].as_mut()
        } else {
            None
        }
    }
}

pub struct ResourceManager {
    textures: HashMap<String, u32>,
}

pub struct Res<'a, T> {
    value: &'a T,
}

pub struct ResMut<'a, T> {
    value: &'a mut T,
}

pub trait SystemParam<'a> {
    type Item;
    fn fetch(world: &'a World) -> Self::Item;
}

pub struct Query<'a, Q> {
    marker: PhantomData<&'a Q>,
}

pub struct RefTuple<'a, T, U> {
    t: &'a T,
    u: &'a U,
}

pub struct ParamSet<'a, P: SystemParam<'a>> {
    p: PhantomData<&'a P>,
}

pub struct EntityRef<'a> {
    world: &'a World,
    id: Entity,
}

impl<'a> EntityRef<'a> {
    pub fn get_pos(&self) -> Option<&'a Position> {
        self.world.positions.get(self.id.0 as usize).and_then(|o| o.as_ref())
    }
}

pub struct Storage<'a, T> {
    data: &'a [T],
}

pub struct SparseSet<'a, T> {
    dense: Vec<T>,
    sparse: Vec<usize>,
    _marker: PhantomData<&'a ()>, 
}

pub struct Scheduler<'a> {
    systems: Vec<System<'a>>,
}

impl<'a> Scheduler<'a> {
    pub fn run(&self, world: &'a World) {
        for sys in &self.systems {
            sys.execute(world);
        }
    }
}

pub struct CommandBuffer<'a> {
    commands: Vec<Box<dyn FnOnce(&mut World) + 'a>>,
}

impl<'a> CommandBuffer<'a> {
    pub fn push<F>(&mut self, cmd: F) 
    where F: FnOnce(&mut World) + 'a 
    {
        self.commands.push(Box::new(cmd));
    }
}

pub struct Archetype<'a> {
    types: Vec<&'a str>, 
    entities: Vec<u32>,
}

pub struct ArchetypeIter<'a> {
    archetypes: &'a [Archetype<'a>],
}

pub struct ComponentHandle<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ComponentHandle<'a, T> {
    pub fn get(&self) -> &'a T {
        unsafe { &*self.ptr }
    }
}

pub struct LockedStorage<'a, T> {
    guard: std::sync::MutexGuard<'a, Vec<T>>,
}

pub struct ReadStorage<'a, T> {
    data: &'a [T],
}

pub struct WriteStorage<'a, T> {
    data: &'a mut [T],
}

pub struct JoinMut<'a, T, U> {
    t: &'a mut [T],
    u: &'a mut [U],
}

pub struct ZipIterMut<'a, T, U> {
    iter_t: std::slice::IterMut<'a, T>,
    iter_u: std::slice::IterMut<'a, U>,
}

impl<'a, T, U> Iterator for ZipIterMut<'a, T, U> {
    type Item = (&'a mut T, &'a mut U);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter_t.next(), self.iter_u.next()) {
            (Some(t), Some(u)) => Some((t, u)),
            _ => None,
        }
    }
}

pub struct SystemData<'a> {
    query: Box<dyn Iterator<Item = &'a Position> + 'a>,
}

pub fn run_system<'a>(data: SystemData<'a>) {
    for _item in data.query {
    }
}

pub struct LateInit<'a, T> {
    val: Option<&'a T>,
}

impl<'a, T> LateInit<'a, T> {
    pub fn init(&mut self, val: &'a T) {
        self.val = Some(val);
    }
    
    pub fn get(&self) -> &'a T {
        self.val.unwrap()
    }
}

fn main() {
    println!("ECS");
}
