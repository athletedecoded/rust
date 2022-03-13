/*
Requiring type annotations in function definitions means the compiler almost 
never needs you to use them elsewhere in the code to figure out what type you mean
*/
pub fn run() {
    greeting("Hi", "Bart");

    let my_sum = add(23,4);
    println!("Sum {}", my_sum);

    // Closure
    let z = 10;
    let add_nums = |x:i32, y:i32| x + y + z;
    println!("Closure sum = {}", add_nums(21,19));
}

fn greeting(g: &str, name: &str) {
    println!("{} {}, nice to meet you!", g, name);
}

// Returning vals from fxns
fn add(x: i32, y:i32) -> i32 {
    // No semicolon => returns value
    x + y
}

