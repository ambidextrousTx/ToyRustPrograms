use std::io;
use std::rand::random;

fn guessed_correctly(guess: uint, machine: uint) -> bool {
    guess == machine
}

fn main() {
    let random_value = random::<uint>() % 10u;
    let mut guess = 0u;
    println!("{}", random_value);
    println!("{}", guessed_correctly(guess, random_value));

}
