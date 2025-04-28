// Type conversion also called type casting is coercing primitive types that can be performed by as keyword.
// as conversions can be chained.
// When casting to an unsigned type T, T::MAX + 1 is added or subtracted until the value fits into the new type.
// Using unsafe methods can lead to undefined behavior.
//
// Example
// Largest possible 8-bit integer (unsigned): 255
//
// | Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0 |
// |-------|-------|-------|-------|-------|-------|-------|-------|
// |   1   |   1   |   1   |   1   |   1   |   1   |   1   |   1   |
// | 128   |  64   |  32   |  16   |   8   |   4   |   2   |   1   |
//
// This means u8::MAX + 1 = 256
//
// Example: 1000 as u8 = 1000 - 256 = 744
//                                  = 744 - 256 = 488
//                                              = 488 - 256 = 232
// Example: -1_i8 as u8 = 255
//
// Example: 97.132_f32 as u8 = 97
//
// Note: we can allow overflowing using the attribute #[allow(overflowing_literals)]. Overflow is whenever the type coercion have to add or subtract a value
// to fit the `as` typed size.
//
// Since Rust 1.45, the `as` keyword performs a *saturating cast*
// when casting from float to int. If the floating point value exceeds
// the upper bound or is less than the lower bound, the returned value
// will be equal to the bound crossed.
// assert_eq!(300.1_f32 as u8, 255);
// assert_eq!(-100.1_f32 as u8, 0);

// This behavior incurs a small runtime cost and can be avoided
// with unsafe methods,  however the results might overflow and
// return **unsolved values**. Use these methods wisely:
#[allow(overflowing_literals)]
pub fn unsafe_example() {
    unsafe {
        // 300.0 is 44 -> 300 - u8::MAX + 1 = 300 - 256 = 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156 -> u8::MAX + 1 = 256 - 100 = 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
