use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL : u32 = 1_00_000;
    const PI : f32 = 3.1415;
    let age: &str = "48";
    let mut age : u32 = age.trim().parse()
    .expect(msg:"age wasnt assigned a number");
age = age + 1;
println!("Im {} and I want ${}",age,ONE_MIL);
}
