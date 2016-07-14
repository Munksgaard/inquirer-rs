extern crate inquirer;

use inquirer::list;

fn main() {
    let choices: Vec<_> = vec!["An option", "Another option", "Something else?"].into_iter()
        .enumerate()
        .map(|(index, item)| (item, index))
        .collect();

    let result = list("Choose an option:", &choices).unwrap();

    println!("You chose {:?}.", result);
}
