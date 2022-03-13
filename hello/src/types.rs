/* Rust Primitives
1. Scalar Types
* signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
* unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
* floating point: f32, f64
* char: single characters/unicodes (4 bytes each)
* bool: true/false
* unit type: empty tuple ()

2. Compound Types
* arrays (uniform type, static): [1,...,n]
* tuples (mixed type, static): (2,"blue",true..)

NB: Rust provides type safety via static typing. Variable bindings 
can be type annotated when declared. However, in most cases, the compiler 
will be able to infer the type of the variable from the context, heavily 
reducing the annotation burden.
*/

use std::convert::From;
use std::convert::Into;
use std::any::type_name;
use std::fmt;

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn run() {
    // Implicit type
    let x = 1;
    let y = 3.14;

    // Explicit type
    let z: i64 = 78655599746;

    // Determine max value of type
    println!("Max i32 = {}", std::i32::MAX);
    println!("Max i64 = {}", std::i64::MAX);

    // Explicit type casting
    let int = y as u8;
    let c = int as char;
    println!("{:?}", (x,y,z, int, c));

    // Type conversion -- From, Into
    let my_str = "hello";
    type_of(&my_str);
    let my_string = String::from(my_str);
    type_of(&my_string);

    // Parsing from String to int
    let four = "4".parse::<i32>();
    println!("Turbo Four: {:?}", four);
    println!("Four: {:?}", four.unwrap());

}