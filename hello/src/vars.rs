pub fn run() {
    // Immutable
    let name = "Kahlia";
    // Mutable
    let mut age = 18;
    println!("My name is {} and I am not {}", name, age);
    age = 28;
    println!("My name is {} and I am {}", name, age);

    // Constant -- must define type for constant
    const ID: i32 = 0099;
    println!("ID #{}", ID);

    // Multivar assignment
    let (my_name, my_age) = ("Kahlia", 28);
    println!("My name is {} and I am {}", my_name, my_age);

}