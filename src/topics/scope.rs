// Range within a program for which an item is valid.

// Global scope:
// Accessible from anywhere in the program.

// Local scope:
// Accessible only within a specific block of code, such as a function or a loop.

pub fn print_scope() {
    let s = "hello";

    println!("s: {}", s);
}

// The variable `s` is only accessible within the block of code where it is defined.
