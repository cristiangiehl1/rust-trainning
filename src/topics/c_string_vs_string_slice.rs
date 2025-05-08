// # String:
// - A String is a `heap-allocated` string type that owns its contents and is mutable.
// - Use String if you need to own the data and be able to mutate it.

// # &str - string slice:
// - A &str is an `immutable` sequence of UTF-8 bytes in memory, it does not own the underlying data and is immutable.
// - Think of &str as a view on a sequence of characters (stored as UTF-8 bytes) in memory.
// - A &str is a borrowed reference to a String, while String is an owned string type.
// - Use &str if you just want to a view of a string, and you don't need to modify it.
// - &str is more lightweight and efficient than String, as it does not require heap allocation.

// # String Literal:
// - A string literal is a sequence of characters enclosed in double quotes("").
// - Fixed size, compile-time known sequence of UTF-8 bytes.
// - The type is &'static str, which indicates the data is stored in `static memory`,
// meaning it is valid throughout the entire lifetime of the program.
// - The data is hardcoded into the executable and stored in read-only memory, meaning they are immutable.
// let s: String = String::from("hello world");

// let hello: String = &s[0..5]; // hello
// let world: String = &s[6..11]; // world

// s
// name       | value
// -----------|----------------
// ptr        | heap address (0x7f8c4b2a3c00)
// len        | 11
// capacity   | 11

// world is a &str points to a sequence of characters in the heap memory
// name       | value
// -----------|----------------
// ptr        | heap address (0x7f8c4b2a3c00)
// len        | 5

// index      | value
// -----------|----------------
// 0          | h              // s pointer
// 1          | e
// 2          | l
// 3          | l
// 4          | o
// 5          | (space)
// 6          | w            // world pointer
// 7          | o
// 8          | r
// 9          | l
// 10         | d

pub fn concatenate_strings() {
    // you can only concatenate strings if the first string is a String and the others variables are &str
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    println!("s3: {}", s3);
    // println!("s1: {}", s1); // error: value moved
    println!("s2: {}", s2); // ok, s2 is borrowed
}

pub fn string_index() {
    let s1 = String::from("hi, #$");
    let h = &s1[0..1]; // h is a &str
    assert_eq!(h, "h");

    let h1 = &s1[3..5]; // h1 is a &str
    assert_eq!(h1, "#$");
}

pub fn string_iterator() {
    let s1 = String::from("hello");
    let mut iter = s1.chars();

    for c in iter {
        println!("{}", c);
    }
}
