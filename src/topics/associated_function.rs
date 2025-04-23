// All functions defined within an "impl" block are called associated functions because
//- they're associated with the type named after the "impl" block. We can define associated functions
//- that don't have "self" as their first parameter (and thus are not methods) because the don't need
//- an instance of the type to work with.

// Functions that is associated with a struct or an enum, but doesn't take an instance as its first argument.
// Called using the name of the type, not and instance of it.
// Often used as constructors for a struct or enum.

struct Rectangle {
    width: u32,
    height: u32,
}

// new() is an associated function, because it doens't have "self" as its first parameter.
// So, to call the new() function, we don't need an instance of the struct.
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

// We can then call the associated function by using the name of the struct and the method name separated by "::".
// let rec1 = Rectangle::new(5, 10);

// println!("Rectangle 1: {} x {}", REC1.width, REC1.height);

pub struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}
