use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Guess the number - ");
    println!("Please input yout guess: ");

    let random_secret: u32 = rand::thread_rng().gen_range(0, 101);

    loop {
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Invalid input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&random_secret) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Greater".red()),
        }
    }
}
