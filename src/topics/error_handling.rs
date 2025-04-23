// Panic!
// Simplest form of *error handling* is to use the `panic!` macro.
// `panic!` will print out an *error message*, *unwind* the *stack* and finally *exit* the program.
// In multithreaded programs it will *exit* the thread in which the `panic!` occurs, not the whole program.
