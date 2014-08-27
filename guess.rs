use std::io;
use std::rand::random;

mod guess {
    pub fn guessed_correctly(guess: uint, machine: uint) -> bool {
        guess == machine
    }
}

#[cfg(not(target_os = "linux"))]
fn test_os() {
    println!("You are not running Linux!")
}

fn main() {
    test_os();
    let random_value = random::<uint>() % 10u;
    let mut guess = 0u;
    for line in io::stdin().lines() {
        // guess = line.unwrap().to_int(); 
        println!("{}", line.unwrap());
    }

    println!("{}", random_value);
    println!("{}", guess);
    println!("{}", guess::guessed_correctly(guess, random_value));

}
