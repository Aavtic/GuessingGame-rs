use std::io::stdin;
use rand::{Rng, thread_rng};
use std::cmp::Ordering;


fn main() {
    let mut win = false;
    let mut user_input = String::new();
    let mut user_guess: u8;
    const CHANCES:i32  = 3;
    println!("****Welcome to Guessing Game****");
    let mut rng = thread_rng();
    let secret_number = rng.gen_range(1..=10);
    for current_chance in 1..=CHANCES{
        stdin().read_line(&mut user_input).expect("Unable to read from stdin");
        user_guess = match user_input.trim().parse::<u8>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Integer expected, please provide an integer");
                continue;
            }
        };
        user_input.clear();
        match user_guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess is not correct. The secret number is greater than the given number. ");
                println!("You have {} left.", CHANCES - current_chance);
            },
            Ordering::Greater => {
                println!("Your guess is not correct. The secret number is lesser than the given number. ");
                println!("You have {} left.", CHANCES - current_chance);
            },
            Ordering::Equal => {
                win = true;
                break;
            }
        }
    }
    if win {
        println!("Congratulations!, You have guessed the number correctly!");
    } else {
        println!("You have run out of chances :-(");
        println!("The secret number was {}\nBetter luck next time", secret_number);
    }

}






