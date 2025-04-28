// # Lifetimes
//
// Another kind of generic ensuring that references are valid as long as needed.
// Every reference has a lifetime, which is the scope for which that reference is valid.
// Most of the time implicit and inferred, don't need to worry.
// Sometimes lifetime annotations are needed, if the compiler can't infer it.
// Lifetime annotations is a concept which most other programming languages don't have.
//
// Main aim of lifetimes is to prevent dangling reference (also called dangling pointers).
//
// Outer sscore declares variable `r` with no initial value. Inner scope declares variable `x` with
// initial value of 5. Then a reference of `x` is assigned to `r`.
//
// `x` the goes out of scope and `r` refers to has gone out of scope.
fn example_dangling_reference_01() {
    // let r;

    {
        let x = 5;
        // r = &x; // `x` does not live long enough.
    }

    // println!("r: {}", r);
}

// # Borrow Checker
//
// Borrow checker compares scopes to determine wheter all borrows are valid.
// Key part of Rust's ownership system.
// Tracks lifetimes of references and ensures that they don't violate the ownership rules.
// Rules ensure that a value is not accessed after it has been moved or freed from memory.
// Important: A reference to a value must never outlive the value itself!

// `i` has the longest lifetime because its scope entirely encloses both
// `borrow1` and `borrow2`. The duration of `borrow1` compared to `borrow2`
// is irrelevnt since they are disjoint.
pub fn example_lifetimes_01() {
    let i = 3;

    {
        let borrow1 = &i; // `borrow1` lifetimes starts
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends.
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }
}

// # Lifetime Annotating
//
// The borrow checker uses explicit lifetime annotations to determine how long a reference should be valid.
// But for us users, in most cases, there is no need to annotate the lifetime, because there are several elision rules, before
// learning these rules, we need to know how to annotate lifetime manually.
//
// ## Function
//
// Ignoring elision rules, lifetimes in function signatures have a few constraints:
// - Any reference must have an annotated lifetime.
// - Any reference being returned must have the same lifetime as one of the inputs or be static.
//
fn example_lifetime_annotating_01() {
    // One input reference with lifetime `'a` which must live
    // at least as long as the function.
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    // Mutable references are possible with lifetimes as well.
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    // Multiple elements with different lifetinmes. In this case, it
    // would be fine for both to have the same lifetime `'a`, but
    // in more complex cases, different lifetimes may be required.
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    // Retuning references that have been passed in is acceptable.
    // However, the correct lifetime must be returned.
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn example_lifetime_annotation_02() {
    let x = "long";
    let y = "longer";

    let result = longest(x, y);
    println!("{}", result);
}

// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving a
// reference to invalid data to be returned.
// pub fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

pub fn valid_output() -> &'static str {
    "foo"
}

pub fn example_lifetime_annotating_03() {
    let x = valid_output();
    println!("{}", x);
}

struct Owner(i32);
impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub fn example_lifetime_annotating_04() {
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();
}

struct ImportantExcerpt {
    part: &'static str,
}

impl ImportantExcerpt {
    fn level(&self) -> i32 {
        3
    }
}

// # Elision
//
// Some lifetime patterns are so common that the borrow checker will allow you to omit them to
// save typing and to improve readability.
// This is known as *Elision*. Elision exist in Rust only because these patterns are common.
//
// Three Rules of Lifetime Elision
// The compiler uses three rules to figure out lifetimes of references that aren't explicit annotation.
// 1. Compiler assigns a lifetime parameter to each parameter that's a reference.
// 2. If there is exactly one input lifetime parameter that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple lifetime parameters but one of the is `&self` or `&mut` self the lifetime of self is
// assigned to all output lifetime parameters.
//
// Examples
fn _first_word(s: &str) -> &str {
    s
}

// Compiler applies first rule: each parameter gets it own lifetime.
// Second rule applies because there is exactly one input lifetime, so the lifetime of the one input
// parameter gets assigned to the output lifetime.
fn _first_word_2<'a>(s: &'a str) -> &str {
    s
}

// In this case, the compiler could infer the lifetime and we don't have to specify them manually.
fn _first_word_3<'a>(s: &'a str) -> &'a str {
    s
}

// fn longest_1(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// First rule: each parameter gets its own lifetime
// Here the second rule doens't apply because there is more than one input lifetime. Also,
// third rule doesn't apply because this function is not a method.
// fn longest_2<'a, 'b>(x: &'a str, y: &'b str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// We have to manually annotate the lifetime parameters.
fn _longest_3<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct _ImportantExcerpt1<'a> {
    pub part: &'a str,
}

