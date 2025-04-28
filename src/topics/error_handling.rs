// Panic!
// Simplest form of *error handling* is to use the `panic!` macro.
// `panic!` will print out an *error message*, *unwind* the *stack* and finally *exit* the program.
// In multithreaded programs it will *exit* the thread in which the `panic!` occurs, not the whole program.
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");

        panic!()
    }

    println!("Exercise Failed if printing out this line!")
}

fn panic_example_01() {
    drink("lemonade");

    println!("Exercise Falied if printing out this line!");
}

fn production_rate_per_hour(speed: u16) -> f64 {
    // this will cause an overflow if speed >= 2
    let cph = 221; // 2 * 221 = 442 // u8 max value == 255
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed & cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y);
}

fn working_items_per_minutes(speed: u16) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}

// Common Panic Cases
fn panic_example_02() {
    // assert_eq did not match
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    let ele = v[2];

    // unwrap may panic when get return a None
    let ele = v.get(1).unwrap(); // Some(2) -> 2

    // Sometimes, the compiler is unable to find the overflow errors for you in compile time, so
    // a panic will occur
    let v = production_rate_per_hour(2);

    // because of the same reason as above, we have to wrap it in a function to make the panic occur.
    divide(15, 1);

    println!("Success!")
}

// Detailed call stack
//
// By default the stack unwinding will only give something like this:
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99' note: run with `RUST_BACKTRACE=1` enviroment variable to display a backtrace
//
// Though there is the reason of panic and the line of the code is showing where the panic has occured,
// sometimes we want to get more info about the call stack.
//
// Unwinding and abort
//
// By default, when a `panic` occurs, the program starts unwinding, which means Rust walks back up the stack and
// cleans up the data from each function it encounters.
//
// But this walk back and clean up is a lot of work. The alternative is to immediatly abort the program without cleaning up.
//
// If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting by adding
// below content to `Cargo.toml`:

// [profile.release]
// panic = 'abort'
