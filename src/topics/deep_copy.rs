//  Deep Copy:
// Whenever you want to copy the data of a variable that is allocated in the HEAP memory.
// We have to explicitly tell the compiler to COPY the data.

pub fn deep_copy() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // this will create a DEEP COPY of s1, so s2 is a new variable with its own data.

    println!("s1: {}, s2: {}", s1, s2); // this will work, because s1 and s2 are both owners of their own data
}
