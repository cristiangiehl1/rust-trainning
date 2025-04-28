// # Iterators
//
// - Allows to perform task on sequence of items in turn.
// - Iterators are lazy, meaning no effect until methods are called that consume the iterator to use it up.
// - All iterators implement trait Iterator which has `next()`` method, which gets called automatically when
// traversing over some data.
// - Some methods consume iterator while others produce a new iterator from the provided iterator.
pub fn _example_iterators_01() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{}", x);
    }
}

// In the code above, you may consider `for` as as simple loop, but actuallty it is iterating over a iterator.
// By default `for` will apply the `into_iter` to the collection, and change it into a iterator. As a result,
// the following code is equivalent to previous one:
pub fn _example_iterators_01_1() {
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}", x);
    }
}

pub fn _example_iterators_01_2() {
    let arr = [0; 10];
    for x in arr {
        println!("{}", x);
    }
}

pub fn _example_iterators_01_3() {
    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

// ## next method
//
// All iterators implement a trait named `Iterator` that is defined in the standard library:
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// And we can call the `next` method on iterators directly.
pub fn _example_iterators_02() {
    // When we put a collection into a iterator it should be mutable.
    let mut v1 = vec![1, 2].into_iter();

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}

// ## into_iter, iter and iter_mut
//
// In the previous section, we have mentioned that `for` will apply the `into_iter` to the collection, and change it into a iterator.
// However, this is not the only way to convert collections into iterators.
//
// `into_iter`, `iter`, `iter_mut`, all of them can convert into iterator, but in different ways.
//
// - `into_iter` -> consumes the collection, once the collection has been consumed, it is no longer available for reuse,
// because its ownership has been moved within the loop.
// - `iter` -> this borrows each element of the collection through each iteration, thus leaving the collection unthouced and
// available for reuse after the loop.
// - `iter_mut` -> this mutably borrows each element of the collection, allowing for the collectionm to be modified in place.
pub fn _example_iterators_03() {
    let v = vec![0; 10];

    // consume v
    for i in v.into_iter() {
        println!("{}", i);
    }

    // can't use it because it was moved.
    // println!("{:?}", v);

    let v_2 = vec![1; 10];

    // don't take ownership only borrowing the values.
    for i in v_2.iter() {
        println!("{}", i);
    }

    // we can call it again here.
    println!("{:?}", v_2);
}

pub fn _example_iterators_04() {
    let mut names = vec!["Cristian", "Joao Pedro", "Michelle"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Michelle" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

pub fn _example_iterators_05() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next() {
        *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}

// ## Creating our own iterator
//
// We can not only create iterators from collection's types, but also can create iterators by
// implementing the `Iterator` trait on our own types.
struct _Counter {
    count: u32,
}

impl _Counter {
    fn _new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for _Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn _example_iterators_06() {
    let mut counter = _Counter::_new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[derive(Debug)]
struct _Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `_Fibonnaci`
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for _Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let forward = self.curr + self.next;

        self.curr = self.next;
        self.next = forward;

        Some(self.curr)
    }
}

fn _fibonnaci() -> _Fibonacci {
    _Fibonacci { curr: 0, next: 1 }
}

pub fn _example_iterators_07() {
    let mut fib = _fibonnaci();

    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));

    println!("{:?}", fib);
}

// ## Methods that Consume the Iterator
//
// The `Iterator` trait has a number of methods with default implementations provided by the standard library.
//
// ### Consuming adaptors
//
// Some of these methods call the method `next` to use up the iterator, so they are called consuming adaptors.
pub fn _example_iterators_08() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method.
    let total: i32 = v1_iter.sum();

    println!("Sum: {}", total);
    assert_eq!(total, 6);

    // The `sum` method will take ownership of the Iterator, so we cannot use it anymore.
    // println!("{:?}, {:?}", v1, v1_iter);
    println!("{:?}", v1);
}

// ## Collect
//
// Other than converting a collection into a iterator, we can also `collect` the result values into
// a collection, `collect` will consume the iterator.
use std::collections::HashMap;

pub fn _example_iterators_09() {
    let names = [("sunface", 18), ("sunfei", 18)];
    let folks: HashMap<&str, i32> = names.into_iter().collect();

    println!("{:?}", folks);

    let v1 = vec![1, 2, 3];

    // iter only take references to the elements inside the Vec.
    let _v2_2: Vec<&i32> = v1.iter().collect();
    let v2: Vec<i32> = v1.into_iter().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}

// ## Iterators adaptors
//
// Methods allowing you to change one iterator into another iterator are known as iterator adaptors.
// You can chain multiple iterator adaptors to perform complex actions in a readable way.
//
// But because all iterators are lazy, you have to call one of the consuming adapters to get results from
// calls to iteretor adapters.
pub fn _example_iterators_10() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|e| e + 1).collect();

    println!("{:?}", v1);
    println!("{:?}", v2);

    assert_eq!(v2, vec![2, 3, 4])
}
