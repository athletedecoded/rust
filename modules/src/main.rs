/*
cd maths;
cargo run
*/
// use std::{cmp::Ordering, io}; **Nested module import
use maths as cool_maths;

fn main() {
    let mut x:i32 = 5;
    cool_maths::twenty_twelve(&mut x);
    println!("{}",x);
}