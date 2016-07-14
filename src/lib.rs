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

use std::io::{Write, stdout, stdin};
use std::fmt::Display;
use termion::{TermRead, TermWrite, IntoRawMode, color, Style, Key};

/// An option the user can choose
///
/// (Since the name "Option" is reserved for the well-known type representing
/// nullability, we are calling this one "Choice".)
pub trait Choice {
    /// User visible text
    type Text: Display;
    /// Internal value representing this choice
    type Value;

    /// Get a reference to the text
    fn text(&self) -> &Self::Text;

    /// Get a reference to the value of this choice
    fn value(&self) -> &Self::Value;
}

impl<'a> Choice for &'a str {
    type Text = &'a str;
    type Value = &'a str;

    fn text(&self) -> &Self::Text {
        self
    }

    fn value(&self) -> &Self::Value {
        &self
    }
}

impl<'a, T, V> Choice for (T, V) where
    T: Display
{
    type Text = T;
    type Value = V;

    fn text(&self) -> &T {
        &self.0
    }

    fn value(&self) -> &V {
        &self.1
    }
}

/// Render a list the user can select one value from
pub fn list<'c, C, V>(prompt: &str, choices: &'c [C]) -> Result<&'c V, Error> where
    C: Choice<Value=V>
{
    let stdin = stdin();
    let mut stdout = try!(stdout().into_raw_mode());
    try!(stdout.hide_cursor());

    try!(stdout.color(color::Green));
    print!("[?] ");
    try!(stdout.reset());
    println!("{}", prompt);

    for _ in 0..choices.len() - 1 {
        println!("");
    }

    let mut cur: usize = 0;

    let mut input = stdin.keys();

    loop {
        try!(stdout.move_cursor_up(choices.len() as u32));
        for (i, s) in choices.iter().enumerate() {
            if cur == i {
                print!("\n\r");
                try!(stdout.clear_line());
                try!(stdout.style(Style::Bold));
                print!("  > {}", s.text());
                try!(stdout.reset());
            } else {
                print!("\n\r");
                try!(stdout.clear_line());
                print!("    {}", s.text());
            }
        }

        try!(stdout.lock().flush());

        let next = try!(input.next().ok_or_else(|| Error::NoMoreInput));

        match try!(next) {
            // newline
            Key::Char('\n') => {
                break;
            }
            Key::Up if cur != 0 => {
                cur -= 1;
            }
            Key::Down if cur != choices.len() - 1 => {
                cur += 1;
            }
            Key::Ctrl('c') => {
                panic!("Ctrl-C");
            }
            _ => {
                // pass
            }
        }
    }

    print!("\n\r");
    try!(stdout.show_cursor());

    choices
        .iter()
        .nth(cur)
        .ok_or_else(|| Error::InvalidChoice(cur))
        .map(|choice| choice.value())
}

quick_error! {
    /// Inquirer Error
    #[derive(Debug)]
    pub enum Error {
        /// Error while dealing with file or stdin/stdout
        Io(err: std::io::Error) {
            from()
            cause(err)
            display("I/O error")
            description(err.description())
        }
        /// Input read error
        // `std::io::CharsError` is unstable!
        Chars(err: std::io::CharsError) {
            from()
            cause(err)
            display("Chars error")
            description(err.description())
        }
        /// Invalid choice
        // TODO: Make this a type system error instead
        InvalidChoice(option_num: usize) {
            display("Option `{}` is not valid", option_num)
            description("Invalid choice")
        }
        /// No more input
        NoMoreInput {
            display("Didn't get any more input")
        }
    }
}
