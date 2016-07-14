use std::io::{Write, stdout, stdin};
use termion::{TermRead, TermWrite, IntoRawMode, color, Style, Key};

use choice::Choice;
use error::Error;

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

    choices
        .iter()
        .nth(cur)
        .ok_or_else(|| Error::InvalidChoice(cur))
        .map(|choice| choice.value())
}
