// Set of methods that can be implemented for multiple types in order to provide commom functionality and behavior between them.
// Traits consist of method signatures only, which then have to be implemented by the target type.
// Similar to "classes" in other languages, not quite the same though.
// Defines shared behavior in an abstract way.

use std::fmt::{Debug, Display};

trait Animal {
    fn sound(&self) -> String;
}

struct Sheep;
struct Cow;

impl Animal for Sheep {
    fn sound(&self) -> String {
        "Baa".to_string()
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        "Moo".to_string()
    }
}

// Traits as Parameters
// Traits can be used as parameters in functions, allowing for polymorphic behavior.
// The function notify() takes as argument any type that has implemented the Summary trait
pub fn notify_parameters(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bounds
// Similar to example using "impl Summary" but more verbose.
// Trait bounds are declared like generics, after name of the function.
pub fn notify<T: Animal>(cow: &T) {
    println!("Breaking news! {}", cow.sound());
}

// Use trait bounds if you have lots of parameters to avoid this:
pub fn sound(sheep: &impl Animal, cow: &impl Animal) {
    println!("Animals sounds! {} and {}", sheep.sound(), cow.sound());
}

// With trait bounds, you can do this:
pub fn soudn<T: Animal>(sheep: &T, cow: &T) {
    println!("Animals sounds! {} and {}", sheep.sound(), cow.sound());
}

// WHERE CLAUSES
// If you have multiple trait bounds, you can use where clauses to make it more readable.
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // function body
    0
}

//  You could do this instead:
fn some_function_where_clause<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // function body
    0
}

// Return Types that Implement Traits
// Here we have a trait Animal which is implemented for two structs, Dog and Cat.
// The two functions returns a struct, either Dog or Cat, that implements the Animal trait.

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) -> String {
        "Woof".to_string()
    }
}

impl Animal for Cat {
    fn sound(&self) -> String {
        "Meow".to_string()
    }
}

fn return_dog() -> impl Animal {
    Dog
}

fn return_cat() -> impl Animal {
    Cat
}

// Exercise 1:
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student;
struct Teacher;

impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I am a good student")
    }
}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I', not a bad teacher")
    }
}

// Exercise 2:
// Derive - The compiler is capable of providing basic implementations for some traits via the #[derive] attribute.
// `Centimeters`, a tuple struct that can be compared
// PartialEq => allows us to compare two instances of the struct using == and !=
// PartialOrd => allows us to compare two instances of the struct using <, >, <=, >=
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// Exercise 3:
// Operator - in  Rust, many of the operators can be overloaded via traits. That is, some operators can be
// used to accomplish different tasks based on their input arguments. This is possible because operators are syntatic sugar for method calls.
// For example, the + operator in a + b calls  the add method (as in a.add(b)). This add method is part of the Add trait. Hence, the + operator
// can be used by any implementor of the Add trait.
fn multiply<T: std::ops::Mul<Output = T>>(n1: T, n2: T) -> T {
    n1 * n2
}

// assert_eq!(6, multiply(2u8, 3u8));
// assert_eq!(5.0, multiply(1.0, 5.0));

// Exercise 5:
// Use trait as function parameters - instead of a concrete type for the item parameter, we specify the impl keyword and the trait
// name. This parameter accepts any type that implements the specified trait.
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn summary<T: Summary>(a: &T) {
    let output = a.summarize();
    println!("Summary: {}", output);
}

// Trait Bound + Static Dispatch declared => fn [fn_name]<T: MyTrait>(x: T) -> T {} // Fixed size known at compile time.
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        42
    }
}

impl MyTrait for String {
    fn f(&self) -> Self {
        self.clone()
    }
}

// Trait Bound + Static Dispatch declared => fn [fn_name]<T: MyTrait>(x: T) -> T {} // Fixed size known at compile time.
fn my_function<T: MyTrait>(x: T) -> T {
    x.f()
}

// Object Safe
// You can only make object-safe traits into trait objects. A trait is object safe if all the methods defined in the trait have the following properties:
// - The return type ins't `Self`.
// - There are no generic type parameters.
trait MyTraitDyn {
    fn f(&self) -> Box<dyn MyTraitDyn>;
}

impl MyTraitDyn for u32 {
    fn f(&self) -> Box<dyn MyTraitDyn> {
        Box::new(42)
    }
}

impl MyTraitDyn for String {
    fn f(&self) -> Box<dyn MyTraitDyn> {
        Box::new(self.clone())
    }
}

fn my_function_dyn(x: Box<dyn MyTraitDyn>) -> Box<dyn MyTraitDyn> {
    x.f()
}
