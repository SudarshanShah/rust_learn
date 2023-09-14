use std::io; // using input/output library from standard library
use rand::Rng; // Rng -> trait defines methods that random number generators implement
use std::cmp::Ordering; // comparing two values and return result of type Ordering enum

fn main() {
    // println!() --> its a Rust macro
    println!("Guess the number!");

    // rand::thread-rng() --> gives us particular random number generator
    // gen-range(start..=end) --> method on generator --> method defined by Rng trait
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // mutable variable
        // String::new() --> return a new empty string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse() --> return Result enum with variants OK() and Err()
        // Err(_) --> '_' means we want to match all values in error    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
