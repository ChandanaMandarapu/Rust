use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut v = Vec::new();
    for i in 0..200 {
        v.push(i);
    }

    let sum: i32 = v.iter().sum();
    println!("{}", sum);

    let mut map = HashMap::new();
    for i in 0..150 {
        map.insert(i, i * 2);
    }

    let mut total = 0;
    for (_, v) in map.iter() {
        total += v;
    }

    println!("{}", total);

    let mut set = HashSet::new();
    for i in 0..300 {
        set.insert(i % 20);
    }

    println!("{}", set.len());

    let mut queue = VecDeque::new();
    for i in 0..100 {
        queue.push_back(i);
    }

    while let Some(x) = queue.pop_front() {
        if x % 25 == 0 {
            println!("{}", x);
        }
    }

    let data = generate_data();
    let filtered = filter_even(data);
    let squared = square_all(filtered);
    let reduced = reduce_sum(squared);

    println!("{}", reduced);

    let users = generate_users();
    let grouped = group_by_age(users);

    for (age, list) in grouped {
        println!("{} {}", age, list.len());
    }
}

fn generate_data() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..400 {
        v.push(i);
    }
    v
}

fn filter_even(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().filter(|x| x % 2 == 0).collect()
}

fn square_all(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|x| x * x).collect()
}

fn reduce_sum(v: Vec<i32>) -> i32 {
    v.into_iter().fold(0, |a, b| a + b)
}

#[derive(Clone)]
struct User {
    id: u32,
    age: u32,
}

fn generate_users() -> Vec<User> {
    let mut users = Vec::new();
    for i in 0..300 {
        users.push(User {
            id: i,
            age: 18 + (i % 40),
        });
    }
    users
}

fn group_by_age(users: Vec<User>) -> HashMap<u32, Vec<User>> {
    let mut map = HashMap::new();
    for u in users {
        map.entry(u.age).or_insert(Vec::new()).push(u);
    }
    map
}
