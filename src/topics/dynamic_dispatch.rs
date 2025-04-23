// Specified methods to be called is determined at runtime.
// Works by creating a reference or smart pointer to a trait object usegin `&dyn` or `Box<dyn>`.
// When trait object is created, compiler will build a vtable (virtual table) for that trait.
// - vtable is a table that contains a pointer to the implementation of each method in the trait for the specific type of the object that reference
// - points to.
// Compiler will do a lookup in a vtable to determine which method should be called for which type that implements the given trait.
// This lookup will cause overhead but allows for more flexible code.
// Overhead (extra programming cost) is when the compiler has to do a lookup in the vtable to determine which method should be called.

// let shapes: Vec<&dyn Area> = vec![&square, &circle];

// Memory layout - Trait Object (stack)
// name     | type       | value
// --------------------------------
// pointer  | *mut ()    | address of Square/Circle
// vtable   | *mut ()    | address of vtable

// VTable for Square (heap)
// - destructor
// - size
// - alignment
// - area function pointer

// VTable for Circle (heap)
// - destructor
// - size
// - alignment
// - area function pointer

// We have an Animal trait and two structs that implements this trait.
// We have a function that takes random number and returns a type that implements the Animal trait (either Dog or Cat).
// Compiler can not know size at compile time, as one type might be bigger than the other
// The noise() method gets called on returned type that implements Animal trait, again compiler can not know this, this resolves at runtime.
trait Animal {
    fn noise(&self);
}

struct Cat;
struct Dog;

impl Animal for Dog {
    fn noise(&self) {
        println!("woff")
    }
}

impl Animal for Cat {
    fn noise(&self) {
        println!("Meow")
    }
}

pub fn random_animal(random_number: u8) -> Box<dyn Animal> {
    if random_number < 10 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}
