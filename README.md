# ⚙️ Rust 

## Top Resources ##
* [The Rust Book](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
* [Half an hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) -- great quick reference
* [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
* [crates.io](https://crates.io/)
* [Stanford CS 110: Safety in Systems Programming](https://reberhardt.com/cs110l/spring-2020/)

<hr>

## 🦀 Rustacean Lingo 
* `rustup` = rust installer
* `rustc` = rust compiler
* `cargo` = rust package manager
* `crate` = library
* `package` = 1+ crates which provide a functionality
* `module` = organises code and defines the privacy boundary
* `glob` = {set} of functions/methods imported from crate::module ie. use std::cmp::{min, max}
* `turbofish` = ::<> syntax format

<hr>

## Hello World
Like C++, rust files can be compiled an run as binaries
```
rustc hello.rs
./hello
```
Typically, rust programs are managed and compiled by cargo
```
cd path/to/project
cargo run <cli args>
```

<hr>

## Primitives, Variables & Types
* Variables hold primitive data or references
* Rust provides type safety via static typing. Type can be decalred or infered by compiler. 
* Each signed variant can store numbers from -2**(n - 1) to 2**(n - 1) - 1 
* Unsigned variants can store numbers from 0 to 2**(n - 1)
* Integr overflow should be handled with `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*` methods
* By default, variables are immutable but can be overrriden using the `mut` modifier
* Rust is a block scoped language which allows variable shadowing
* if-else branches must return the same type
* Can use ::From and ::Into traits to convert types

<hr>

## Functions
* Function definitions require type annotations by design
* Statements are instructions that perform some action and do not return a value. 
* Expressions evaluate to a resulting value.
* Return using `return` or omit semi-colon
* Methods are defined within the context of a type using the implementation `impl` block function

<hr>

## Ownership & Rust's Memory Model
* Ownership is Rust's principal feature and defines the rules for memory management. 
* A program **will not compile** if any of the rules are violated --> Rust can make memory safe
guarantees without the use of a garbage collector or manual memory management.
* Recall:
    * Stack memory = LIFO --> For data structures with known, fixed size
    * Heap memory = allocates appropriate "chunk" of memory and returns pointer to that location
    * Retrieving and writing to heap memory is slower than stack memory
    * At execution, function variables (incl. heap pointers) are pushed onto the stack
* Ownership keeps track of heap & statck data -- aiming to minimise the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space
* Once a variable goes out of scope, the memory is automatically returned/deallocated 

**Rules of Ownership**
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, Rust automatically calls the `drop` destructor to deallocate memory

**Double Free**
* "double free" -- when we try to free two references pointing to the same heap location
* first heap variable is "moved" into the second and becomes an invalidated reference
* `clone` allows us to duplicate heap memory variables
* `Copy` trait allows us to duplicate stack memory variables

**References & Borrowing**
* Passing variables into functions moves/copies them into the scope of the function
* To mutate the variable value, can return the updated value to the variable or pass by mutable reference
* Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type
* A reference does not take ownership of the value, it "borrows" it
* Syntax: & = reference, * = dereference
* Cannot have simultaneous mutable references to the same data
* At any given time, you can have *either* one mutable reference or any number of immutable references.
* Rust also prevents "dangling references" -- ie. reference pointing to memory which has been deallocated/dropped

**Data Races**
* Data races cause undefined behaviour and are difficult to diagnose at runtime
* Rust will not compile if data races are present
* A data race occurs when:
    1. Two or more pointers access the same data at the same time.
    2. At least one of the pointers is being used to write to the data.
    3. There’s no mechanism being used to synchronize access to the data.

**Slices**
* Reference to contiguous sequence of elements, rather than entire data 

<hr>

## Module System
* Crates: binary/libray
* Packages: 1+ crates with Cargo.toml build file
* A package can contain 1 library crate and multiple binary crates
* Modules: organise code and define the privacy boundary
``` rust
my-project
    |--src
    |   |--main.rs //default binary crate root
    |   |--lib.rs //if exists, default library create root
    |   |--bin/
    |       |--bin1
    |       |--bin2
    |--Cargo.toml
```
Compiling Projects
``` rust
// Compile package
> cargo build
// Build and run binary root (ie. main.rs)
> cargo run
```

**Privacy**
* All Rust items are private by default. 
* Parent modules cannot access private child modules but child modules are in scope and can access private ancestor modules.