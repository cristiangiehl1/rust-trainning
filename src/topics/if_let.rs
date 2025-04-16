// In a match statement, every case has to be handled. This sometimes leads to annoying boilerplate code
/// that is not necessary. Instead we can use if let to unwrap a value of an Option type.
pub fn _if_let_example() {
    let o = Some(7);

    if let Some(i) = o {
        println!("Unwrapped value: {}", i);
    }

    println!("Success!");
}

enum Foo {
    Bar(u8),
    Baz(u8),
    Qux(u32),
}

pub fn _if_let_example_2() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("Unwrapped value: {}", i);
    } else {
        println!("Not a Bar");
    }
    println!("Success!");
}
