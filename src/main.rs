use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input, please enter only numbers!");
                continue;
            }
        };

        let current: u32 = thread_rng().gen_range(0..11);

        match guess.cmp(&current) {
            Ordering::Less => println!("Aim Higher!"),
            Ordering::Equal => {
                println!("Correct guess! Exiting game!");
                break;
            }
            Ordering::Greater => println!("Aim lower!"),
        }
        println!("Current = {current}");
    }
}
