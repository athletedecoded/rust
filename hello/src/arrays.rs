/*
Arrays -- static data structure of same type
Stack memory allocated
*/

use std::mem;

pub fn run() {
    let mut nums: [i32 ; 5] = [1,2,3,5,7];
    println!("{:?}", nums);
    // Reassign vals
    nums[2] = 200;
    println!("{:?}", nums);
    println!("Get idx 3 {}", nums[3]);
    println!("Length = {}", nums.len());
    // Size
    println!("Byte size = {}", mem::size_of_val(&nums));
    // Slice
    let slice: &[i32] = &nums[..2];
    println!("Slice: {:?}", slice)

}