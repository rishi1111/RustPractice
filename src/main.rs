mod practice;

use core::num::ParseIntError;
use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn user_input() -> Result<u32, ParseIntError> {
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn guessing(){
    println!("Guess the number!");

    loop {
        let current: u32 = thread_rng().gen_range(0..11);
        let res: Result<u32, ParseIntError> = user_input();
        match res {
            Ok(guess) => {
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
            Err(_) => {
                continue;
            }
        }
    }
}
fn main() {
    practice::c_to_f(30.0);
    practice::f_to_c(98.6);
    let fb = practice::fibbonacci(9);
    println!("{fb}");
    practice::twelve_days_of_xmas();
}
