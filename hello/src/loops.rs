/*
`loop` defines an infinite loop with `break` and `continue` control flow
*/

#![allow(unreachable_code)]
pub fn run() {
    let mut count = 0;
    //  Infinite loop
    let m = loop {
        count += 1;
        if count == 42 {
            break count; //Return count
        }
    };
    println!("Found the meanining of life {}", m);

    // Nested loops
    println!("Nested Loop..");
    'loopy: loop {
        println!("In the loopy loop");
        
        'mini: loop {
            println!("In the mini loop");
            // break; // This breaks into the loopy loop
            break 'loopy; // This breaks the loopy loop 
        }
        println!("Broke out into the loopy loop");
        break; // Breaks the loopy loop
    }

    // While loop
    println!("While Loop..");
    let mut i = 1;
    while i < 10 {
        if i % 3 == 0 {
            println!("{} is divisible by 3",i);
        }
        i+=1;
    }
    // For loop
    println!("For Loop..");
    for n in 1..12 {
        if n % 4 == 0 {
            println!("{} is divisible by 4",n);
        }
    }
    // Rust also has endpoint inclusive range
    println!("Include the endpoint..");
    for n in 1..=12{
        if n % 4 == 0 {
            println!("{} is divisible by 4",n);
        }
    }
}