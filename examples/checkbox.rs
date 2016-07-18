extern crate inquirer;

fn main() {
    let choices = &["Pepperoni", "Ham", "Pineapple"];

    match inquirer::checkbox("Choose your toppings:", choices) {
        Ok(result) => println!("You chose {:?}.", result),
        Err(inquirer::Error::UserAborted) => {
            println!("Pressed Ctrl-C, exiting.");
            std::process::exit(1);
        }
        Err(err) => println!("{:?}", err),
    }
}
