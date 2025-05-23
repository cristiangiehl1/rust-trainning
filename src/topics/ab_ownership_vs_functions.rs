pub fn ownership_vs_functions() {
    let s = String::from("hello"); // s comes into scope.

    takes_ownership(s); // s's values moves into the function...
                        // ... so if we use s here, it will throw an error

    // println!("{}", s); // This would cause an error because s is no longer valid.

    let x = 5; // x comes into scope.
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still use x afterward

    println!("Print original x value {}", x); // This will work because i32 implements the Copy trait.
} // Here, x goes out of scope, the s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("Print x value with copy trait {}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

pub fn ownership_vs_functions_2() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope.
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope.
    some_string // some_string is returned and moves out to the calling function
} // Here, some_string goes out of scope and is dropped. Its memory is freed.

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
