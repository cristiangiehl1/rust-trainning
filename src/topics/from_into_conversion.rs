// `From` and `Into` traits are used for type conversions between different types without requiring explicit casts.
// Part os standard library.
// Can be implemented for custom types.
// Implementing `From` for a type will give us `Into` implementation for the given type for free.
//
// Example
// Here we  have created a struct Number with one field value.
// We want to be able to convert an i32 value directly to a Number type value in the value field.
// We do this by implementing the From trait for our custom type Number and provide the customized from method.
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self { value: n }
    }
}

pub fn test_from_into() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);

    println!("Success!")
}

// The `From` trait allows for a type to define how to create itself from another type, hence
// providing a very simple mechanism for converting between several types.
//
// The `From` and `Into` traits are inherently linked, and this is actually part of its implemantation.
// It means if we write something like this: `impl From<T> for U`, the we can use let u:U = U::from(T) or let u:U = T.into().
//
// The `Into` trait is simply the reciprocal of the `From` trait. That is, if you have implemented the
// `From` trait for your type, then the `Into` trait will be automatically implemented for the same type.
//
// Using the `Into` trait will typically require the type annotations as the compiler is unable to determine this most of the time.
//
// Example converting `&str` into `String`
fn converting_string_slice_into_string() {
    let my_str = "hello";

    let _string1 = String::from(my_str);
    let _string2 = my_str.to_string();
    let _string3: String = my_str.into();
}

fn from_into_exercise_01() {
    let i1: i32 = false.into();
    let i2 = i32::from(false);

    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let _i3: u32 = 'a'.into();

    let s1 = 'a'.to_string();
    let s2 = String::from('a');
    let s3: String = 'a'.into();

    assert_eq!(s1, s2);
    assert_eq!(s2, s3);

    println!("Success!");
}

// When performing error handling it is often useful to implement `From` trait for our own error type. Then we can use `?`
// to automatically convert the underlying error type to our own error type.
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(e: num::ParseIntError) -> Self {
        CliError::ParseError(e)
    }
}

fn from_into_exercise_02(file_name: &str) -> Result<i32, CliError> {
    // `?` automatically converts io::Error to CliError
    let contents = fs::read_to_string(file_name)?;
    // num::ParseIntoError -> CliError
    let num = contents.trim().parse()?;
    Ok(num)
}

// TryFrom/TryInto
//
// Similar to `From` and `Into`, `TryFrom` and `TryInto` are generic traits for converting between types.
//
// Unlike `From/Into`, `TryFrom` and `TryInto` are used for fallible conversions and return a `Result` instead of a plain value.
fn tryfrom_tryinto_exercise_01() {
    let n = 256_i16;

    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!(
                "there is an error when converting: {:?}, but we catch it",
                e.to_string()
            );
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn tryfrom_tryinto_exercise_02() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));

    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!")
}

// Convert any type to String
//
// To convert any type to `String`, you can simply use the `ToString` trait for that type. Rather
// than doing that directly, you should implement the `fmt::Display` trait which will automatically provides `ToString`
// and also allows you to print the type with `prinln!`.
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn try_into_exercise_03() {
    let origin = Point { x: 0, y: 0 };

    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");
}

// Parse a String
//
// We can use `parse` method to convert a `String` into a `i32` number, this is
// because `FromStr` is implemented for `i32` type in standard library: `impl FromStr for i32`.
// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;

pub fn from_into_exercise_04() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;

    assert_eq!(sum, 35);

    println!("Success!");
}
