pub fn run() {    
    // Print macro
    println!("Hello, world!");
    
    // Formatting traits -- fmt::display
    println!("Hi I'm {}, a newbie {}!", "Kahlia", "Rustacean");
    
    // Positional formatting arguments 
    println!("I'm {0}, a newbie {1}acean to the {1} language!", "Kahlia", "Rust");
    
    // Named formatting arguments
    println!("Hi I'm {name}, {vibe} to be learning Rust!", name="Kahlia", vibe="stoked");

    // Placeholder traits
    println!("12 in binary is {x:b}, in hex is {x:x}, and in octal in {x:o}",x=12);

    // fmt::debug
    println!("This is a debug statement: {:?}", (2, true, "blue"));
    
    // pretty print
    println!("This is a pretty debug statement: {:#?}", (2, true, "blue"));
}