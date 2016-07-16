extern crate inquirer;

use inquirer::{list, Error};

fn main() {
    #[derive(Debug)]
    enum Color { Red, Blue, Green };
    let choices =  &[("Red", Color::Red), ("Blue", Color::Blue), ("Green", Color::Green)];

    match list("Choose your favorite color:", choices) {
        Ok(result) => println!("You chose {:?}.", result),
        Err(Error::UserAborted) => {
            println!("Pressed Ctrl-C, exiting.");
            std::process::exit(1);
        }
        Err(err) => println!("{:?}", err)
    }
}
