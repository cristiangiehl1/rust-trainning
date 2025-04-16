// Way to stored related pieces of information in a single variable
// Collection of values of different types grouped together as a single compound value (type composed of other types)
// Store as a fixed-size contiguous block of memory on the stack
// Signature is (T, T, T...)

pub fn tuples_01() {
    let _t0: (u8, i16) = (0, -1);

    let _t1: (u8, (i16, u32)) = (0, (-1, 1));

    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}

pub fn tuples_02() {
    let t: (&str, &str, &str) = ("i", "am", "sunface");

    assert_eq!(t.2, "sunface");
}

// LONG TUPLES CANNOT BE PRINTED
pub fn long_tuples() {
    let too_long_tuple = (
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    );
    // println!("too long tuple {:?}", too_long_tuple); // error: the trait `std::fmt::Debug` is not implemented for `T`
}

pub fn destructuring_tuples() {
    let t = (1, 2, 3);

    // destructuring
    let (a, b, c) = t;

    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);

    // destructuring with ignore
    let (a, _, c) = t;

    assert_eq!(a, 1);
    assert_eq!(c, 3);
}

pub fn destructuring_assignments() {
    let (x, y, z);

    // destructuring
    (x, y, z) = (1, 2, 3);

    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
}

pub fn tuples_as_function_arguments() {
    fn sum_mutiply(nums: (i32, i32)) -> (i32, i32) {
        (nums.0 + nums.1, nums.0 * nums.1)
    }

    let (x, y) = sum_mutiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}
