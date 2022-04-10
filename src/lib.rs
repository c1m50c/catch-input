use std::io::{self, Write};
use std::ops::FnOnce;


/// Returns a trimmed [`String`] containing the user input from standard input.
/// You may pass a function that is meant to work as the input prompt.
/// 
/// # Example
/// ```rs
/// let inp = get_input_with(|| { print!("Input => "); });
/// assert!(inp, String::from("Hello, World!"));
/// ```
/// The application input would like this...
/// ```txt
/// Input => Hello, World!
/// ```
#[inline]
pub fn get_input_with<F: FnOnce()>(func: F) -> String {
    let mut result = String::new();
    func();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result)
        .expect("Failed to read line from Standard Input.");
    
    return result.trim().to_string()
}


/// Macro that returns a trimmed [`String`] containing the user input from standard input.
/// You may pass an argument to be used as a prompt, you may use a [`FnOnce`] or a type with the [`Display`] trait implemented.
/// 
/// # Example
/// ```rs
/// let input_a = input!("InputA => ");
/// let input_b = input!(|| { print!("InputB => "); });
/// 
/// assert!(input_a, String::from("Hello"))
/// assert!(input_b, String::from("World"))
/// ```
/// The application input would like this...
/// ```txt
/// InputA => Hello
/// InputB => World
/// ```
#[macro_export]
macro_rules! input {
    ($x:tt) => { $crate::get_input_with(|| { print!("{}", $x); }) };
    ($x:expr) => { $crate::get_input_with($x) };
}