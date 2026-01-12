use std::marker::PhantomData;

pub struct Vector2 { x: f32, y: f32 }

pub struct Body {
    pos: Vector2,
    vel: Vector2,
    mass: f32,
}

pub struct Collider<'a> {
    body: &'a Body,
    shape: Shape<'a>,
}

pub enum Shape<'a> {
    Circle { radius: f32 },
    Box { w: f32, h: f32 },
    Polygon { vertices: &'a [Vector2] },
}

pub struct World<'a> {
    bodies: Vec<Body>,
    colliders: Vec<Collider<'a>>,
}

pub struct Contact<'a> {
    a: &'a Body,
    b: &'a Body,
    normal: Vector2,
}

pub trait NarrowPhase<'a> {
    fn collide(&self, c1: &Collider<'a>, c2: &Collider<'a>) -> Option<Contact<'a>>;
}

pub struct BroadPhase<'a> {
    colliders: &'a [Collider<'a>],
}

impl<'a> BroadPhase<'a> {
    pub fn potential_pairs(&self) -> Vec<(&'a Collider<'a>, &'a Collider<'a>)> {
        let mut pairs = Vec::new();
        for (i, c1) in self.colliders.iter().enumerate() {
            for c2 in &self.colliders[i+1..] {
                pairs.push((c1, c2));
            }
        }
        pairs
    }
}

pub struct PhysicsSystem<'a> {
    world: &'a mut World<'a>,
}

impl<'a> PhysicsSystem<'a> {
    pub fn step(&mut self, _dt: f32) {
    }
}

pub struct Manifold<'a> {
    contacts: Vec<Contact<'a>>,
}

pub struct Island<'a> {
    bodies: Vec<&'a Body>,
}

pub struct Constraint<'a> {
    a: &'a Body,
    b: &'a Body,
}

pub struct Solver<'a> {
    constraints: Vec<Constraint<'a>>,
}

impl<'a> Solver<'a> {
    pub fn solve(&self) {
    }
}

pub struct RayCastResult<'a> {
    body: &'a Body,
    point: Vector2,
}

pub struct Ray<'a> {
    origin: &'a Vector2,
    dir: Vector2,
}

pub struct SpatialHash<'a> {
    cell_size: f32,
    grid: std::collections::HashMap<(i32, i32), Vec<&'a Body>>,
}

impl<'a> SpatialHash<'a> {
    pub fn insert(&mut self, body: &'a Body) {
    }
}

pub struct Material<'a> {
    friction: f32,
    restitution: f32,
    name: &'a str,
}

pub struct BodyDef<'a> {
    material: &'a Material<'a>,
}

pub struct Joint<'a> {
    body_a: &'a Body,
    body_b: &'a Body,
}

pub struct SoftBody<'a> {
    nodes: Vec<&'a Body>,
    springs: Vec<(&'a Body, &'a Body)>,
}

pub struct FluidParticle<'a> {
    pos: &'a mut Vector2,
}

pub struct FluidSystem<'a> {
    particles: Vec<FluidParticle<'a>>,
}

pub struct ForceGenerator<'a> {
    target: &'a mut Body,
}

pub struct Gravity<'a> {
    gen: ForceGenerator<'a>,
}

pub struct RegionQuery<'a> {
    bounds: (Vector2, Vector2),
    _marker: PhantomData<&'a ()>,
}

pub struct DebugRenderer<'a> {
    bodies: &'a [Body],
}

impl<'a> DebugRenderer<'a> {
    pub fn draw(&self) {
    }
}

pub struct CollisionListener<'a, F> 
where F: Fn(Contact<'a>)
{
    callback: F,
    _marker: PhantomData<&'a ()>,
}

fn main() {
    println!("Physics");
}
