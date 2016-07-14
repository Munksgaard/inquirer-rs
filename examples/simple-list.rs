extern crate inquirer;

use inquirer::simple_list;

fn main() {
    let choices =  &["An option", "Another option", "Something else?"];
    let result = simple_list("Choose an option:", choices);
    println!("You chose {}.", result);
}
