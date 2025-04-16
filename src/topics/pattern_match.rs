// Powerful construct that allows to compare a value agains a set of patterns, the execute
// - different code based on which pattern matched.
// Patterns can be made up of literal values, variable, names, wildcards, etc...
// In match, all possible cases must be handled, enforced by the compiler.

// Here, we have an enum Coin which holds different denomitations of US coins.
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// The fn pattern_match_example takes as argument a Coin (which can only hold one field of the enum) and checks,
/// which field in the enum has been matched. Then returns the right amount as u8.
pub fn pattern_match_example(coin: Coin) -> u8 {
    let coin_value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    println!("Coin value: {}", coin_value);
    coin_value
}

// matches! macro is a powerful macro that allows to match a value against a pattern and returns true or false
// - matches!(value, pattern) returns true if value matches the pattern, false otherwise
pub fn matches_example() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A' ..='Z' | 'a' ..='z' | '0' ..='9'))
    }

    println!("Success!");
}

enum MyEnum {
    Foo,
    Bar,
}

pub fn matches_example_2() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    println!("Count of Foo: {}", count);
    assert_eq!(count, 2);
    println!("Success!");
}

pub fn match_example_2(n: i32) {
    match n {
        1 => println!("One"),
        2..=5 => println!("Two to Five"),
        6..=10 => println!("Six to Ten"),
        _ => println!("match -infinite -> 0 or 11 -> +infinite"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

// pattern binding or subpattern matching
// - allows to match a pattern and bind the value to a variable
pub fn match_example_3() {
    let p = Point { x: 1, y: 2 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point {
            // I could not use x here to println! because it's not bind to a value
            // otherwise y is bind to a value using y @
            // (10 | 20 | 30) which is a pattern
            x: 0..=5,
            y: y @ (10 | 20 | 30),
        } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    println!("Success!");
}

enum Message {
    Hello { id: i32 },
}

pub fn match_example_4() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range [3, 7]: {}", id);
        }
        Message::Hello {
            id: new_id @ (10 | 11 | 12),
        } => {
            println!("Found an id in range [10, 12]: {}", new_id);
        }
        Message::Hello { id } => {
            println!("Found an id: {}", id);
        }
    }
}

// A MATCH GUARD is an additional if condition specified after the pattern in a match that must also match, alogn with the pattern matching,
/// for that arm to be chosen.
pub fn match_guard_example() {
    let num = Some(4);
    let split = 5;

    match num {
        // additional condition to check if the value is less than split
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}

pub fn match_ignoring_remaining_parts() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 4096);
        }
    }
}
