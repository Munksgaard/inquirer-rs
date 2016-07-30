use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

use choice::Choice;
use error::Error;

/// Render a list the user can select one value from
///
/// The return type is a `Result` that contains either a reference to the value
/// of the selected choice or a custom error. **Please note:** If the user
/// presses <kbd>Ctrl</kbd><kbd>C</kbd>, this will result in an `UserAborted`
/// error that the application should handle.
///
/// # Examples
///
/// Simple example, using only string slices for options.
///
/// ```rust,no_run
/// extern crate inquirer;
///
/// let choices =  &["Red", "Blue", "Green"];
/// let result = inquirer::list("Chose your favorite color:", choices).unwrap();
/// ```
///
/// After choosing the first option, `result` will be `"Red"`.
///
/// ## Complex types
///
/// You can also use tuples for options, where the first item is printed to the
/// screen, but the second item is returned as the value of the selection.
///
/// ```rust,no_run
/// # extern crate inquirer;
/// enum Color { Red, Blue, Green };
/// let choices = &[("Red", Color::Red), ("Blue", Color::Blue), ("Green", Color::Green)];
/// let result = inquirer::list("Chose your favorite color:", choices).unwrap();
/// ```
///
/// Here, `result` will be `Red` when you select the first option.
///
/// ## Error Handling
///
/// ```rust,no_run
/// # extern crate inquirer;
/// let choices =  &["Red", "Blue", "Green"];
///
/// match inquirer::list("Choose your favorite color:", choices) {
///     Ok(result) => println!("You chose {:?}.", result),
///     Err(inquirer::Error::UserAborted) => {
///         println!("Pressed Ctrl-C, exiting.");
///         std::process::exit(1);
///     }
///     Err(err) => println!("{:?}", err)
/// }
/// ```
pub fn list<'c, C, V>(prompt: &str, choices: &'c [C]) -> Result<&'c V, Error>
    where C: Choice<Value = V>
{
    let stdin = stdin();
    let mut stdout = try!(stdout().into_raw_mode());
    try!(write!(stdout,
           "{}{}[?] {}{}\n",
           cursor::Hide,
           color::Fg(color::Green),
           style::Reset,
           prompt));

    for _ in 0..choices.len() - 1 {
        try!(write!(stdout, "\n"));
    }

    let mut cur: usize = 0;

    let mut input = stdin.keys();

    loop {
        print!("{}", cursor::Up(choices.len() as u16));
        for (i, s) in choices.iter().enumerate() {
            try!(write!(stdout, "\n\r{}", clear::CurrentLine));

            if cur == i {
                try!(write!(stdout, "{}  > {}{}", style::Bold, s.text(), style::Reset));
            } else {
                try!(write!(stdout, "    {}", s.text()));
            }
        }

        try!(stdout.lock().flush());

        let next = try!(input.next().ok_or_else(|| Error::NoMoreInput));

        match try!(next) {
            Key::Char('\n') => {
                // Enter
                break;
            }
            Key::Up if cur != 0 => {
                cur -= 1;
            }
            Key::Down if cur != choices.len() - 1 => {
                cur += 1;
            }
            Key::Ctrl('c') => {
                try!(write!(stdout, "\n\r{}", cursor::Show));
                return Err(Error::UserAborted);
            }
            _ => {
                // pass
            }
        }
    }

    try!(write!(stdout, "\n\r{}", cursor::Show));

    choices.get(cur)
        .ok_or_else(|| Error::InvalidChoice(cur))
        .map(|choice| choice.value())
}
