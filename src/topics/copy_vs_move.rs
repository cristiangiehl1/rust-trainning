// Scalar values with fixed sizes (all types we covered at the beginning)
/// will automatically get COPIED in the stack, copying here is cheap.
// Dynamic sized data won't be copied, but MOVED, copying would be too expensive.

pub fn copy_vs_move() {
    let x = 5; // the integer value of variable x will get copied into y and both variables are usable
    let y = x; // because i32 value has been copied. i32 is fixed size!
    println!("x: {}, y: {}", x, y); // this will work, because x is still the owner of the value

    let s1 = String::from("Hello"); // s1 is the owner of the string. As s1 is just a pointer to data on the heap,
                                    // just the pointer will get copied into s2, NOT the data itself.
                                    // This is called "move" in Rust.
    let s2 = s1; // s1 is moved to s2, now s2 is the owner of the string. So, the first variable s1 will be dropped and cannot be used after
                 // assigning it to s2, to avoid dangling pointers.
                 // println!("{}", s1); // this will throw an error, because s1 is no longer the owner of the string
    println!("s2: {}", s2); // this will work, because s2 is the owner of the string
}
