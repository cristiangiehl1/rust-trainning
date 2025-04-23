// Trait that can be automatically implemented for a struct or an enum by the Rust compiler.
// Called "derivable" because they can be derived automatically.
// Most commom derivable traits are:
// - Debug: allowing to output content via "{:?}"
// - Clone: enables type to be duplicated with "clone()" method
// - Copy: enables type to be copied implicity, without requiring "clone()"
// - PartialEq: enables comparison
