use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess a number (1-100)");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  Guess must be a number");
                continue;
            }
        };
    
        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("{}", "Too high, guess again".red()),
            Ordering::Less => println!("{}", "Too low, guess again".red()),
            Ordering::Equal => {
                println!("{}", "Correct!".green());
                break;
            }
        }
    }
}