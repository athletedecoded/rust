/*
cargo run hello
cargo run status
*/

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    //  args[0] = target of executable file
    let command = args[1].clone();
    let name = "Homer";
    let status = 42;
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hello {}", name);
    }if command == "status" {
        println!("Status: {}%", status);
    }
}