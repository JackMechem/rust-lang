use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("\nGuess the number\n\n");

    let number_to_guess = rand::thread_rng().gen_range(1..=100);

    // println!("The number to guess is {number_to_guess}");
    loop {
        println!("Your guess (1-100): ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error Reading Line (main.rs : line 10)\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Input a Number");
                continue;
            },
        };

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }
}
