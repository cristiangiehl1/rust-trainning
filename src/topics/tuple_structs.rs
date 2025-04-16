// Like normal structs but using tuple-like syntax for defining the fields.
// Basicaly a named tuple.
// Instantiated by parathesis instead of curly braces.
// Accessed through dot notation and index.

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
}
