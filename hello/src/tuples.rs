/*
Tuples have a max capacity of 12 elements
*/

pub fn run() {
    let person: (&str, &str, i32) = ("Kahlia", "Colorado", 80301);

    println!("Name: {}, State: {}, Postcode: {}", person.0, person.1, person.2);
}