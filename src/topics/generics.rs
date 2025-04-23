// Generics:
//- Placeholders for concrete types.
//- Enables writing more reusable and flexible code.
//- Avoids having duplicate code for different types.
//- Zero cost abstraction, Rust compiler will at compile time fill out generics with concrete types.

// Const Generics:
//- Type parameter that represents a compilte-time constant value.
//- Allows to write generic code that operates on values that are known at compile time.
//- Used for array sizes, bit widths and other compile-time constants.

struct A; // Concrete type `A`
struct S(A); // Concrete type `S` that contains a value of type `A`
struct SGen<T>(T); // Generic type `SGen` that can contain any type `T`

fn reg_fn(_s: S) {} // non-generic function that takes a concrete type `S`
fn gen_spec_t(_s: SGen<A>) {} // non-generic function that takes implicitly specified type `A`
fn gen_spec_i32(_s: SGen<i32>) {} //non-generic function that takes implicitly specified type `i32`
fn generic<T>(_s: SGen<T>) {} // Generic function that takes a generic type `SGen<T>`

pub fn test_generics() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(5));

    generic::<char>(SGen('a')); // Explicitly specifying the type `char`
    generic(SGen('Z'));
    generic(SGen(7.7));

    println!("Generics and Const Generics in Rust");
}

// A function call with explicit type specified type parameters looks like: fun::<A, B, C, ...>(args).
pub fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub struct Point<T> {
    x: T,
    y: T,
}

// let integer_point = Point { x: 5, y: 10 };
// let float_point = Point { x: 5.0, y: 10.0 };

#[derive(Clone, Debug)]
pub struct Point2<T, U> {
    pub x: T,
    pub y: U,
}

impl<T: Clone, U: Clone> Point2<T, U> {
    pub fn mixup<V, W>(self, p2: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, y: p2.y }
    }

    pub fn mixup_associated<V: Clone, W: Clone>(
        p1: &Point2<T, U>,
        p2: &Point2<V, W>,
    ) -> Point2<T, W> {
        Point2 {
            x: p1.x.clone(),
            y: p2.y.clone(),
        }
    }
}

// let p = Point2 { x: 5, y: 10.0 }; // T is i32, U is f64

pub struct Val<T> {
    pub val: T,
}

impl<T> Val<T> {
    pub fn value(&self) -> &T {
        &self.val
    }
}

// CONST GEMERICS
#[derive(Debug)]
pub struct Array<T, const N: usize> {
    pub data: [T; N],
}
