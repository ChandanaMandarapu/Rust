use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct PoolManager<'a, T, K>
where
    T: 'a,
    K: Hash + Eq + 'a,
{
    resources: HashMap<K, Vec<&'a T>>,
    borrowed: HashMap<K, usize>,
    _marker: PhantomData<&'a T>,
}

pub struct PooledResource<'a, T, K>
where
    T: 'a,
    K: Hash + Eq + 'a,
{
    item: &'a T,
    manager: &'a PoolManager<'a, T, K>,
    key: K,
}

pub trait ResourceProvider<'a, T> {
    fn provide(&'a self) -> &'a T;
    fn validate(&self, item: &'a T) -> bool;
}

pub struct SimpleProvider<'a, T> {
    storage: Vec<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> SimpleProvider<'a, T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            storage: items,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> ResourceProvider<'a, T> for SimpleProvider<'a, T> {
    fn provide(&'a self) -> &'a T {
        &self.storage[0]
    }
    
    fn validate(&self, _item: &'a T) -> bool {
        true
    }
}

impl<'a, T, K> PoolManager<'a, T, K>
where
    T: 'a,
    K: Hash + Eq + Clone + 'a,
{
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            borrowed: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn add_resource(&mut self, key: K, resource: &'a T) {
        self.resources.entry(key).or_insert_with(Vec::new).push(resource);
    }

    pub fn acquire(&'a mut self, key: K) -> Option<PooledResource<'a, T, K>> {
        if let Some(list) = self.resources.get_mut(&key) {
            if let Some(item) = list.pop() {
                let count = self.borrowed.entry(key.clone()).or_insert(0);
                *count += 1;
                return Some(PooledResource {
                    item,
                    manager: self,
                    key,
                });
            }
        }
        None
    }

    pub fn release(&mut self, mut resource: PooledResource<'a, T, K>) {
        if let Some(count) = self.borrowed.get_mut(&resource.key) {
            if *count > 0 {
                *count -= 1;
            }
        }
        self.resources
            .entry(resource.key.clone())
            .or_insert_with(Vec::new)
            .push(resource.item);
    }
    
    pub fn available_count(&self, key: &K) -> usize {
        self.resources.get(key).map(|v| v.len()).unwrap_or(0)
    }
    
    pub fn borrowed_count(&self, key: &K) -> usize {
        *self.borrowed.get(key).unwrap_or(&0)
    }
}

pub struct ComplexKey<'a> {
    pub region: &'a str,
    pub id: u32,
}

impl<'a> Hash for ComplexKey<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.region.hash(state);
        self.id.hash(state);
    }
}

impl<'a> PartialEq for ComplexKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.region == other.region && self.id == other.id
    }
}

impl<'a> Eq for ComplexKey<'a> {}

impl<'a> Clone for ComplexKey<'a> {
    fn clone(&self) -> Self {
        Self {
            region: self.region,
            id: self.id,
        }
    }
}

pub struct RegionManager<'a, 'b> 
where 'a: 'b 
{
    pool: &'b mut PoolManager<'a, String, ComplexKey<'a>>,
    active_regions: Vec<&'a str>,
}

impl<'a, 'b> RegionManager<'a, 'b> {
    pub fn new(pool: &'b mut PoolManager<'a, String, ComplexKey<'a>>, regions: Vec<&'a str>) -> Self {
        Self {
            pool,
            active_regions: regions,
        }
    }
    
    pub fn get_resource_for_region(&'b mut self, region: &'a str, id: u32) -> Option<PooledResource<'a, String, ComplexKey<'a>>> {
        if self.active_regions.contains(&region) {
            let key = ComplexKey { region, id };
            self.pool.acquire(key)
        } else {
            None
        }
    }
}

pub struct LifetimeWrapper<'a, T> {
    pub data: &'a T,
}

pub struct NestedPool<'a, 'b, T> 
where T: 'a + 'b
{
    inner: &'b PoolManager<'a, T, String>,
}

pub fn transpose_lifetime<'a, 'b, T>(input: &'a T, _context: &'b str) -> &'a T {
    input
}

pub trait LifetimeProcessor<'a> {
    type Output;
    fn process(&'a self, data: &'a str) -> Self::Output;
}

pub struct StringProcessor<'a> {
    prefix: &'a str,
}

