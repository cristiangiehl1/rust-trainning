// Way of defining a type with only one of a possible set of values
// We can only access one variant of an enum at a time
// Can hold additional information using tuples
// Especially useful whe using in match statements

enum IpAddr {
    V4(String),
    V6(String),
}

pub fn enum_test() {
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
}

// Explicit discriminator
enum Number {
    Zero,  // 0
    One,   // 1
    Two,   // 2
    Three, // 3
}

enum Number1 {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

// C-like enum, you cannot use as a discriminator a point floating number
enum Number2 {
    Zero = 5,
    One, // 6
    Two, // 7
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum_test2() {
    let _m = Message::Move { x: 10, y: 20 };
    let _m1 = Message::Write(String::from("Hello"));
    let _m2 = Message::ChangeColor(255, 255, 255);
    let _m3 = Message::Quit;

    // Destructuring
    if let Message::Move { x, y } = _m {
        println!("x: {}, y: {}", x, y);
    }
}
