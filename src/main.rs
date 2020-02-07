extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn read_number() -> u32 {
    let stdin = io::stdin();
    let mut guess = String::new();

    stdin.read_line(&mut guess)
        .expect("io:stdin().read_line failure");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input parsing failure, please, try again");
            return read_number();
        }
    };

    return guess;
}

fn new_try(expected_number: u32) {
    let guess = read_number();

    println!("You input: {}", guess);

    match guess.cmp(&expected_number) {
        Ordering::Less => {
            println!("Too small!");
            return new_try(expected_number);
        }
        Ordering::Greater => {
            println!("Too big!");
            return new_try(expected_number);
        }
        Ordering::Equal => {
            println!("Sic! You amazing! Guessed correctly!");
            return;
        }
    }
}

fn main() {
    let expected_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("Try to guess number!");

    println!("Please, input you number");

    new_try(expected_number);
}