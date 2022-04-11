//! Library giving an easy way to fetch input from the standard input through the exported [`input`] macro.
//! 
//! # Example
//! ```rust
//! use catch_input::input;
//! 
//! fn main() {
//!     let input_a = input!("What's your favorite food? ");
//!     let input_b = input!(|| { print!("What's your favorite drink? ") });
//!     
//!     assert!(input_a, String::from("Oatmeal"));
//!     assert!(input_b, String::from("Water"));
//!     
//!     // $ cargo run
//!     // What's your favorite food? Oatmeal
//!     // What's your favorite drink? Water
//! }
//! ```

use std::io::{self, Write};
use std::ops::FnOnce;


/// Returns a trimmed [`String`] containing the user input from standard input.
/// You may pass a function that is meant to work as the input prompt.
/// 
/// ### **This is meant for implementation in the [`input`] macro, not for external use.**
/// 
/// # Example
/// ```rust
/// let inp = _get_input_with(|| { print!("Input => "); });
/// assert!(inp, String::from("Hello, World!"));
/// ```
/// The application input would like this...
/// ```txt
/// Input => Hello, World!
/// ```
#[inline]
#[doc(hidden)]
pub fn _get_input_with<F: FnOnce()>(prompt: F) -> String {
    let mut result = String::new(); prompt();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result)
        .expect("Failed to read line from Standard Input.");
    
    return result.trim().to_string()
}


/// Macro that returns a trimmed [`String`] containing the user input from standard input.
/// You may pass an argument to be used as a prompt, you may use a [`FnOnce`] or a type with the [`Display`] trait implemented.
/// 
/// # Example
/// ```rust
/// let input_a = input!("InputA => ");
/// let input_b = input!(|| { print!("InputB => "); });
/// 
/// assert!(input_a, String::from("Hello"))
/// assert!(input_b, String::from("World"))
/// 
/// /*
/// $ cargo run
/// InputA => Hello
/// InputB => World
/// */
/// ```
#[macro_export]
macro_rules! input {
    ($x:tt) => { $crate::_get_input_with(|| { print!("{}", $x); }) };
    ($x:expr) => { $crate::_get_input_with($x) };
}