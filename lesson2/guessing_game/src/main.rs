use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number between 1 - 100:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {guess}");

    
        match guess.cmp(&mut secret_number) {
            Ordering::Less => println!("Your guess: {guess}, is too small!"),
            Ordering::Greater => println!("Your guess: {guess}, is too large!"),
            Ordering::Equal => {
                println!("Congratulations, you guessed correctly!\nAnswer: {secret_number}");
                break;
            }
        }
    }
}
