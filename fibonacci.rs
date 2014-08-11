// compute the nth Fibonacci number
fn main() {
    let mut first = 1u;
    let mut second = 1u;
    let mut sum = 0u;
    let mut count = 0u;

    println!("Computing the 10th Fibonacci number");
    loop {
        count += 1;
        if count == 9 {
            println!("We are done here");
            break;
        }

        sum = first + second;
        first = second;
        second = sum;
    
    }

    println!("Computed the 10th Fibonacci {}", sum);
}
