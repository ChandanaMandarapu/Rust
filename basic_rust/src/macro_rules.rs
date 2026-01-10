macro_rules! count_tt {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count_tt!($($tail)*) };
}

macro_rules! tuple_len {
    ($name:ident) => { 1 };
    ($head:ident $($tail:ident)*) => { 1 + tuple_len!($($($tail)*)*) };
}

macro_rules! gen_struct {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        struct $name { $($field : $ty),* }
        impl $name {
            fn new($($field : $ty),*) -> Self { Self { $($field),* } }
        }
    };
}

macro_rules! chain {
    ($first:expr $(, $method:ident($($arg:expr),*))* $(,)?) => {{
        let mut temp = $first;
        $(temp = temp.$method($($arg),*);)*
        temp
    }};
}

macro_rules! vec_deque {
    (@single $x:expr) => (std::collections::VecDeque::from([$x]));
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vec_deque!(@single $rest)),*]));
    ($($x:expr),* $(,)?) => {{
        let mut dq = std::collections::VecDeque::new();
        $(dq.push_back($x);)*
        dq
    }};
}

macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:tt)*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut hm = std::collections::HashMap::new();
        $(hm.insert($k, $v);)*
        hm
    }};
}

macro_rules! repeat {
    ($e:expr; $n:expr) => {{
        let mut v = Vec::new();
        for _ in 0..$n { v.push($e); }
        v
    }};
}

macro_rules! zip {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        a.into_iter().zip(b.into_iter()).collect::<Vec<_>>()
    }};
}

macro_rules! unwrap_or_return {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return,
        }
    };
}

gen_struct!(Person { name: String, age: u32 });

fn main() {
    let dq = vec_deque![1, 2, 3, 4, 5];
    let hm = hashmap!{"a" => 1, "b" => 2};
    let v = repeat!(42; 100);
    let z = zip!(vec![1, 2, 3], vec![4, 5, 6]);
    let p = Person::new("rust".into(), 18);
    let x = chain!(10, pow(2), checked_add(5).unwrap(), wrapping_mul(3));
    println!("{} {} {} {} {}", dq.len(), hm.len(), v.len(), z.len(), x);
}