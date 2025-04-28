// Types which want to be printable Debug or Display traits must be implemented.
// Automatic implementationms are only provided for types in the standard library.
// Debug trait can be implemented simply by using derivable trait.
// Display trait must be manually implemented.
// `std::fmt::Debug` or `std::fmt:Display`

use std::path::Display;

use crate::topics::generics::{Point, Point2};

pub fn example_format_01() {
    let s1 = "hello";
    let s = format!("{}, world!", s1);

    println!("{}", s);
    assert_eq!(s, "hello world!");
}

pub fn example_print_01() {
    print!("hello world, ");
    println!("I am");
    println!("Sunface!")
}

pub fn example_debug_01() {
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);
}

pub fn example_debug_02() {
    #[derive(Debug)]
    struct Structure(i32);

    println!("{} months in a year.", 12);

    println!("Now {:?} will print!", Structure(12));
}

pub fn example_debug_03() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    let person = Person {
        name: "Sunface".to_string(),
        age: 20,
    };

    /*
    using the sintaxe below will make the output much more pretty
      1)  Prettier:
       Person {
           name: "Sunface",
           age: 18,
       }

       2) Instead of the default output:
       Person { name: "Sunface", age: 20 }
    */
    println!("{:#?}", person);
}

pub fn example_debug_04() {
    use std::fmt;

    struct Structure(i32);
    struct Deep(Structure);

    impl std::fmt::Debug for Deep {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0 .0)
        }
    }

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)))
}

pub fn example_display_01() {
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl std::fmt::Display for Point2D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Display: {} + {}i", self.x, self.y)
        }
    }

    impl std::fmt::Debug for Point2D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(
        format!("{:?}", point),
        "Debug: Complex { real: 3.3, imag: 7.2 }"
    );

    println!("{}", point);
    println!("{:?}", point);
}

pub fn example_display_02() {
    struct List(Vec<i32>);

    impl std::fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let vec = &self.0;

            write!(f, "[");

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
}
