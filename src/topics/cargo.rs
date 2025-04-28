// Official package manager and building tool.
// Helps automate tasks such as creating new projects, building, running, testing code and managing dependencies.
// Crate is a compilation unit of Rust source code.
// Crates.io repository for Rust packages.
//
// # Crates
//
// A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of the crate.
//
// ## Binary:
// - Compiled into an executable binary
// - Basically a "program"
//
// ## Libray:
// - Compiled into a library.
// - Reusable code that can be shared across multiple projects.
//
// Crate root:
// - Source file that is the root module of the crate
// - In binaries: src/main.rs
// - In libraries: src/lib.rs
//
// # Modules
//
// Way of organizing code by grouping related items together.
// Can be imported using namespaces avoinding naming collisions.
// Also controls privacy of its items like functions. structs, enums, etc.
// When compiling the compiler starts from the crate root, then checks if modules are declared and looks for submodules.
// Submodules could be directly written inline within curly braces, in a file which has the module name ending in .rs in a directory
// which has the name of the module and a mod.rs file inside it.

// See rust-trainning-cargo
