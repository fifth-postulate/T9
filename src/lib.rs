#![deny(missing_docs)]
//! This library allows you to easily go from words to port numbers and back again.
//!
//! For example, let's say you want to have a Rust related web application and you need a port number for your server.
//! Looking at the number pad of your phone and figuring out the right buttons to press to get `rust` is hard work.
//! Instead you could use this crate and the following code:
//!
//! ```
//! # extern crate t9;
//! # use t9::pad;
//! # fn check_that_rust_corresponds_to_7878() {
//! let digits = pad::digits_for("rust");
//! let output = digits.to_string();
//! assert_eq!(output, String::from("7878"));
//! # }
//! ```
//!
//! If you, like me, tend to forget which word you used to come up with a port number, this crate has got your back.
//! You just need to create a tree full of candidate words and call `words_at`:
//!
//! ```
//! # extern crate t9;
//! # use t9::{pad, tree::Tree};
//! # fn check_that_7878_corresponds_to_rust() {
//! # let mut tree = Tree::empty();
//! # tree.add("rust");
//! let digits = pad::digits_for("7878`");
//! let words = tree.words_at(digits);
//! assert!(words.contains(&String::from("rust")));
//! # }
//! ```
pub mod pad;
pub mod tree;
