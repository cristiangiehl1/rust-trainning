// SMART POINTER that allows to store data on the heap rather than the stack.
// Use Box when you have a type whose size can't be known at compile time.
// Returns a pointer to the data stored on the heap.

// ================================================
// & (Reference) vs Box (Heap Allocation)
// ================================================
//
// Feature                      | & (Reference)                    | Box (Box<T>)
// -------------------------------------------------------------------------------------------
// Ownership                    | Borrowed (non-owning)            | Owning (moves data to the heap and has the ownership)
// Memory location              | Stack                            | Heap
// Lifetimes                    | Limited                          | Can be passed across scopes
// Mutability control           | Borrowed mutability              | Owns and can mutate if Box is mutable
// Size known at compile time?  | Yes                              | Yes (Box itself is pointer-sized)
// Dereferencing                | *ref                             | *box or auto-deref
// Use with trait objects       | Yes: `&dyn Trait`                | Yes: `Box<dyn Trait>` (more flexible)
// Common use case              | Temporary access, borrowing      | Ownership of heap-allocated values
// Cost                         | Zero-cost (stack only)           | Heap allocation/deallocation (slower)
// Multiple references allowed  | Yes (`&` and `&mut`)             | Only one owner (unless wrapped in Rc/Arc)
// Cloneable?                   | No                               | Cloning Box clones the heap value (if value implements Clone)
//
// (*** Summary ***)
// - **& (Reference)**: Used for borrowing data without taking ownership. Efficient and fast since data remains on the stack.
//   Requires lifetimes and does not allocate memory on the heap.
//
// - **Box**: Used to allocate data on the heap and take ownership. Especially useful when working with trait objects or large structs
//   where you want fixed-size, heap-stored values. Useful for polymorphism (`Box<dyn Trait>`) and recursive types.
//
// Example:
//
//     fn print_name(name: &String) {}         // Borrowing with &
//
//     let boxed = Box::new(42);               // Heap-allocated value
//     let animal: Box<dyn Animal> = Box::new(Dog {});
