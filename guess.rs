use std::rand::random;

fn main() {
    let random_value = random::<u8>() % 10u8;
    let mut guess = 0u8;

    println!("{}", random_value);

}
