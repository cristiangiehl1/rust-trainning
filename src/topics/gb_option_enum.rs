// Option is an enum that represents a value that may or may not be present.
// Known in other languages as "null", refering to the absence of a value.
// Used to handle cases where a function or method might fail to return a value.

// enum Option<T> {
//     None,    // Represents a value that is absent
//     Some(T), // Represents a value that is present
// }

pub fn _example_option_enum() {
    let five = Some(5);
    let six = plus_one(five); // plues_one() expects an argument of type Option, so we have to wrap an i32 inside Some().
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
