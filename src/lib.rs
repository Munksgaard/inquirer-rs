extern crate termion;

use termion::{TermRead, TermWrite, IntoRawMode, color, Style, Key};
use std::io::{Write, stdout, stdin};

pub fn simple_list(prompt: &str, choices: &[&str]) -> usize {
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
        for (i, s) in choices.iter().enumerate() {
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
    cur
}

pub fn list<'a, T>(prompt: &str, choice_pairs: &'a [(&str, T)]) -> &'a T {
    let choices: Vec<&str> = choice_pairs.iter().map(|&(s, _)| s).collect();
    let choice = simple_list(prompt, &choices);

    &choice_pairs[choice].1
}
