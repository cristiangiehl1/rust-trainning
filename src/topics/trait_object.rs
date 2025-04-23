// We can also use the impl Trait syntax in the return position to return a value of same type that implements a trait.
// However, you can only use impl Trait if you're returning a single type, use Trait Objects instead when you really need to return different several types.
// Using `impl Trait` doesn't work when returning multiple types.
// Different implementations of a trait probably different amounts of memory, but sizes of types must be known at compile time.
// In this case, trait objects can be used.
// A trait object is essentially a pointer to any type that implements the given trait, where the precise type can only be known at runtime.

use std::fmt::Debug;

#[derive(Debug)]
struct Sheep {}
#[derive(Debug)]
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "Baaaaaaah".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "Mooooo".to_string()
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// FIX the errors here, you can make a fake random, or you can use Trait Object.
fn random_animal(random_number: f64) -> impl Animal {
    if random_number < 0.5 {
        Sheep {}
    } else {
        Sheep {} // Cow {} This will not compile, because we are returning different types.
                 // We can use a trait object to return different types that implement the same trait.
    }
}

// dyn is a prefix that indicates that the type is a trait object.
pub fn random_animal_trait_object(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// Here we have a function which returns a type that implements the Animal trait. This could be Dog or Cat. As the trait object is behind a pointer,
// the size is known at compile time, which is usize (size of a pointer).
// This allows for more flexible code as the exact return type doesn't have to be known at compile time as long as the size is fixed.
pub trait Animal2: Debug {}

#[derive(Debug)]
struct Dog2;
#[derive(Debug)]
struct Cat2;

impl Animal2 for Dog2 {}
impl Animal2 for Cat2 {}

pub fn reuturn_animal_2(s: &str) -> &dyn Animal2 {
    match s {
        "dog" => &Dog2,
        "cat" => &Cat2,
        _ => panic!("Unknown animal"),
    }
}

#[derive(Debug)]
pub struct Animal3 {
    name: String,
    age: u8,
}

pub trait Actions: Debug {}

#[derive(Debug)]
struct Dog {
    info: Animal3,
}
#[derive(Debug)]
struct Cat {
    info: Animal3,
}

impl Actions for Dog {}
impl Actions for Cat {}

pub fn return_animal(s: &str) -> Box<dyn Actions> {
    match s {
        "dog" => Box::new(Dog {
            info: {
                Animal3 {
                    name: "Dog".to_string(),
                    age: 5,
                }
            },
        }),
        "cat" => Box::new(Cat {
            info: {
                Animal3 {
                    name: "Cat".to_string(),
                    age: 3,
                }
            },
        }),
        _ => panic!("Unknown animal"),
    }
}

// Array with trait objects
pub trait Bird {
    fn quack(&self);
}

pub struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying!")
    }
}

pub struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck... oh sorry, the swan is flying!")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan")
    }
}
