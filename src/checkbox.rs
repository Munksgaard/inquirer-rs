use std::io::{Write, stdout, stdin};
use termion::{TermRead, TermWrite, IntoRawMode, color, Style, Key};

use choice::Choice;
use error::Error;

/// Render a checkbox style list the user can select multiple value from.
///
/// The return type is a `Result` that contain a vector of references to results
/// or a custom error. **Please note:** If the user presses
/// <kbd>Ctrl</kbd><kbd>C</kbd>, this will result in an `UserAborted` error that
/// the application should handle.
///
/// # Examples
///
/// Simple example, using only string slices for options.
///
/// ```rust,no_run
/// extern crate inquirer;
///
/// let choices =  &["Pepperoni", "Ham", "Pineapple"];
/// let result = inquirer::checkbox("What topppings would you like?", choices).unwrap();
/// ```
///
/// ## Complex types
///
/// You can also use tuples for options, where the first item is printed to the
/// screen, but the second item is returned as the value of the selection.
///
/// ```rust,no_run
/// # extern crate inquirer;
/// let choices =  &[("Alpha", 1), ("Beta", 2), ("Gamma", 3)];
/// let result = inquirer::checkbox("Choose an option:", choices).unwrap();
/// ```
///
/// ## Error Handling
///
/// ```rust,no_run
/// # extern crate inquirer;
/// let choices =  &["Pepperoni", "Ham", "Pineapple"];
///
/// match inquirer::checkbox("Choose your toppings:", choices) {
///     Ok(result) => println!("You chose {:?}.", result),
///     Err(inquirer::Error::UserAborted) => {
///         println!("Pressed Ctrl-C, exiting.");
///         std::process::exit(1);
///     }
///     Err(err) => println!("{:?}", err)
/// }
/// ```
pub fn checkbox<'c, C, V>(prompt: &str, choices: &'c [C]) -> Result<Vec<&'c V>, Error>
    where C: Choice<Value = V>
{
    let stdin = stdin();
    let mut stdout = try!(stdout().into_raw_mode());
    try!(stdout.hide_cursor());

    try!(stdout.color(color::Green));
    print!("[?] ");
    try!(stdout.reset());
    println!("{} (Press <space> to select)", prompt);

    let mut selected: Vec<_> = (0..choices.len()).map(|_| false).collect();

    for _ in 0..choices.len() - 1 {
        println!("");
    }

    let mut cur: usize = 0;

    let mut input = stdin.keys();

    loop {
        try!(stdout.move_cursor_up(choices.len() as u32));
        for (i, s) in choices.iter().enumerate() {
            print!("\n\r");
            try!(stdout.clear_line());

            if cur == i {
                try!(stdout.style(Style::Bold));
                print!(" > ");
            } else {
                print!("   ");
            }

            if selected[i] {
                try!(stdout.style(Style::Bold));
                print!("● {}", s.text());
                try!(stdout.reset());
            } else {
                print!("○ {}", s.text());
            }

            try!(stdout.reset());

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
            Key::Char(' ') => selected[cur] = !selected[cur],
            Key::Ctrl('c') => {
                print!("\n\r");
                try!(stdout.show_cursor());
                return Err(Error::UserAborted);
            }
            _ => {
                // pass
            }
        }
    }

    print!("\n\r");
    try!(stdout.show_cursor());

    Ok(choices.iter()
        .zip(selected.iter())
        .filter_map(|(choice, &selected)| if selected {
            Some(choice.value())
        } else {
            None
        })
        .collect())
}
