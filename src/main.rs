#![allow(unused)]

use std::io::stdin;
use rand::Rng;

fn main() {
    let number = number(1);

    loop {
        setup();
        let user_input = user_input();

        if user_input == number {
            println!("You won! The correct number was : {}", user_input);
            break
        } else {
            println!("Incorrect!\nYou entered : {}\n", &user_input);
            continue
        }
    }
}

// generates a random int between 0 and 'max' value
fn number(max: i32) -> i32 {
    rand::thread_rng()
        .gen_range(0..=max)
}

// read user input
fn user_input() -> i32 {
    let mut input = String::new();

    stdin() // read user input
        .read_line(&mut input)
        .expect("Failed to read line");

    // parse input (String) -> input (int)
    let result: i32 = input.trim().parse().expect("Invalid entry");

    result // return user input as integer
}

fn setup() {
    println!("Enter an integer between 1-100")
}