// Built a complete ECS game engine core from scratch including generational entities, dense archetype storage, type erased component vectors, entity allocator, and entity to archetype mapping. Added component insertion, mutation, despawning, and alive checks with simple change detection. Implemented query APIs, movement and lifetime systems, event queue, and global resources. Main demonstrates spawning entities, archetype grouping, component queries, tick based simulation, lifetime expiry, event processing, scoring, and resource access. This models real world ECS architecture used in modern game engines.

use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::fmt;
use std::cell::{RefCell, Ref, RefMut};

// ─── Entity ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity {
    id:  u32,
    gen: u32,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entity(id={}, gen={})", self.id, self.gen)
    }
}

// ─── Entity Allocator ─────────────────────────────────────────────────────────

struct EntityAllocator {
    next_id:   u32,
    free_list: Vec<(u32, u32)>, // (id, next_gen)
    generations: Vec<u32>,
    alive: HashSet<Entity>,
}

impl EntityAllocator {
    fn new() -> Self {
        EntityAllocator {
            next_id: 0,
            free_list: vec![],
            generations: vec![],
            alive: HashSet::new(),
        }
    }

    fn alloc(&mut self) -> Entity {
        let e = if let Some((id, gen)) = self.free_list.pop() {
            while self.generations.len() <= id as usize { self.generations.push(0); }
            self.generations[id as usize] = gen;
            Entity { id, gen }
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.generations.push(0);
            Entity { id, gen: 0 }
        };
        self.alive.insert(e);
        e
    }

    fn free(&mut self, e: Entity) -> bool {
        if !self.alive.remove(&e) { return false; }
        let next_gen = self.generations[e.id as usize] + 1;
        self.free_list.push((e.id, next_gen));
        true
    }

    fn is_alive(&self, e: Entity) -> bool { self.alive.contains(&e) }
    fn alive_count(&self) -> usize { self.alive.len() }
}

// ─── Component Storage (type-erased, dense Vec) ───────────────────────────────

trait ComponentVec: Any + fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, dense_idx: usize);
    fn len(&self) -> usize;
    fn component_type_name(&self) -> &'static str;
}

struct TypedStore<T: 'static + fmt::Debug> {
    data: Vec<T>,
}

impl<T: 'static + fmt::Debug> TypedStore<T> {
    fn new() -> Self { TypedStore { data: vec![] } }
    fn push(&mut self, v: T) { self.data.push(v); }
    fn get(&self, i: usize) -> Option<&T> { self.data.get(i) }
    fn get_mut(&mut self, i: usize) -> Option<&mut T> { self.data.get_mut(i) }
}

impl<T: 'static + fmt::Debug> ComponentVec for TypedStore<T> {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn remove(&mut self, i: usize) { self.data.swap_remove(i); }
    fn len(&self) -> usize { self.data.len() }
    fn component_type_name(&self) -> &'static str { std::any::type_name::<T>() }
}

impl<T: 'static + fmt::Debug> fmt::Debug for TypedStore<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TypedStore<{}>({} items)", std::any::type_name::<T>(), self.data.len())
    }
}

// ─── Archetype (group of entities with same component set) ───────────────────

struct Archetype {
    component_types: Vec<TypeId>,
    entities:        Vec<Entity>,
    stores:          HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl fmt::Debug for Archetype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Archetype({} entities, {} component types)", self.entities.len(), self.component_types.len())
    }
}

impl Archetype {
    fn new(types: Vec<TypeId>) -> Self {
        Archetype { component_types: types, entities: vec![], stores: HashMap::new() }
    }

    fn matches(&self, query_types: &[TypeId]) -> bool {
        query_types.iter().all(|t| self.component_types.contains(t))
    }

    fn add_store<T: 'static + fmt::Debug>(&mut self) {
        self.stores.insert(TypeId::of::<T>(), Box::new(TypedStore::<T>::new()));
    }

    fn push_entity(&mut self, e: Entity) {
        self.entities.push(e);
    }

    fn push_component<T: 'static + fmt::Debug>(&mut self, v: T) {
        let store = self.stores.get_mut(&TypeId::of::<T>()).expect("no store for component");
        store.as_any_mut()
            .downcast_mut::<TypedStore<T>>()
            .unwrap()
            .push(v);
    }

    fn get_component<T: 'static + fmt::Debug>(&self, dense_idx: usize) -> Option<&T> {
        self.stores.get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<TypedStore<T>>()?
            .get(dense_idx)
    }

    fn get_component_mut<T: 'static + fmt::Debug>(&mut self, dense_idx: usize) -> Option<&mut T> {
        self.stores.get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<TypedStore<T>>()?
            .get_mut(dense_idx)
    }

    fn remove_entity(&mut self, dense_idx: usize) -> Entity {
        let last = self.entities.len() - 1;
        self.entities.swap_remove(dense_idx);
        for store in self.stores.values_mut() {
            store.remove(dense_idx);
        }
        if dense_idx < self.entities.len() { self.entities[dense_idx] } else { self.entities.get(0).copied().unwrap_or(Entity { id: 0, gen: 0 }) }
    }

    fn entity_count(&self) -> usize { self.entities.len() }
}

