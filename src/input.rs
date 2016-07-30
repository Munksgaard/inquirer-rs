use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::{color, style};

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

    try!(write!(stdout, "{}[?] {}{} ", color::Fg(color::Green), style::Reset, prompt));

    try!(stdout.lock().flush());

    match TermRead::read_line(&mut stdin) {
        Ok(Some(s)) => Ok(s),
        Ok(None) => Err(Error::NoMoreInput),
        Err(e) => Err(Error::Io(e)),
    }
}
