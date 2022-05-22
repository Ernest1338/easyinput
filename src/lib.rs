//! # easyinput
//!
//! Dependency-less library providing an easy abstraction for getting user input
//!
//! # Usage
//!
//! Basic usage
//!
//! ```rust
//! use easyinput::input;
//!
//! fn main() {
//!     let user_input = input("What is your name? ");
//!     println!("Hello, {}!", user_input);
//! }
//! ```
//!
//! # LICENSE
//!
//! This project is distributed under MIT license.

use std::io::{stdin, stdout, Write};

/// Abstraction over stdin().read_line() providing
/// easy way of getting input from the user
///
/// # Examples
///
/// ```no_run
/// use easyinput::input;
///
/// let user_input = input("What is your name? ");
/// println!("Hello, {}!", user_input);
/// ```
pub fn input(message: &str) -> String {
    let mut user_input: String = String::new();
    print!("{message}");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut user_input)
        .expect("ERROR: Unable to read user input");
    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }
    user_input
}