// Here the third rule applies, which states that if there is a reference to self (&self)
// the all references will have the same lifetime as &self.
impl<'a> _ImportantExcerpt1<'a> {
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Elision Examples
/* Remove all the lifetimes that can be elided */
fn _input<'a>(x: &i32) {
    println!("`annotaded_input`: {}", x);
}

fn _input_elided(x: &i32) {
    println!("`annotaded_input`: {}", x);
}

fn _pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn _pass_elided(x: &i32) -> &i32 {
    x
}

// nothing to elid
fn _logest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct _Owner1(i32);

impl _Owner1 {
    fn _add_one<'a>(&'a mut self) {
        self.0 += 1
    }

    fn _add_one_elided(&mut self) {
        self.0 += 1
    }

    fn _print<'a>(&'a self) {
        println!("`print`: {}", self.0)
    }

    fn _print_elided(&self) {
        println!("`print`: {}", self.0)
    }
}

// nothing to elid
struct _Person<'a> {
    age: u8,
    name: &'a str,
}

// other approach
struct _Person1 {
    age: u8,
    name: &'static str,
}

// nothing to elid
enum _Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// ## Static Lifetime
//
// Refers to a lifetime that lasts for the entire duration of the program's execution.
// Any reference or borrowed value with static lifetime can be safetly used throughthout the program.
// Can be coerced to a shorter lifetime if needed.

// String literals have a static lifetime because they are hardcoded into the
// executable meaning they are valid throghout the entire duration of the program's execution.
pub fn _example_static_lifetime_01() {
    let _s: &'static str = "Hello, world!";
}

// 'static as part of trait bound
fn _generic<T>(_x: T)
where
    T: 'static,
{
}

// ### &'static
//
// As a reference lifetime, &'static indicates the data pointed to by the reference lives as long as the running program.
// But it can still be coerced to a shorter lifetime.
// 1. There are several ways to make a variable with 'static lifetime, two of them are stored in the read-only memory of the binary.

pub fn _example_static_lifetime_02() {
    // Every string literal is hardcoded into the binary.
    let option_01 = "hello";
    let option_02: &'static str = "hello";

    // both solutions will work.
    _need_static(option_01);
    _need_static(option_02);

    println!("Success!");
}

fn _need_static(r: &'static str) {
    assert_eq!(r, "hello");
}

// 2. Another way to make 'static lifetime is using `Box::leak`
#[derive(Debug)]
struct _Config {
    a: String,
    b: String,
}

static mut config: Option<&mut _Config> = None;

// fn init() -> Option<&'static mut _Config> {
//     Some(& mut _Config) {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     }
// }

// 3. &'static only indicates that the data can live forever, not the reference. The latter
// one will be constrained by its scope.
pub fn _example_static_lifetime_03() {
    let static_string: &'static str = "I'm in read-only memory";

    {
        // Make a `string` literal and print it:
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    println!("static_string reference remains alive: {}", static_string);
}

// 4. &'static can be coerced to a shorter lifetime.
// Make a constant with 'static lifetime.
static NUM: i32 = 18; // static will always remaining at the same memory location.
const NUM_CONST: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    // 'static > 'a -> 'static lives longer than 'a
    &NUM
}

// T: 'static
//
// As a trait bound, it means the type does not contain any non-static references. Eg. the receiver
// can hold on to the type for as long as they want and it will never become invalid until they drop it.
//
// It's important to understand this means that any owned data always passes a 'static lifetime bound,
// but a reference to that owned data generally does not.
use std::fmt::Debug;

fn _print_it<T: Debug + 'static>(input: T) {
    print!("'static value passed in is: {:?}", input);
}

fn _print_it1(input: impl Debug + 'static) {
    print!("'static value passed in is: {:?}", input);
}

fn _print_it2<T: Debug + 'static>(input: &T) {
    print!("'static value passed in is: {:?}", input);
}

pub fn _example_static_lifetime_04() {
    // `i` is owned and contains no references, thus it's 'static.
    let i = 5;
    const b: i32 = 10;
    static c: i32 = 10;
    _print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not static:
    // _print_it(&i); // this will trigger compiler error -> `i` does not live long enough.
    // _print_it1(&i); // this will trigger compiler error -> `i` does not live long enough.
    _print_it(&b);
    _print_it(&c);
    _print_it1(&b);
    _print_it1(&c);

    // but this one works!
    _print_it2(&i);
}
