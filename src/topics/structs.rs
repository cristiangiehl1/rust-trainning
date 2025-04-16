// Compound type allowing to group together values of different types into a named data structure.
// Similar to tuples. but each value has a name so values can be accessed through this name.
// Have to be instanciated with data, think of it like the struct is the template for the instances you create from it.

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn struct_test() {
    let mut user_1 = User {
        active: true,
        username: String::from("user_1"),
        email: String::from("user_1@domain.com"),
        sign_in_count: 1,
    };

    // Accessing and Mutating Struct Fields
    // - We can access and mutate single fields of strucs:
    user_1.email = String::from("another_email@domain.com")
}

pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub fn struct_update_syntax() {
    let user_1 = User {
        active: true,
        username: String::from("user_1"),
        email: String::from("user_1@domain.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("anotheremail@domain.com"),
        ..user_1 // This will copy all the fields from user_1 to user2
    };

    println!("user2: {:?}", user2);
}

// Partial Move
// - Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at
/// the same time. Doing this will result in a partial move of the variable, which means that parts of the variable
/// will be moved while others parts stay. In such a case, the parent variable cannot be used afterwards as a whole,
/// however, the parts that are only referenced (and not moved) can still be used.

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

pub fn struct_partial_move() {
    let f = File {
        name: String::from("file.txt"),
        data: String::from("Hello World!"),
    };

    let name = f.name; // Moved ownership of f.name to name

    // println!("{}, {}, {:?}", f.data, f.name, f); // Error: f.name was moved")
    println!("{}, {}", f.data, name);
}
