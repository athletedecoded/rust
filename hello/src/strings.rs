/*
Primitive str type ==> immuate, fixed length string in memory
String ==> dynamic, head-allocated data structure used when modification/ownership is required
*/

pub fn run() {
    let greet = "Hey there";
    let mut dynamic_greet = String::from("Hello ");

    // Get length
    println!("Length {}", greet.len());

    dynamic_greet.push('B');
    dynamic_greet.push_str("uzz Lightyear");
    println!("{}", dynamic_greet);
    // Byte capacity
    println!("{}", dynamic_greet.capacity());
    // Empty string
    println!("{}", dynamic_greet.is_empty());
    // Contains
    println!("Contains `light` {}", dynamic_greet.contains("light"));
    println!("Contains `Light` {}", dynamic_greet.contains("Light"));
    // Assertion testing
    let mut s = String::with_capacity(10);
    s.push('T');
    s.push('o');
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());

}