extern crate inquirer;

use inquirer::{list, Error};

fn main() {
    let choices =  &[("An option", 42), ("Another option", 13), ("Something else?", 24)];

    match list("Choose an option:", choices) {
        Ok(result) => println!("You chose {:?}.", result),
        Err(Error::UserAborted) => {
            println!("Pressed Ctrl-C, exiting.");
            std::process::exit(1);
        }
        Err(err) => println!("{:?}", err)
    }
}
