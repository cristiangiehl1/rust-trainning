use std::{collections::HashMap, hash::Hash};

// Data structure to store key-value pairs.
// Allocated on the heap as it is dynamicall sized, can grow and shrink.
// Allows for efficient lookup, insertion and deletion of data.
// Each key is hashed to a unique index in underlying array.
// All of the keys must be the same type and all of the values must be the same type as well.
//
// Where Vectors store values by an integer index, HashMap store values by key. It is a hash map implemented with quadratic probing and SIMD lookup.
// By default, `HashMap` uses a hasing algorithm selected to provide resistance against HashDoS attacks.
//
// The default hashing algorithm is currentlyt `SipHash 1-3`, though this is subject to change at any point in the future. While its performance
// is very competitive for medium sized keys, other hashing algorithms will outperform it for small keys such as integers as well as large
// keys such as long strings, though those algorithms will typically not protect against attack such as HashDoS.
//
// The hash table implementation is a Rust port of Google's SwissTable.
pub fn hashmap_exercise_1() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}

pub fn hashmap_exercise_2() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // This will directlly convert an array of tuples into a HashMap, only possible if it's an array of tuples.
    let teams_map2 = HashMap::from(teams.clone());

    // Other solution
    let teams_map2_2: HashMap<&str, i32> = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);
    assert_eq!(teams_map2, teams_map2_2);
}

fn random_stat_buff() -> u8 {
    42
}

pub fn hashmap_exercise_3() {
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // Insert a key using a function that provides a new value only if it
    // doens't already exist
    player_stats
        .entry("health")
        .or_insert_with(random_stat_buff);

    // Ensures a value is in the entry by inserting the default if empty, and return
    // a mutable reference to the value in the entry.
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");
}

// Requirrements of HashMap key
//
// Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`. This includes:
// - `bool` (though not very useful since there is only two possible keys)
// - `int`, `uint`, and all variations thereof
// - `String` and `&str` (tips: you can have a `HashMap` keyed by `String` and call .get() with an `&str`)
//
// Note that `f32` and `f64` do not implement `Hash` likely because floating-point precision errors would make using the as hashmap keys horrible error-prone.
//
// All collection classses implement `Eq` and `Hash` if their contained type also respectively implements `Eq` and `Hash`. For example, `Vec<T>`
// will implement `Hash` if `T` implements `Hash`.
#[derive(Debug, Eq, PartialEq, Hash)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

pub fn hashmap_excercise_4() {
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

// Ownership
//
// For types that implement the `Copy` trait, like `i32`, the values are copied into `HashMap`. For owned values like `String`, the values will be
// moved and `HashMap` will be the owner of those values.
pub fn hashmap_excercise_5() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap: {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    m2.insert(&v2, v1);

    assert_eq!(v2, "hello");

    println!("Success!");
}

// Third-party hash libs
//
// If the performance of `SipHash 1-3` doesn't meet your requirements, you can find replacements in crates.io or github.com.
//
// The usage of third-party hash looks like this:
