
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use crate::input::input_number;

pub fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    let mut count = 0;
    loop {
        println!("Please input your guess.");

        let guess = input_number();

        count += 1;

        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("low"),
            Ordering::Greater => println!("high"),
            Ordering::Equal => {
                println!("you win! Guesses: {count}");
                break;
            }
        }
    }
}