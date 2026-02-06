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

trait Peano {
    type Pred: Peano;
    fn to_usize() -> usize;
}

impl Peano for Zero {
    type Pred = Zero;
    fn to_usize() -> usize {
        0
    }
}

impl<N: Peano> Peano for Succ<N> {
    type Pred = N;
    fn to_usize() -> usize {
        1 + N::to_usize()
    }
}

trait BoolType {
    const BOOL: bool;
}

struct True;
struct False;

impl BoolType for True {
    const BOOL: bool = true;
}

impl BoolType for False {
    const BOOL: bool = false;
}

trait If<Then, Else> {
    type Output;
}

impl<Then, Else> If<Then, Else> for True {
    type Output = Then;
}

impl<Then, Else> If<Then, Else> for False {
    type Output = Else;
}

trait LessThan<N: Nat>: Nat {
    type Output: BoolType;
}

impl<N: Nat> LessThan<N> for Zero {
    type Output = True;
}

impl LessThan<Zero> for Succ<Zero> {
    type Output = False;
}

impl<N: Nat, M: Nat> LessThan<Succ<M>> for Succ<N>
where
    N: LessThan<M>,
{
    type Output = <N as LessThan<M>>::Output;
}

struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Copy + Default, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new() -> Self {
        Matrix {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < ROWS && col < COLS {
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), ()> {
        if row < ROWS && col < COLS {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize, const COLS2: usize> 
    Mul<Matrix<T, COLS, COLS2>> for Matrix<T, ROWS, COLS>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T, ROWS, COLS2>;

    fn mul(self, rhs: Matrix<T, COLS, COLS2>) -> Self::Output {
        let mut result = Matrix::new();
        
        for i in 0..ROWS {
            for j in 0..COLS2 {
                let mut sum = T::default();
                for k in 0..COLS {
                    sum = sum + self.data[i][k] * rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        
        result
    }
}

trait Tensor<const RANK: usize> {
    type Scalar;
    fn rank() -> usize {
        RANK
    }
}

struct Tensor0<T> {
    value: T,
}

struct Tensor1<T, const DIM0: usize> {
    data: [T; DIM0],
}

struct Tensor2<T, const DIM0: usize, const DIM1: usize> {
    data: [[T; DIM1]; DIM0],
}

impl<T> Tensor<0> for Tensor0<T> {
    type Scalar = T;
}

impl<T, const DIM0: usize> Tensor<1> for Tensor1<T, DIM0> {
    type Scalar = T;
}

impl<T, const DIM0: usize, const DIM1: usize> Tensor<2> for Tensor2<T, DIM0, DIM1> {
    type Scalar = T;
}

trait Shape {
    const DIMS: &'static [usize];
}

struct Shape0;
struct Shape1<const D0: usize>;
struct Shape2<const D0: usize, const D1: usize>;
struct Shape3<const D0: usize, const D1: usize, const D2: usize>;

impl Shape for Shape0 {
    const DIMS: &'static [usize] = &[];
}

impl<const D0: usize> Shape for Shape1<D0> {
    const DIMS: &'static [usize] = &[D0];
}

impl<const D0: usize, const D1: usize> Shape for Shape2<D0, D1> {
    const DIMS: &'static [usize] = &[D0, D1];
}

impl<const D0: usize, const D1: usize, const D2: usize> Shape for Shape3<D0, D1, D2> {
    const DIMS: &'static [usize] = &[D0, D1, D2];
}

trait StateMachine {
    type State;
    type Event;
    fn transition(state: Self::State, event: Self::Event) -> Self::State;
}

struct Locked;
struct Unlocked;

enum DoorEvent {
    Lock,
    Unlock,
}

struct DoorMachine;

impl StateMachine for DoorMachine {
    type State = ();
    type Event = DoorEvent;
    
    fn transition(_state: Self::State, _event: Self::Event) -> Self::State {
        ()
    }
}

trait Protocol {
    type Request;
    type Response;
}

struct HttpProtocol;

impl Protocol for HttpProtocol {
    type Request = String;
    type Response = String;
}

trait Session<P: Protocol> {
    fn send(&mut self, request: P::Request) -> Result<(), ()>;
    fn receive(&mut self) -> Result<P::Response, ()>;
}

struct TypedSession<P: Protocol, State> {
    _protocol: PhantomData<P>,
    _state: PhantomData<State>,
}

struct Idle;
struct Waiting;
struct Ready;

impl<P: Protocol> TypedSession<P, Idle> {
    fn new() -> Self {
        TypedSession {
            _protocol: PhantomData,
            _state: PhantomData,
        }
    }

    fn send(self, _request: P::Request) -> TypedSession<P, Waiting> {
        TypedSession {
            _protocol: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<P: Protocol> TypedSession<P, Waiting> {
    fn poll(self) -> Result<TypedSession<P, Ready>, TypedSession<P, Waiting>> {
        Ok(TypedSession {
            _protocol: PhantomData,
            _state: PhantomData,
        })
    }
}

impl<P: Protocol> TypedSession<P, Ready> {
    fn receive(self) -> (P::Response, TypedSession<P, Idle>) {
        (
            unsafe { std::mem::zeroed() },
            TypedSession {
                _protocol: PhantomData,
                _state: PhantomData,
            },
        )
    }
}

trait TypeRelation<A, B> {
    type Proof;
}

struct Equal<A, B>(PhantomData<(A, B)>);

impl<A> TypeRelation<A, A> for () {
    type Proof = Equal<A, A>;
}

trait Refinement<T> {
    fn check(value: &T) -> bool;
}

struct Positive;

impl Refinement<i32> for Positive {
    fn check(value: &i32) -> bool {
        *value > 0
    }
}

struct RefinedType<T, R: Refinement<T>> {
    value: T,
    _refinement: PhantomData<R>,
}

impl<T, R: Refinement<T>> RefinedType<T, R> {
    fn new(value: T) -> Option<Self> {
        if R::check(&value) {
            Some(RefinedType {
                value,
                _refinement: PhantomData,
            })
        } else {
            None
        }
    }

    fn into_inner(self) -> T {
        self.value
    }
}

trait ConstExpr<const N: usize> {
    const VALUE: usize = N;
}

impl<const N: usize> ConstExpr<N> for () {}

struct Array<T, const N: usize>
where
    [T; N]: Sized,
{
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N>
where
    [T; N]: Sized,
{
    fn new() -> Self {
        Array {
            data: [T::default(); N],
        }
    }
}

trait Concat<const M: usize> {
    type Output;
    fn concat<T: Copy>(self, other: Array<T, M>) -> Self::Output;
}

impl<T: Copy, const N: usize, const M: usize> Concat<M> for Array<T, N>
where
    [T; N]: Sized,
    [T; M]: Sized,
    [T; N + M]: Sized,
{
    type Output = Array<T, { N + M }>;

    fn concat(self, other: Array<T, M>) -> Self::Output {
        let mut result = [self.data[0]; N + M];
        result[..N].copy_from_slice(&self.data);
        result[N..].copy_from_slice(&other.data);
        Array { data: result }
    }
}