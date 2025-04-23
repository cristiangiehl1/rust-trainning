// Resolves method calls at compile time.
// Compiler generates functions code for each concrete type that implements trait.
// Calls appropriate function based on concrete type.
// Faster and more efficient than dynamic dispatch, but doesn't support flexibility.
trait Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) {
        println!("Woof");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("Meow");
    }
}

// When we run the code the compiler generates methods for each concrete type (Dog, Cat) and the called method sound() can be resolved because it is known at compile time
// which method for whick type has to be resolved.
// Note: The output might not actually look like that because compiler applies optimizations.
struct DogCompilerGenerated;
struct CatCompilerGenerated;

impl DogCompilerGenerated {
    fn sound(&self) {
        println!("Woof");
    }
}
impl CatCompilerGenerated {
    fn sound(&self) {
        println!("Meow");
    }
}
