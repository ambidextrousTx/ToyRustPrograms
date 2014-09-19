// Implement the Fizz Buzz until 30
fn main() {
    let mut count: int = 1;
    loop {
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz: {}", count);
        } else if count % 3 == 0 {
            println!("Fizz: {}", count);
        } else if count % 5 == 0 {
            println!("Buzz: {}", count);
        }

        if count == 30 {
            println!("All done");
            break;
        }

        count += 1;
    
    }

}
