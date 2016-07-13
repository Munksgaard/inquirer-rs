#![feature(slice_patterns)]

extern crate termion;

mod lib;

use lib::*;

fn main() {
    let choices =  &[("hej", 42), ("med", 13), ("dig", 24)];
    let result = list("Choose an option:", choices);
    println!("you chose {}", result);
}