impl<'a> LifetimeProcessor<'a> for StringProcessor<'a> {
    type Output = String;
    fn process(&'a self, data: &'a str) -> Self::Output {
        format!("{}{}", self.prefix, data)
    }
}

pub struct Buffer<'a> {
    data: &'a [u8],
}

pub struct Cursor<'a> {
    buffer: &'a Buffer<'a>,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(buffer: &'a Buffer<'a>) -> Self {
        Self { buffer, pos: 0 }
    }
    
    pub fn read_byte(&mut self) -> Option<&'a u8> {
        if self.pos < self.buffer.data.len() {
            let byte = &self.buffer.data[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }
    
    pub fn peek(&self) -> Option<&'a u8> {
        if self.pos < self.buffer.data.len() {
            Some(&self.buffer.data[self.pos])
        } else {
            None
        }
    }
    
    pub fn sub_cursor<'b>(&'b self) -> Cursor<'a> 
    where 'a: 'b 
    {
        Cursor {
            buffer: self.buffer,
            pos: self.pos,
        }
    }
}

pub struct MultiLayerCache<'a, T> {
    l1: &'a [T],
    l2: &'a [T],
    l3: &'a [T],
}

impl<'a, T> MultiLayerCache<'a, T> {
    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index < self.l1.len() {
            Some(&self.l1[index])
        } else if index < self.l1.len() + self.l2.len() {
            Some(&self.l2[index - self.l1.len()])
        } else if index < self.l1.len() + self.l2.len() + self.l3.len() {
            Some(&self.l3[index - self.l1.len() - self.l2.len()])
        } else {
            None
        }
    }
    
    pub fn iter(&self) -> CacheIter<'a, T> {
        CacheIter {
            cache: self,
            pos: 0,
        }
    }
}

pub struct CacheIter<'a, T: 'a> {
    cache: &'a MultiLayerCache<'a, T>,
    pos: usize,
}

impl<'a, T> Iterator for CacheIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.cache.get(self.pos);
        if item.is_some() {
            self.pos += 1;
        }
        item
    }
}

pub struct ReferenceHolder<'a> {
    r1: &'a str,
    r2: &'a str,
}

impl<'a> ReferenceHolder<'a> {
    pub fn longest(&self) -> &'a str {
        if self.r1.len() > self.r2.len() {
            self.r1
        } else {
            self.r2
        }
    }
    
    pub fn combine(&self) -> String {
        format!("{}{}", self.r1, self.r2)
    }
    
    pub fn switch<'b>(&'b mut self, new_ref: &'a str) 
    where 'a: 'b 
    {
        self.r1 = new_ref;
    }
}

pub fn complex_lifetime_interaction<'a, 'b, 'c>(
    x: &'a str, 
    y: &'b str, 
    z: &'c str
) -> &'a str 
where 'b: 'a, 'c: 'a 
{
    if x.len() > y.len() {
        x
    } else {
        y 
    }
}

pub struct VarianceTester<'a> {
    pub slice: &'a [u8],
}

impl<'a> VarianceTester<'a> {
    pub fn shorten<'b>(&'b self) -> &'b [u8] 
    where 'a: 'b
    {
        &self.slice[0..1]
    }
}

pub struct DoubleRef<'a, 'b, T> {
    pub x: &'a T,
    pub y: &'b T,
}

impl<'a, 'b, T> DoubleRef<'a, 'b, T> 
where T: PartialEq 
{
    pub fn are_equal(&self) -> bool {
        self.x == self.y
    }
    
    pub fn pick_first(&self) -> &'a T {
        self.x
    }
    
    pub fn pick_second_if_long_enough(&self) -> Option<&'a T> 
    where 'b: 'a 
    {
        Some(self.y)
    }
}

pub struct RecursiveRef<'a> {
    pub value: i32,
    pub next: Option<&'a RecursiveRef<'a>>,
}

impl<'a> RecursiveRef<'a> {
    pub fn depth(&self) -> usize {
        match self.next {
            Some(n) => 1 + n.depth(),
            None => 1,
        }
    }
}

pub struct ClosureHolder<'a, F> 
where F: Fn(&'a str) -> usize
{
    pub processor: F,
    pub _marker: PhantomData<&'a ()>,
}

impl<'a, F> ClosureHolder<'a, F>
where F: Fn(&'a str) -> usize
{
    pub fn execute(&self, input: &'a str) -> usize {
        (self.processor)(input)
    }
}

pub fn create_processor<'a>(cutoff: usize) -> impl Fn(&'a str) -> bool {
    move |s: &'a str| s.len() > cutoff
}

pub struct Context<'a> {
    pub data: HashMap<&'a str, i32>,
}

