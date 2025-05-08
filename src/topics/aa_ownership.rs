// # Ownership
//
// Rust's ownership system is unique and sets it apart from other programming languages.
// Set of rules that govern how memory is managed in Rust.
// Rules are enforced at compile time.
// If any of the rules are violated, the code will not compile.

// Three Rules of Ownership:
// 1. Each value in Rust has a variable that’s called its “owner”.
// 2. A value can only have one owner at a time.
// 3. When the owner of a value goes out of scope, Rust will automatically drop the value.
//    This is called "automatic memory management" or "garbage collector".

// Owner: The owner of a value is the variable or data structure that holds it and is responsable
// for allocating and freeing the memory used to store that data.

// Ownership prevents memory safety issues:
// - Dangling pointers: A pointer that references a memory location that has already been freed.
// - Double free: Attempting to free the same memory location more than once.
// - Memory leaks: Memory that is allocated but never freed.
