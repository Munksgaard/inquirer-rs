use std::io::{Write, stdout, stdin};
use termion::{TermRead, TermWrite, IntoRawMode, color, Key};

use error::Error;

/// Confirm a selection by answering yes or no.
///
/// The return type is a `Result` that contains a bool, indicating if the user
/// answered yes or no. The `default` parameter indicates what happens if the
/// user presses Enter.
///
/// # Examples
///
/// ```rust,no_run
/// # extern crate inquirer;
/// let answer = inquirer::confirm("Do you want to use inquirer?", true);
/// ```
///
/// ## Error Handling
///
/// ```rust,no_run
/// # extern crate inquirer;
/// match inquirer::confirm("Do you want to use inquirer?", true) {
///     Ok(result) => println!("You chose {:?}.", result),
///     Err(inquirer::Error::UserAborted) => {
///         println!("Pressed Ctrl-C, exiting.");
///         std::process::exit(1);
///     }
///     Err(err) => println!("{:?}", err)
/// }
/// ```
pub fn confirm(prompt: &str, default: bool) -> Result<bool, Error> {
    let stdin = stdin();
    let mut stdout = try!(stdout().into_raw_mode());

    try!(stdout.color(color::Green));
    print!("[?] ");
    try!(stdout.reset());
    print!("{} ", prompt);
    let (y, n) = if default {
        ('Y', 'n')
    } else {
        ('y', 'N')
    };
    print!("({}/{}) ", y, n);

    try!(stdout.lock().flush());

    let mut input = stdin.keys();

    let mut result = default;

    loop {
        let next = try!(input.next().ok_or_else(|| Error::NoMoreInput));

        match try!(next) {
            Key::Char('\n') =>       // Enter: Use the default
                break,
            Key::Char('y') | Key::Char('Y') => {
                result = true;
                break;
            }
            Key::Char('n') | Key::Char('N') => {
                result = false;
                break;
            }
            Key::Ctrl('c') => {
                print!("\n\r");
                return Err(Error::UserAborted);
            }
            _ => {
                // pass
            }
        }
    }

    print!("\n\r");
    Ok(result)
}