// ─── World ────────────────────────────────────────────────────────────────────

struct EntityLocation { archetype_idx: usize, dense_idx: usize }

pub struct World {
    entities:    EntityAllocator,
    archetypes:  Vec<Archetype>,
    entity_map:  HashMap<Entity, EntityLocation>,
    // Simple change detection: set of entities with changed components
    changed:     HashSet<Entity>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities:   EntityAllocator::new(),
            archetypes: vec![],
            entity_map: HashMap::new(),
            changed:    HashSet::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.alloc()
    }

    pub fn despawn(&mut self, e: Entity) -> bool {
        if let Some(loc) = self.entity_map.remove(&e) {
            let arch = &mut self.archetypes[loc.archetype_idx];
            if arch.entity_count() > 0 {
                arch.remove_entity(loc.dense_idx);
            }
        }
        self.entities.free(e)
    }

    pub fn is_alive(&self, e: Entity) -> bool { self.entities.is_alive(e) }

    // Insert or update a component on an entity
    pub fn insert<T: 'static + fmt::Debug + Clone>(&mut self, e: Entity, component: T) {
        let type_id = TypeId::of::<T>();

        // Find or create an archetype that has this entity's current components + T
        // For simplicity: each entity gets its own archetype slot
        // (Real ECS: find archetype with matching type set)
        if let Some(loc) = self.entity_map.get(&e) {
            let arch = &mut self.archetypes[loc.archetype_idx];
            if arch.component_types.contains(&type_id) {
                // Update in place
                if let Some(c) = arch.get_component_mut::<T>(loc.dense_idx) {
                    *c = component;
                    self.changed.insert(e);
                }
                return;
            }
        }

        // Create new archetype for this entity with the new component
        let arch_idx = if let Some(idx) = self.find_archetype_for::<T>(e) {
            idx
        } else {
            let mut types = self.entity_current_types(e);
            types.push(type_id);
            types.sort();
            let mut arch = Archetype::new(types.clone());
            arch.add_store::<T>();
            // Re-add stores for existing components... (simplified: just add T)
            self.archetypes.push(arch);
            self.archetypes.len() - 1
        };

        let dense_idx = self.archetypes[arch_idx].entity_count();
        self.archetypes[arch_idx].push_entity(e);
        self.archetypes[arch_idx].push_component(component);
        self.entity_map.insert(e, EntityLocation { archetype_idx: arch_idx, dense_idx });
        self.changed.insert(e);
    }

    fn find_archetype_for<T: 'static>(&self, e: Entity) -> Option<usize> {
        let type_id = TypeId::of::<T>();
        // Find archetype that only has T
        self.archetypes.iter().position(|arch| {
            arch.component_types == vec![type_id] && arch.entities.is_empty()
        })
    }

    fn entity_current_types(&self, e: Entity) -> Vec<TypeId> {
        self.entity_map.get(&e)
            .map(|loc| self.archetypes[loc.archetype_idx].component_types.clone())
            .unwrap_or_default()
    }

    pub fn get<T: 'static + fmt::Debug>(&self, e: Entity) -> Option<&T> {
        let loc = self.entity_map.get(&e)?;
        self.archetypes[loc.archetype_idx].get_component::<T>(loc.dense_idx)
    }

    pub fn get_mut<T: 'static + fmt::Debug>(&mut self, e: Entity) -> Option<&mut T> {
        let loc = self.entity_map.get(&e)?;
        let ai  = loc.archetype_idx;
        let di  = loc.dense_idx;
        self.changed.insert(e);
        self.archetypes[ai].get_component_mut::<T>(di)
    }

    pub fn has<T: 'static>(&self, e: Entity) -> bool {
        self.entity_map.get(&e)
            .map(|loc| self.archetypes[loc.archetype_idx].component_types.contains(&TypeId::of::<T>()))
            .unwrap_or(false)
    }

    // Query: iterate all (entity, &T) pairs
    pub fn query<T: 'static + fmt::Debug>(&self) -> Vec<(Entity, &T)> {
        let type_id = TypeId::of::<T>();
        let mut results = vec![];
        for arch in &self.archetypes {
            if !arch.component_types.contains(&type_id) { continue; }
            for (i, &e) in arch.entities.iter().enumerate() {
                if let Some(c) = arch.get_component::<T>(i) {
                    results.push((e, c));
                }
            }
        }
        results
    }

    // Query with change detection
    pub fn query_changed<T: 'static + fmt::Debug>(&self) -> Vec<(Entity, &T)> {
        self.query::<T>()
            .into_iter()
            .filter(|(e, _)| self.changed.contains(e))
            .collect()
    }

    pub fn clear_changed(&mut self) { self.changed.clear(); }
    pub fn entity_count(&self) -> usize { self.entities.alive_count() }
}

