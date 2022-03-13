/*
* `Option` is an enum which encodes the concept of a value being present or absent
* To enforce compiler safety, Rust does not have an explicit `Null` value type
* Ensures we don't assume not-null valuen case, when it could be present
* Must convert an Option<T> to a T before performing operations 
    --> Prevents the assumption something isn’t null when it actually is.
* `match` control flow allows us to define what happens when there is Some(T) and None
* compiler confirms that both cases are handled

*/

/* Option implementation definition
enum Option<T> {
    None,
    Some(T),
}
*/
pub fn run() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

