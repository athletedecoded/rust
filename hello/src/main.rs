/*
cd hello
cargo run
*/

// Declare module imports
mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod loops;
mod fxns;
mod structs;
mod enums;
mod cli;


fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    loops::run();
    fxns::run();
    structs::run();
    enums::run();
    cli::run();
}
