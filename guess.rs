use std::rand::random;

mod guess {
    pub fn guessed_correctly(guess: uint, machine: uint) -> bool {
        guess == machine
    }
}

fn main() {
    let random_value = random::<uint>() % 10u;
    let mut guess = 0u;
    guess += 1;
    println!("{}", random_value);
    println!("{}", guess::guessed_correctly(guess, random_value));

}
