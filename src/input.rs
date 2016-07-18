use std::io::{Write, stdout, stdin};
use termion::{TermRead, TermWrite, color};

use error::Error;

/// Prompt the user to input a string.
///
/// The return type is a `Result` that contains a `String`.
///
/// # Examples
///
/// ```rust,no_run
/// # extern crate inquirer;
/// let answer = inquirer::input("What is the airspeed velocity of an unladen swallow?");
/// ```
///
/// ## Error Handling
///
/// ```rust,no_run
/// # extern crate inquirer;
/// match inquirer::input("What is the airspeed velocity of an unladen swallow?") {
///     Ok(result) => println!("You chose {:?}.", result),
///     Err(inquirer::Error::UserAborted) => {
///         println!("Pressed Ctrl-C, exiting.");
///         std::process::exit(1);
///     }
///     Err(err) => println!("{:?}", err)
/// }
/// ```
pub fn input(prompt: &str) -> Result<String, Error> {
    let mut stdin = stdin();
    let mut stdout = stdout();

    try!(stdout.color(color::Green));
    print!("[?] ");
    try!(stdout.reset());
    print!("{} ", prompt);

    try!(stdout.lock().flush());

    match TermRead::read_line(&mut stdin) {
        Ok(Some(s)) => Ok(s),
        Ok(None) => Err(Error::NoMoreInput),
        Err(e) => Err(Error::Io(e)),
    }
}
