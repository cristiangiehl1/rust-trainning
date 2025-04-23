// Like arrays but *dynamically sized*, meaning they can grow and shrink.
// Allocated on the *heap* as *contiguous* block of memory.
// All elements must have the *same type*.
// Special macro: `vec!`

fn is_vec(v: Vec<u8>) {}

// How to declare a Vector and convert an Array to Vector.
pub fn vectors_exercise_1() {
    let arr = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(v);

    let v = vec![1, 2, 3];
    is_vec(v);

    let v = vec![1, 2, 3];
    is_vec(v.clone());

    let mut v1 = Vec::new();
    is_vec(v1.clone());

    for i in &v {
        v1.push(*i);
    }

    assert_eq!(v, v1);

    println!("Success!")
}

// How to extend a Vector using `extend` method. Extends receive a iterator.
pub fn vectors_exercise_2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop(); // [1, 2]
    v1.push(3); // [1, 2, 3]

    let mut v2 = Vec::new();
    // here we can pass v1 values to v2.
    v2.extend(&v1);

    assert_eq!(v1, v2);

    let s1 = String::from("This chars will be extended as Vector values");
    let mut v3 = Vec::new();

    v3.extend(s1.chars());

    println!("{:?}", v3);

    println!("Success!");
}

// Types conversion to Vector
pub fn vectors_exercise_3() {
    // Array -> Vector
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);

    // String -> Vector
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();

    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]); // [0, 0, 0, .., 0]

    println!("Success!");
}

pub fn vectors_exercise_4() {
    let mut v = Vec::from([1, 2, 3]);

    for i in 0..5 {
        match v.get(i) {
            Some(_e) => v[i] += 1,
            None => v.push(i + 2),
        }
    }

    for i in 0..5 {
        println!("{:?}", v[i]);
    }
}

// A Vec can be mutable. On the other hand, slices are read-only objects. To get a slice, use `&`.
// In Rust, it's more common to pass slices as arguments rather than vectors when you just want to provide read access. The same goes for `String` and `&str`.
pub fn vectors_exercise_5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[..v.len()];

    assert_eq!(slice1, slice2);

    // Slices are ready only, they are immutable.
    // Note: slice and &Vec are different
    let vec_ref = &mut v;
    (*vec_ref).push(4);
    let slice3 = &v[0..];
    // slice3.push(4); // we could not use slice to mutate the Vector 'v', this will cause compile error.

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}

// Capacity
// The capacity of a vector is the amount of space allocated for any future elements that will be added onte the vector.
// This is not to be confused with the length of a vector, which specifies the number of actual elements within the vector.
// If a vector's length exceeds its capacity, its capacity will automatically be increased, but its elements will have to be reallocated.
//
// For example, a vector with capacity 10 and length 0 would be an empty vector with space for 10 more elements. Pushing 10 or fewer elements
// onto the vector will not change its capacity or cause reallocation to occur. However, if the vector's length is increased to 11, it will have
// to reallocate, which can be slow. For this reason, it is recommended to use Vec::with_capacity whenever possible to specify how big the vector
// is expected to get.
