#![allow(unused)]

use std::io;
use rand::Rng;

fn main() {


    println!("{}", number(100));

}

// generator number
fn number(max: i32) -> i32 {
    rand::thread_rng().gen_range(0..=max) // generates a random int between 0 and 'max' value
}