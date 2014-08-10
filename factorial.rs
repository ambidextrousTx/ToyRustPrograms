fn main() {
    let mut prod = 1u;
    let mut count = 1u;

    println!("Computing factorial!");
    loop {
        count += 1;
        if count == 11 {
            println!("We are done here");
            break;
        }
    
        prod *= count;
    }

    println!("Computed the factorial for 10: {}", prod);
}
