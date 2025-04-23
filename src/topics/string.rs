// String is mutable.
// String size can change at runtime.
// String stored on the stack with a pointer to the heap.
// Value of String is stored on the heap.

// let s1 = String::from("Hello");

// Stack Memory - s1
// name     | value
// ptr      | address to the heap
// len      |   5
// capacity |   5

// ptr: Pointer to data stored on the heap.
// len: Number of bytes used to store the string.
// capacity: Total amount of memory received from the allocator.
// Size of s1 will be 24 bytes. 3 * 8 bytes for the pointer, length and capacity.
// Note: the length will always be less than or equal to the capacity.
// Note: You can create a String with a defined capacity using String::with_capacity(int)

// Heap Memory
// index    |  value
// 0        |  H
// 1        |  e
// 2        |  l
// 3        |  l
// 4        |  o

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

fn exercise_1() {
    let mut s = String::from("hello, ");
    s.push_str("world"); // string slice or string literal `&str`
    s.push('!'); // char

    assert_eq!(s, "hello, wordl!");
    move_ownership(s);

    println!("Success!");
}

fn exercise_2() {
    let mut s = String::from("hello, world");
    let slice1 = &s;
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    let slice3 = &mut s;
    slice3.push('l');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}

pub fn exercise_3() {
    let s = String::from("hello, ぁまピ");
    let slice1 = &s[..1]; // ASCII char like `h` only takes 1 byte in UTF8 format

    let slice2 = &s[7..10]; // Unicode char like `ぁ` takes 3 bytes in UTF8 format
    println!("Slice2: {}", slice2);
    assert_eq!(slice2, "ぁ");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, 'ぁ')
        }
    }

    println!("Success!")
}

// You can use utf8_slice to slice UTF8 string, it can index chars instead of bytes
use utf8_slice;
pub fn example_4() {
    let s = "The 🚀 goes to the 🌘";

    let uft8_slice = utf8_slice::slice(s, 4, 5);
    println!("UTF8 Slice - {}", uft8_slice);

    let bytes_slice = &s[4..8];
    println!("Bytes Slice - {}", bytes_slice);
}

pub fn exercise_4() {
    let mut s = String::new();

    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap_or_default();

    assert_eq!(s, s1);
}
