// Option is an enum that represents a value that may or may not be present.
// Known in other languages as "null", refering to the absence of a value.
// Used to handle cases where a function or method might fail to return a value.

enum Option<T> {
    Some(T), // Represents a value that is present
    None,    // Represents a value that is absent
}
