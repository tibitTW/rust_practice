use rand::{self, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("######## Start the guess game ########");
    println!("Guess the number from 1 to 99!");
    let secret_number = rand::rng().random_range(1..100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Enter a number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Exactly! You win!");
                break;
            }
        }
    }
}
