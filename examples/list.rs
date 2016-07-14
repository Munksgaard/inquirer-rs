extern crate inquirer;

use inquirer::list;

fn main() {
    let choices =  &[("An option", 42), ("Another option", 13), ("Something else?", 24)];
    let result = list("Choose an option:", choices);
    println!("You chose {}.", result);
}
