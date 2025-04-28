// Result
//
// `Result` is an *enum* type that represents the *outcome* of an operation that *could potentially fail*.
//
// Two possible variants:
// - Ok(T): A value *T was found*;
// - Err(e): An *error was found* with a value `e`.
//
// The *expected* outcome is *Ok*, the *unexpected* outcome is *Err*.
//
// Since `Result` is an enu,. the possible variantes can be matched using a *match pattern*.

use std::num::ParseIntError;

// The functuion divide() returns a `Result` that holds *f32 float* in case of *success* and a *string literal* in case of an *error*.
// If the second argument provided is 0.0 we return an `Error`, because *dividing by 0 is not allowed*. Otherwise we divide the two arguments
// and return the result *wrapped in Ok()*.
fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("Division by zero");
    }
    Ok(x / y)
}

// When calling this function we receive a value of type Result. We can the *match* that result and specify what will be the *output*
// in case of *success* or in case of an *error*.
fn result_example_01() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}

// Unwrap
//
// The `unwrap()` method takes as input a value of *type Result* and takes out the value which is wrapped inside Ok(T) in case of success
// or panics in case of an error.

// ? => Question Mark Operator
//
// The `?` operator is a *shorthand* way to propagate errors or unwrap Ok() results.
// Basically the same as unwrap() but instead of panic *returns an error*.
// *Replaces* an *entire match statement*.
// Can be used in the *main() function*.
fn result_example_02() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    println!("{}", number);
    Ok(())
}

fn question_mark_operator_example_01() -> Result<(), ParseIntError> {
    let number_str = "10";
    let n = number_str.parse::<i32>()?;
    println!("{}", n);
    Ok(())
}

// Type Alias
//
// Way of giving a new name to an existing type.
// Don't confuse with associated types in traits!
type U64 = u64;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn question_mark_operator_example_02() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}

// Map & and_then
//
// `map` -> Maps a result<T, E> to Result<U, E> by applying a function contained `Ok` value, leaving an `Err` value untouched.
// This function can be used to compose the results of two functions.
// Closure -> FnOnce(T) -> U
fn add_two_map(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}

// `and_then` -> Calls `op` if the result is `Ok`, otherwise returns the `Err` value of `self`.
// This function can be used for control flow based on `Result` values.
// Closure -> F: FnOnce(T) -> Result<U, E>
fn add_two_and_then(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|n| Ok(n + 2))
}

//
