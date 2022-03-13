/*
Vectors -- dynamic arrays
*/

use std::mem;

pub fn run() {
    let mut v: Vec<i32> = vec![1,2,3,5,7];
    // Reassign vals
    v[2] = 200;
    // Add
    v.push(11);
    v.push(13);
    println!("{:?}", v);
    // Remove
    v.pop();
    // Get
    println!("Get idx 3 {}", v[3]);
    //Length
    println!("Length = {}", v.len());
    // Size
    println!("Byte size = {}", mem::size_of_val(&v));
    // Slice
    let slice: &[i32] = &v[..2];
    println!("Slice: {:?}", slice);
    // Loop
    for x in v.iter() {
        println!("{x}");
    }
    // Looped mutation
    for x in v.iter_mut() {
        *x *= *x;
    }
    println!("Squared vector {:?}", v);

}