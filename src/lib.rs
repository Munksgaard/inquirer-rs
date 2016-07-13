extern crate termion;

use termion::{TermRead, TermWrite, IntoRawMode, color, Style, Key};
use std::io::{Write, stdout, stdin};
use std::fmt::Display;

pub fn list<'a, T, S: Display>(prompt: &str, choices: &'a [(S, T)]) -> &'a T {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.hide_cursor().unwrap();

    stdout.color(color::Green).unwrap();
    print!("[?] ");
    stdout.reset().unwrap();
    println!("{}", prompt);

    for _ in 0..choices.len()-1 {
        println!("")
    }

    let mut cur: usize = 0;

    let mut input = stdin.keys();

    loop {
        stdout.move_cursor_up(choices.len() as u32).unwrap();
        for (i, &(ref s, _)) in choices.iter().enumerate() {
            if cur == i {
                print!("\n\r");
                stdout.clear_line().unwrap();
                stdout.style(Style::Bold).unwrap();
                print!("  > {}", s);
                stdout.reset().unwrap();
            } else {
                print!("\n\r");
                stdout.clear_line().unwrap();
                print!("    {}", s);
            }
        }

        stdout.lock().flush().unwrap();

        match input.next().unwrap().unwrap() {
            Key::Char('\n') => { // newline
                break;
            }
            Key::Up if cur != 0 => {
                cur = cur - 1;
            }
            Key::Down if cur != choices.len() - 1 => {
                cur = cur + 1;
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
    stdout.show_cursor().unwrap();
    &choices[cur].1
}
