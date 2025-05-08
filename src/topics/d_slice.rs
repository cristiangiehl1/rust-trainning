// # Slices
//
// Reference to contiguous sequence of elements in a collection.
// Provide a way to borrow part of a collection without taking ownership of the entire collection.
// Can be created from arrays, vectors, Strings and other collections implementing the Deref trait.

// slice has the type &[i32] in this example.
// Works like string slices do, by storing a reference to the first element and a length.
pub fn _example_slice_01() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

pub fn _example_slice_02() {
    let arr = ['₯', '₳', '￥'];

    let slice = &arr[..2];

    assert!(std::mem::size_of_val(&slice) == 16);
}

pub fn _example_slice_03() {
    let arr = [1, 2, 3, 4, 5];

    let slice = &arr[1..4];

    assert_eq!(slice, &[2, 3, 4]);
}
