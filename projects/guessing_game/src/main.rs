use std::cmp::Ordering;
// This is a simple guessing game in Rust.

use std::io;

use rand::Rng;

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    // Uncomment the line below to hide the secret number

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Trim the input and parse it to a number
        // let guess: u32 = guess.trim().parse().expect("Please enter a valid number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
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
