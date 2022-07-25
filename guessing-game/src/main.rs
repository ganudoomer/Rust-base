use colored::*;
use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let mut rng = thread_rng();
    println!("Guess the number!");
    let secret_number: u32 = rng.gen_range(0..101);
    println!("The secret number is {}", secret_number);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guessed {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}", "Too small!".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Too Great!".yellow()),
        }
        println!("{}", guess);
    }
}
