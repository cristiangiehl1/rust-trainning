// Normal flow of a program: Top to bottom, line by line.
// Concept that refers to ability to control the order in which statements or instructions are executed in a program.
// Allows to specify which instructions should be executed under which conditions and in what order.

// Conditionals:
// - if / else
// - match

// Loops:
// - for / while / loop
// - break / continue

pub fn for_loop_example() {
    let a = [4, 3, 2, 1];

    // enumerate returns an iterator that yields pairs of (index, value)
    for (i, v) in a.iter().enumerate() {
        println!("a[{}] = {}", i, v);
    }
}

pub fn while_loop_example() {
    let mut n = 1;

    while n < 10 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

pub fn continue_break_loop_example() {
    let mut n1 = 0;
    let mut n2 = 0;

    for _i in 0..=100 {
        if n1 == 66 {
            break;
        }
        n1 += 1;
    }

    for _i in 0..=100 {
        if n2 != 66 {
            n2 += 1;
            continue;
        }
        break;
    }

    assert_eq!(n1, 66);
    assert_eq!(n2, 66);

    println!("n = {}", n1);
    println!("n = {}", n2);
}

pub fn loop_example() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // infinite loop
    // You have to manually break out of it by using a break statement
    let result = loop {
        count += 1;

        if count == 3 {
            println!("Three!");
        }

        if count == 5 {
            println!("Five!");
        }

        // Loop is an expression, so we could use break to return a value
        if count == 10 {
            println!("Ten!");
            break count * 2;
        }
    };

    println!("End count: {}", count);
    println!("Result: {}", result);
}

// It's possible to break or continue outer loops when dealing with nested loops
/// in these cases, the loops must be annotated with some 'label, and the label must be
/// provided to the break/continue statement.

pub fn nested_loops_example() {
    let mut count = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        '_inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }
}
