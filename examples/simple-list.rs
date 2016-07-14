extern crate inquirer;

use inquirer::list;

fn main() {
    let choices =  &["An option", "Another option", "Something else?"];
    let result = list("Choose an option:", choices).unwrap();
    println!("You chose {:?}.", result);
}
