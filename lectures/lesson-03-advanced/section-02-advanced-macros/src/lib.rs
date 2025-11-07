//! Section: Advanced Declarative Macros
//!
//! Implement two macros: `vec_of_strings!` and `make_map!`.

/// Build a `Vec<String>` from comma-separated expressions that implement `ToString`.
/// Usage: `vec_of_strings!("a", 1, String::from("x"))` => `vec!["a", "1", "x"]`
#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),* $(,)?) => {
        {
            // TODO: expand to a Vec<String> using `to_string()` on each expr
            let v: ::std::vec::Vec<::std::string::String> = ::std::vec![];
            v
        }
    };
}

/// Build a `HashMap<String, i32>` from `key => value` pairs.
/// Keys are converted with `to_string()`.
#[macro_export]
macro_rules! make_map {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {
        {
            // TODO: construct and return a HashMap<String, i32>
            let m: ::std::collections::HashMap<::std::string::String, i32> = ::std::collections::HashMap::new();
            m
        }
    };
}

// Re-export std items used by tests without forcing students to import them in macros scope.
pub use std::collections::HashMap;
