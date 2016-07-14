//! Inquirer – Fancy user interaction on the command line

// Termion depends on nighty (for Keys), we need this for the unstable `std::io::CharsError`
#![feature(io)]

#![deny(missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unused_import_braces, unused_qualifications)]

extern crate termion;
#[macro_use] extern crate quick_error;

mod choice;
mod error;
mod list;

pub use choice::Choice;
pub use error::Error;
pub use list::list;
