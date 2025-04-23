// Allow to specify a type that is associated with the trait.
// When implementing the trait for a specific type we have to specify the concrete type.
// Basically a type placeholder that the trait methods can use in their signature.
// Similar to generic types but are more flexible because they allow a trait to have different associated types for different implementing types.
trait MyTrait {
    // here we define a trait that has an associated type and a method that returns a value of this type
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
}

struct MyStruct {}

impl MyTrait for MyStruct {
    // when implementing the trait for a specific type (MyStruct), then we have to give the associated type MyType a concrete type,
    // in this case i32.
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}
