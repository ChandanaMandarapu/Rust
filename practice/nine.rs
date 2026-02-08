use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

trait Nat {
    const VALUE: usize;
}

struct Zero;
struct Succ<N: Nat>(PhantomData<N>);

impl Nat for Zero {
    const VALUE: usize = 0;
}

impl<N: Nat> Nat for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;
type Four = Succ<Three>;
type Five = Succ<Four>;
type Six = Succ<Five>;
type Seven = Succ<Six>;
type Eight = Succ<Seven>;
type Nine = Succ<Eight>;
type Ten = Succ<Nine>;

trait NatAdd<N: Nat> {
    type Output: Nat;
}

impl<N: Nat> NatAdd<N> for Zero {
    type Output = N;
}

impl<N: Nat, M: Nat> NatAdd<M> for Succ<N>
where
    N: NatAdd<M>,
{
    type Output = Succ<<N as NatAdd<M>>::Output>;
}

trait NatMul<N: Nat> {
    type Output: Nat;
}

impl<N: Nat> NatMul<N> for Zero {
    type Output = Zero;
}

impl<N: Nat, M: Nat> NatMul<M> for Succ<N>
where
    N: NatMul<M>,
    M: NatAdd<<N as NatMul<M>>::Output>,
{
    type Output = <M as NatAdd<<N as NatMul<M>>::Output>>::Output;
}

trait Vec<T> {
    type Item;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct StaticVec<T, N: Nat> {
    data: [Option<T>; 32],
    len: usize,
    _phantom: PhantomData<N>,
}

impl<T, N: Nat> StaticVec<T, N> {
    fn new() -> Self {
        StaticVec {
            data: Default::default(),
            len: 0,
            _phantom: PhantomData,
        }
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N::VALUE {
            return Err(value);
        }
        self.data[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }
}

impl<T, N: Nat> Vec<T> for StaticVec<T, N> {
    type Item = T;

    fn get(&self, index: usize) -> Option<&Self::Item> {
        if index < self.len {
            self.data[index].as_ref()
        } else {
            None
        }
    }
}

trait HKT {
    type Applied<T>;
}

struct OptionHKT;
impl HKT for OptionHKT {
    type Applied<T> = Option<T>;
}

struct VecHKT;
impl HKT for VecHKT {
    type Applied<T> = std::vec::Vec<T>;
}

trait Functor: HKT {
    fn map<A, B, F>(fa: Self::Applied<A>, f: F) -> Self::Applied<B>
    where
        F: FnMut(A) -> B;
}

impl Functor for OptionHKT {
    fn map<A, B, F>(fa: Self::Applied<A>, mut f: F) -> Self::Applied<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(|a| f(a))
    }
}

impl Functor for VecHKT {
    fn map<A, B, F>(fa: Self::Applied<A>, mut f: F) -> Self::Applied<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(|a| f(a)).collect()
    }
}

trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::Applied<A>;
    fn ap<A, B, F>(ff: Self::Applied<F>, fa: Self::Applied<A>) -> Self::Applied<B>
    where
        F: FnMut(A) -> B;
}

impl Applicative for OptionHKT {
    fn pure<A>(a: A) -> Self::Applied<A> {
        Some(a)
    }

    fn ap<A, B, F>(ff: Self::Applied<F>, fa: Self::Applied<A>) -> Self::Applied<B>
    where
        F: FnMut(A) -> B,
    {
        match (ff, fa) {
            (Some(mut f), Some(a)) => Some(f(a)),
            _ => None,
        }
    }
}

trait Monad: Applicative {
    fn bind<A, B, F>(ma: Self::Applied<A>, f: F) -> Self::Applied<B>
    where
        F: FnMut(A) -> Self::Applied<B>;
}

impl Monad for OptionHKT {
    fn bind<A, B, F>(ma: Self::Applied<A>, mut f: F) -> Self::Applied<B>
    where
        F: FnMut(A) -> Self::Applied<B>,
    {
        ma.and_then(|a| f(a))
    }
}

trait TypeFamily {
    type Member<'a, T>;
}

struct LifetimeFamily;
impl TypeFamily for LifetimeFamily {
    type Member<'a, T> = &'a T;
}

struct RefCellFamily;
impl TypeFamily for RefCellFamily {
    type Member<'a, T> = std::cell::RefCell<T>;
}

trait Container: TypeFamily {
    fn create<'a, T>(value: T) -> Self::Member<'a, T>
    where
        T: 'a;
}

impl Container for RefCellFamily {
    fn create<'a, T>(value: T) -> Self::Member<'a, T>
    where
        T: 'a,
    {
        std::cell::RefCell::new(value)
    }
}

trait TypeEq<T> {
    fn cast(self) -> T;
}

impl<T> TypeEq<T> for T {
    fn cast(self) -> T {
        self
    }
}

struct Refl<A, B>(PhantomData<(A, B)>);

impl<A> Refl<A, A> {
    fn new() -> Self {
        Refl(PhantomData)
    }
}

trait TypeList {
    type Head;
    type Tail: TypeList;
}

struct Nil;
struct Cons<H, T: TypeList>(PhantomData<(H, T)>);

impl TypeList for Nil {
    type Head = ();
    type Tail = Nil;
}

impl<H, T: TypeList> TypeList for Cons<H, T> {
    type Head = H;
    type Tail = T;
}

trait Contains<T, L: TypeList> {
    fn index() -> usize;
}

impl<T, Tail: TypeList> Contains<T, Cons<T, Tail>> for () {
    fn index() -> usize {
        0
    }
}

impl<T, H, Tail: TypeList> Contains<T, Cons<H, Tail>> for Tail
where
    Tail: Contains<T, Tail>,
{
    fn index() -> usize {
        1 + <Tail as Contains<T, Tail>>::index()
    }
}

trait HList {
    fn len() -> usize;
}

impl HList for Nil {
    fn len() -> usize {
        0
    }
}

impl<H, T: HList> HList for Cons<H, T> {
    fn len() -> usize {
        1 + T::len()
    }
}

struct HCons<H, T> {
    head: H,
    tail: T,
}

struct HNil;

trait Append<L> {
    type Output;
    fn append(self, other: L) -> Self::Output;
}

impl<L> Append<L> for HNil {
    type Output = L;
    fn append(self, other: L) -> Self::Output {
        other
    }
}

impl<H, T, L> Append<L> for HCons<H, T>
where
    T: Append<L>,
{
    type Output = HCons<H, T::Output>;
    fn append(self, other: L) -> Self::Output {
        HCons {
            head: self.head,
            tail: self.tail.append(other),
        }
    }
}