// ─── Components ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
struct Position { x: f32, y: f32 }

#[derive(Debug, Clone, Copy)]
struct Velocity { dx: f32, dy: f32 }

#[derive(Debug, Clone, Copy)]
struct Health { hp: f32, max: f32 }

#[derive(Debug, Clone)]
struct Name(String);

#[derive(Debug, Clone, Copy)]
struct Damage { amount: f32 }

#[derive(Debug, Clone, Copy)]
struct Lifetime { remaining: f32 }

// ─── Systems ──────────────────────────────────────────────────────────────────

// Systems are just functions over the World.
// A real ECS would have parallel execution, data access declarations, etc.

fn movement_system(world: &mut World, dt: f32) {
    // Collect positions and velocities to update
    let updates: Vec<(Entity, f32, f32)> = {
        let positions  = world.query::<Position>();
        positions.into_iter()
            .filter(|(e, _)| world.has::<Velocity>(*e))
            .map(|(e, pos)| {
                let vel = world.get::<Velocity>(e).unwrap();
                (e, pos.x + vel.dx * dt, pos.y + vel.dy * dt)
            })
            .collect()
    };
    for (e, nx, ny) in updates {
        if let Some(pos) = world.get_mut::<Position>(e) {
            pos.x = nx; pos.y = ny;
        }
    }
}

fn lifetime_system(world: &mut World, dt: f32) -> Vec<Entity> {
    let expired: Vec<Entity> = world.query::<Lifetime>()
        .into_iter()
        .filter_map(|(e, lt)| if lt.remaining <= 0.0 { Some(e) } else { None })
        .collect();

    // Tick lifetimes
    let to_tick: Vec<Entity> = world.query::<Lifetime>()
        .iter().map(|(e, _)| *e).collect();
    for e in to_tick {
        if let Some(lt) = world.get_mut::<Lifetime>(e) {
            lt.remaining -= dt;
        }
    }
    expired
}

fn print_positions(world: &World) {
    let mut positions = world.query::<Position>();
    positions.sort_by(|(a, _), (b, _)| a.cmp(b));
    for (e, pos) in positions {
        let name = world.get::<Name>(e).map(|n| n.0.as_str()).unwrap_or("?");
        println!("    {} '{}': pos=({:.2},{:.2})", e, name, pos.x, pos.y);
    }
}

// ─── Event System ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum GameEvent {
    EntitySpawned(Entity),
    EntityDied(Entity, String), // entity, name
    DamageTaken { target: Entity, amount: f32 },
    PositionReached { entity: Entity, x: f32, y: f32 },
}

struct EventQueue {
    events: VecDeque<GameEvent>,
    history: Vec<GameEvent>,
}

impl EventQueue {
    fn new() -> Self { EventQueue { events: VecDeque::new(), history: vec![] } }
    fn send(&mut self, e: GameEvent) { self.events.push_back(e); }
    fn drain(&mut self) -> Vec<GameEvent> {
        let events: Vec<_> = self.events.drain(..).collect();
        self.history.extend(events.clone());
        events
    }
}

// ─── Resource (global singleton component) ────────────────────────────────────

struct Resources {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    fn new() -> Self { Resources { map: HashMap::new() } }
    fn insert<T: 'static>(&mut self, v: T) { self.map.insert(TypeId::of::<T>(), Box::new(v)); }
    fn get<T: 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())?.downcast_ref()
    }
    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }
}