pub struct ContextConsumer<'a> {
    pub ctx: &'a Context<'a>,
}

impl<'a> ContextConsumer<'a> {
    pub fn get_val(&self, key: &str) -> Option<i32> {
        self.ctx.data.get(key).copied()
    }
}

pub struct MutableIter<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T> Iterator for MutableIter<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        let slice = std::mem::take(&mut self.slice);
        if slice.is_empty() {
            return None;
        }
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)
    }
}

pub struct DataContainer<T> {
    data: Vec<T>,
}

impl<T> DataContainer<T> {
    pub fn get_ref<'a>(&'a self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }
    
    pub fn get_mut_ref<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
        self.data.get_mut(index)
    }
    
    pub fn get_multi_mut<'a>(&'a mut self, idx1: usize, idx2: usize) -> Option<(&'a mut T, &'a mut T)> {
        if idx1 == idx2 { return None; }
        if idx1 >= self.data.len() || idx2 >= self.data.len() { return None; }
        
        let ptr = self.data.as_mut_ptr();
        unsafe {
            Some((&mut *ptr.add(idx1), &mut *ptr.add(idx2)))
        }
    }
}

pub trait LifetimeTransformer<'a, 'b> {
    fn transform(&self, input: &'a str) -> &'b str;
}

pub struct IdentityTransformer;

impl<'a> LifetimeTransformer<'a, 'a> for IdentityTransformer {
    fn transform(&self, input: &'a str) -> &'a str {
        input
    }
}

pub struct StaticRefHolder {
    pub text: &'static str,
}

impl StaticRefHolder {
    pub fn new() -> Self {
        Self { text: "static" }
    }
}

pub fn longest_of_three<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    if x.len() > y.len() && x.len() > z.len() {
        x
    } else if y.len() > z.len() {
        y
    } else {
        z
    }
}

pub struct KeyProvider<'a, K> {
    keys: Vec<&'a K>,
}

impl<'a, K> KeyProvider<'a, K> {
    pub fn get_key(&self, index: usize) -> Option<&'a K> {
        self.keys.get(index).copied()
    }
}

pub struct PhantomWrapper<'a, T> {
    _marker: PhantomData<&'a T>,
}

pub struct LifeTimeLogicGate<'a> {
    input_a: &'a bool,
    input_b: &'a bool,
}

impl<'a> LifeTimeLogicGate<'a> {
    pub fn and(&self) -> bool {
        *self.input_a && *self.input_b
    }
    pub fn or(&self) -> bool {
        *self.input_a || *self.input_b
    }
}

pub struct ResultContainer<'a, T, E> {
    pub ok: Option<&'a T>,
    pub err: Option<&'a E>,
}

impl<'a, T, E> ResultContainer<'a, T, E> {
    pub fn is_ok(&self) -> bool {
        self.ok.is_some()
    }
}

pub enum LifetimeEnum<'a, T> {
    Ref(&'a T),
    Owned(T),
    None,
}

impl<'a, T: Clone> LifetimeEnum<'a, T> {
    pub fn to_owned(&self) -> T {
        match self {
            LifetimeEnum::Ref(r) => (*r).clone(),
            LifetimeEnum::Owned(o) => o.clone(),
            LifetimeEnum::None => panic!("None"),
        }
    }
}

pub struct A<'a>(&'a i32);
pub struct B<'b>(&'b i32);
pub struct C<'c>(&'c i32);

pub struct ABC<'a, 'b, 'c> {
    a: A<'a>,
    b: B<'b>,
    c: C<'c>,
}

impl<'a, 'b, 'c> ABC<'a, 'b, 'c> {
    pub fn sum(&self) -> i32 {
        *self.a.0 + *self.b.0 + *self.c.0
    }
}

pub fn constrain_lifetimes<'a, 'b>(lhs: &'a i32, rhs: &'b i32) -> i32 
where 'a: 'b 
{
    *lhs + *rhs
}

fn main() {
    let x = 10;
    let pm = PoolManager::<'_, i32, ComplexKey<'_>>::new();
    println!("Pool created");
}
