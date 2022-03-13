/*
**Rules of Ownership**
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, Rust automatically calls the `drop` destructor to deallocate memory

* A reference does not take ownership of the value, it "borrows" from
* Syntax: & = reference, * = dereference
* Cannot have simultaneous references (mutable and/or immutable) to the same data
*/

fn immutable_ref(s: &String) {
    // Error
    // s.push_str(", world");
    println!("{} is immuatble",s);
}

fn mutable_ref(s: &mut String) {
    s.push_str(" world");
    println!("s is now '{}'",s);
}

// fn dangle(s: String) -> &String { 
//     // DANGER! Returning reference to s which will go out of scope
//     &s 
// }

fn undangle(s: String) -> String { 
    // Directly return the string
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // Iterator
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //return the slice
        }
    }
    &s[..] //return the entire slice
}

pub fn run() {
    /* double-free error
    let s1 = String::from("hello");
    let s2 = s1;
    s1 has been "moved" into s2 --> s1 is now an invalidated reference --> prevent double freeing of heap memory
    println!("{}, world!", s1);
    */

    // Explicitly clone/deep copy the heap memory
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // By default, references are immuatable   
    immutable_ref(&s1);
    mutable_ref(&mut s2);

    // Multiple references using scopes
    let mut s = String::from("underwater");
    {
        let r1 = &mut s;
        println!("Mutating s by r1");
        mutable_ref(r1);

    }
    let r2 = &mut s;
    println!("Mutating s by r2");
    mutable_ref(r2);

    // Dangling references
    let p = String::from("amazing");
    // println!("{}", dangle(p));
    println!("{}", undangle(p));

    // Slices
    println!("{}", first_word(&s));
}