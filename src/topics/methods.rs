// Methods are similar to functions:
// - Declare with fn keyword,
// - Can take parameters and return values,

// Unlike functions, methods are defined within the context of a struct(or an enum or trait object),
// - and their first parameter is always "self", which represents the instance of the struct the method is
// - being called on.

// Function that is associated with a particular type or struct
// Takes parameters and returns a value, but defined as a member of a struct or enum
// Called using dot notation (like accessing members of a struct)
// Implemented through the "impl" block

#[derive(Debug)]
struct Rectangle {
    // inside the impl block of the type Rectangle we define the method area() that doesn't take
    /// any arguments and returns the product of width and height of an instance as an u32 integer.
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // self refers to the instance the method is called upon, in this case, the rectangle instance.
        self.width * self.height
    }
}

// Here, we create an instance of Rectangle with values for width and height. Then we can call the method
/// using dot notation on the instance we've created.
const REACT1: Rectangle = Rectangle {
    width: 30,
    height: 50,
};

// println!(
//     "The area of the rectangle is {} square pixels.",
//     rect1.area()
// );

// "self" will take the ownership of current struct instance, however, "&self" wil only borrow a reference from the instance.
// This means that the instance will not be moved, and we can use it again after calling the method.

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("The current state of the traffic light is: {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

#[derive(Debug)]
pub enum TrafficLightColorEnum {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColorEnum {
    pub fn color(&self) -> &str {
        match self {
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Green => "green",
        }
    }
}
