/*
Shadowing
* Allows us to change a variable value/type while maintaining its immutability
*/

pub fn run() {
    let x = 5;
    println!("{}",x);
    let x = x + 1;
    println!("{}",x);
    // Shadowing within a scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}