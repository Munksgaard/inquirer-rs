extern crate inquirer;

use inquirer::{confirm, Error};

fn main() {
    match confirm("Is it working?", false) {
        Ok(true) =>
            println!("It was working!"),
        Ok(false) =>
            println!("It's not working :("),
        Err(Error::UserAborted) => {
            println!("Pressed Ctrl-C, exiting.");
            std::process::exit(1);
        }
        Err(err) => println!("{:?}", err),
    }
}
