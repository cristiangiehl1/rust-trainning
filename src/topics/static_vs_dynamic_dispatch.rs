// ================================================
// Static Dispatch vs Dynamic Dispatch (Rust)
// ================================================
//
// Feature                  | Static Dispatch               | Dynamic Dispatch
// -----------------------------------------------------------------------------------
// Resolved at              | Compile time                  | Runtime
// Syntax                   | Generics (<T: Trait>)         | Trait objects (&dyn Trait / Box<dyn Trait>)
// Performance              | Faster                        | Slightly slower (vtable lookup)
// Inlined by compiler (*)  | Yes                           | No
// Memory usage             | No extra memory               | Needs extra memory for vtable and pointer
// Flexibility              | Less flexible                 | More flexible
// Use case                 | When types are known          | When using polymorphism or mixed types
// Code bloat risk          | Higher (monomorphization)     | Lower
// Example                  | fn run<T: Animal>(a: T) {}    | fn run(a: &dyn Animal) {}
//
// (*) Inlined by compiler:
//     This means the compiler can optimize your code by copying the body of a function
//     directly into the place it's used, avoiding a function call.
//     Static dispatch allows this because the type is known at compile time.
//     Dynamic dispatch prevents inlining because the method is resolved via a vtable
//     at runtime, making the exact function unknown until execution.
//
// (** What is `dyn`?)
//     The `dyn` keyword in Rust is used to indicate a trait object that uses dynamic dispatch.
//     When you write `&dyn Trait` or `Box<dyn Trait>`, you're telling the compiler that
//     the specific type that implements `Trait` will only be determined at runtime.
//     This enables polymorphism, but also comes with runtime overhead due to the vtable lookup.
//
// (*** Quick summary of Static and Dynamic Dispatch)
//     - **Static Dispatch**: The method or function to be called is determined at compile time.
//       The compiler knows the exact type and can optimize the code with techniques like inlining.
//     - **Dynamic Dispatch**: The method or function to be called is determined at runtime.
//       The type is resolved via a vtable (virtual table), which incurs some runtime overhead.
//
// (**** What is `Box`?)
//     `Box<T>` is a smart pointer in Rust that allocates data on the heap instead of the stack.
//     When used with trait objects (`Box<dyn Trait>`), it allows for storing types of unknown size
//     at compile time (like trait objects), while providing ownership and heap allocation.
//     It’s commonly used for dynamic dispatch when you need to return or store different types
//     that implement the same trait, but their sizes are not known at compile time.
