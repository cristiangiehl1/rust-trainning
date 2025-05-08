// # Borrowing
//
// Way of temporarily access data without taking ownership.
// When borrowing, you're taking a reference (pointer) to the data, not the data itself.
// Prevention of `dangling pointers` and `data races`.
// Data can be borrowed as `mutable` or `immutable`.
//
// There are certain rules when borrowing which we have to comply with, otherwise the compiler will throw an error.
// 1. You can have either one mutable reference OR any number of immutable references.
// 2. References must always be valid.
pub fn _test_borrowing() {
    let s1 = String::from("hello");

    let len = _calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
}

fn _calculate_length(s: &String) -> usize {
    s.len()
}

pub fn _viloate_borrowing_1() {
    let mut _s1 = String::from("hello");
    let _r1 = &_s1; // immutable borrow
    let _r2 = &_s1; // immutable borrow
                    // let r3 = &mut s1; // mutable borrow
                    // println!("{} {} {}", r1, r2, r3); // error: cannot borrow `s1` as mutable because it is also borrowed as immutable
}

pub fn _violate_borrowing_2() {
    let mut s1 = String::from("hello");
    let _r1 = &mut s1; // immutable borrow
                       // let r2 = &mut s1; // immutable borrow
                       // println!("{} {}", r1, r2); // error: cannot borrow `s1` as mutable more than once at a time
}

pub fn _borrowing() {
    let mut s = String::from("hello");
    let r1 = &s; // immutable borrow
    let r2 = &s; // immutable borrow
    println!("{} {}", r1, r2); // no error: immutable borrows are allowed

    let r3 = &mut s; // mutable borrow
    println!("{}", r3); // no error: r1 and r2 are no longer used
}
