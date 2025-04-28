// # Closures
//
// Anonymous functions that are able to capture values from the scope in which they are defined.
// Can be defined inline (for example as functions parameter).
// Don't require type annotations.
// Can take ownership of a value by using `move` keyword.
//
// ## Fn Traits
//
// - Trait that defines signature for closures/functions.
// - Describes types, number of arguments and return type.
// - Three different traits:
//
// 1. Fn Once
// -- Closure that can be called once
// -- Takes ownership of capture values
//
// 2. FnMut
// -- Might mutate captured values.
// -- Can be called more than once.
//
// 3. Fn
// -- Doesn't take ownership of captured values.
// -- Doesn't mutate anything.
// -- Might not even capture anything from its environment.
//
// Closure Example
pub fn _example_closures_01() {
    let x = 1;

    // This closure captures the value of x and modifies it.
    // Compiler will capture variables in the least restrictive manner possible.
    // In this case a mutable reference of `x` is taken, rather than taking ownership because it's less restrictive.
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);
}

// From the syntax, we can see that closures are very convenient for on the fly usage. Unlike
// functions, both the input and return types of a closure can be inferred by the compiler.
pub fn _example_closures_02() {
    // Increment via closures and functions
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references.
    //
    // These nameless functions are assigned to appropriately named variables.
    let closure_inferred = |i| i + 1;
    let i = 1;
    println!("Closure Inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

// ## Capturing
//
// Closures can capture variables by borrowing or moving. But they prefer to caputure by borrowing and only go lower when required:
// - By reference: `&T`
// - By mutable reference: `&mut T`
// - By value: `T`

pub fn _example_closure_capturing_01() {
    let color = String::from("green");

    // `move` keyword will make the caputure values move to inside the closure.
    // That means the closure will become the owner of that captured value.
    // let print = move || println!("`color`: {}", color); // comment that line to correct compiler error.
    let print = || println!("`color`: {}", color); // removed `move` keyword will cause the closure to take a immutable reference of the value.

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color; // it wont work because we moved the value to the closure.
    println!("{}", color);
}

pub fn _example_closure_capturing_02() {
    let mut count = 0;

    // Where because we are accessing a `i32` type it will copy the value, because is less expensive to the program than move the ownership.
    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error.
    let _count_reborrowed = &mut count;

    println!("Count: {}", count);
    assert_eq!(count, 0);
}

pub fn _example_closure_capturing_03() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        _take(&movable);
    };

    consume();
    consume();
}

fn _take<T>(_v: &T) {}

pub fn _example_closures_03() {
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));

    // We cannot call the closure again with another type.
    let _n = example_closure(5.to_string());
}

fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

pub fn _example_closures_04() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
}

pub fn _example_closures_05() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    _exec(update_string);

    println!("{:?}", s);
}

fn _exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

// Which trait does the compiler prefer to use?
//
// - Fn: the closure uses the captured value by reference (&T)
// - FnMut: the closure uses the captured value by mutable reference (&mut T)
// - FnOnce: the closure uses the captured value by value (T)
//
// On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.
//
// For instance, consider a paramter annotated as FnOnce. This specifies that the closure may
// capture by `&T`, `&mut T` or `T`, but the compiler will ultimately choose based on how the
// captuired variables are used in the closure. Which trait to use is determined by what the closure
// does with captured value.
//
// This is because if a move is possible, then any type of borrow also be possible. Note
// that the reverse is not true. If the parameter is annotated as `Fn`, then capturing variables by
// `&mut T` or `T` are not allowed.
//
// A function which takes a closuire and returns as `i32`
pub fn _apply<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn _example_closures_06() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one.
    let mut farewell = "goodbye".to_string();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forcer `farewell` to be captured by mutable reference.
        // Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    diary();

    let double = |x| 2 * x;
    println!("3 doubled: {}", _apply_to_3(double))
}

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter".
fn _apply_1<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// A function which takes a closure and returns an `i32`,
fn _apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

// Move closures may still implement `Fn` or `FnMut`, even though they capture variables by move.
// This is because the traits implemented by a closure type are determined by what the closure
// does with captured values, not how it captures them. The `move` keyword only specifies the latter.
pub fn _example_closures_07() {
    let mut s = String::new();

    let update_string = |str| -> String {
        s.push_str(str);
        s
    };

    exec(update_string);

    // We have to declare `FnOnce` because we want to return the value and to do that we need to take the ownership of it.
    fn exec<'a, F: FnOnce(&'a str) -> String>(mut _f: F) {
        _f("hello");
    }
}

// Since closure can be used as arguments, you might wonder can we use functions as arguments too?
// - And indeed we can.
fn _call_me<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn _function() {
    println!("I'm a function!");
}

pub fn _example_closures_08() {
    let closure = || println!("I'm a closure!");

    _call_me(closure);
    _call_me(_function);
}

// ## Closure as return types
//
// Returning a closure is much harder than you may have thouught of.
fn _create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    move |x| x + num
}

fn _create_fn_dyn(choice: bool) -> Box<dyn Fn(i32) -> i32> {
    let num1 = 10;
    let num2 = 20;

    if choice {
        Box::new(move |x| x + num1)
    } else {
        Box::new(move |x| x + num2)
    }
}

pub fn _example_closures_09() {
    let fn_plain = _create_fn();
    fn_plain(1);

    let fn_plain_dyn = _create_fn_dyn(true);
}

pub fn _factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}
