extern crate inquirer;

fn main() {
    let answer = inquirer::input("What is the airspeed velocity of an unladen swallow?");
    println!("You answered: {:?}", answer);
}
