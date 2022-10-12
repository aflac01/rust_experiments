mod branches;
mod guessing_game;
mod shadowing;
mod ownership;
mod struct_demo;
mod input;
mod json_parsing;
use std::io;
use crate::branches::{array_loop, counting_up, liftoff, loop_test};
use crate::guessing_game::guessing_game;
use crate::ownership::ownership;
use crate::shadowing::shadowing;
use crate::struct_demo::rectangles;
use crate::json_parsing::json_parsing;

fn main() {
    loop{
        println!("Choose example");
        println!("1 : Guessing Game");
        println!("2 : Counting Up");
        println!("3 : Counting Down");
        println!("4 : Loop");
        println!("5 : Shadowing");
        println!("6 : Array Loop");
        println!("7 : Ownership");
        println!("8 : Struct");
        println!("9 : Json");



        println!("0 : Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {
                num
            }
            Err(_) => continue,
        };

        match choice {
            1 => guessing_game(),
            2 => counting_up(),
            3 => liftoff(),
            4 => loop_test(),
            5 => shadowing(),
            6 => array_loop(),
            7 => ownership(),
            8 => rectangles(),
            9 => json_parsing(),
            _ => break,
        }
        println!();




    }

}