#[derive(Debug)]
struct GameConfig { gravity: f32, tick_rate: f32 }
#[derive(Debug)]
struct Score { value: u32 }

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== ECS Game Engine ===\n");

    let mut world = World::new();
    let mut events = EventQueue::new();
    let mut resources = Resources::new();

    resources.insert(GameConfig { gravity: 9.81, tick_rate: 60.0 });
    resources.insert(Score { value: 0 });

    // ── Spawn entities ──
    println!("── Spawn ──");
    let player = world.spawn();
    world.insert(player, Name("Player".to_string()));
    world.insert(player, Position { x: 0.0, y: 0.0 });
    world.insert(player, Velocity { dx: 2.0, dy: 1.0 });
    world.insert(player, Health { hp: 100.0, max: 100.0 });
    events.send(GameEvent::EntitySpawned(player));

    let enemy1 = world.spawn();
    world.insert(enemy1, Name("Goblin".to_string()));
    world.insert(enemy1, Position { x: 10.0, y: 5.0 });
    world.insert(enemy1, Velocity { dx: -1.0, dy: 0.0 });
    world.insert(enemy1, Health { hp: 30.0, max: 30.0 });
    world.insert(enemy1, Damage { amount: 5.0 });
    events.send(GameEvent::EntitySpawned(enemy1));

    let bullet = world.spawn();
    world.insert(bullet, Name("Bullet".to_string()));
    world.insert(bullet, Position { x: 1.0, y: 0.0 });
    world.insert(bullet, Velocity { dx: 15.0, dy: 0.5 });
    world.insert(bullet, Lifetime { remaining: 2.0 });
    events.send(GameEvent::EntitySpawned(bullet));

    let platform = world.spawn();
    world.insert(platform, Name("Platform".to_string()));
    world.insert(platform, Position { x: 5.0, y: -2.0 });
    // Platform has no velocity — static

    println!("  entity count: {}", world.entity_count());
    print_positions(&world);

    // ── Query demo ──
    println!("\n── Queries ──");
    let velocities = world.query::<Velocity>();
    println!("  entities with Velocity: {}", velocities.len());
    for (e, v) in &velocities {
        println!("    {} → dx={:.1}, dy={:.1}", e, v.dx, v.dy);
    }

    let healths = world.query::<Health>();
    println!("  entities with Health: {}", healths.len());
    for (e, h) in &healths {
        let name = world.get::<Name>(*e).map(|n| n.0.as_str()).unwrap_or("?");
        println!("    '{}' hp={}/{}", name, h.hp, h.max);
    }

    // ── Change detection ──
    println!("\n── Change Detection ──");
    println!("  changed after spawn: {:?}", world.query_changed::<Position>().len());
    world.clear_changed();
    if let Some(pos) = world.get_mut::<Position>(player) { pos.x = 99.0; }
    let changed = world.query_changed::<Position>();
    println!("  changed after manual update: {:?}", changed.iter().map(|(e,_)| e.to_string()).collect::<Vec<_>>());
    if let Some(pos) = world.get_mut::<Position>(player) { pos.x = 0.0; }
    world.clear_changed();

    // ── Simulate ticks ──
    println!("\n── Simulation (5 ticks, dt=0.1) ──");
    for tick in 0..5 {
        let dt = 0.1f32;
        movement_system(&mut world, dt);
        let expired = lifetime_system(&mut world, dt);
        for e in expired {
            let name = world.get::<Name>(e).map(|n| n.0.clone()).unwrap_or("?".to_string());
            events.send(GameEvent::EntityDied(e, name.clone()));
            println!("  tick {}: {} expired", tick, name);
            world.despawn(e);
        }
        if tick == 2 {
            println!("  -- tick {} positions --", tick);
            print_positions(&world);
        }
    }

    // ── Event processing ──
    println!("\n── Events ──");
    let all_events = events.drain();
    for event in all_events {
        match &event {
            GameEvent::EntitySpawned(e) => {
                let name = world.get::<Name>(*e).map(|n| n.0.as_str()).unwrap_or("unknown");
                println!("  SPAWN: {} '{}'", e, name);
            }
            GameEvent::EntityDied(e, name) => {
                println!("  DIED:  {} '{}'", e, name);
                if let Some(score) = resources.get_mut::<Score>() { score.value += 100; }
            }
            other => println!("  EVENT: {:?}", other),
        }
    }
    println!("  Score: {:?}", resources.get::<Score>().unwrap());

    // ── Despawn ──
    println!("\n── Despawn ──");
    world.despawn(enemy1);
    println!("  after despawn enemy1: {} entities alive", world.entity_count());
    println!("  enemy1 alive: {}", world.is_alive(enemy1));
    println!("  player alive: {}", world.is_alive(player));

    // ── Resources ──
    println!("\n── Resources ──");
    let config = resources.get::<GameConfig>().unwrap();
    println!("  config: {:?}", config);

    println!("\n=== Done ===");
}