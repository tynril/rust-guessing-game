#![allow(unstable)]

use std::{rand, io, os};
use std::cmp::Ordering;

fn compare(a:u32, b:u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

fn main() {
    let random_value = (rand::random::<u32>() % 100) + 1;

    let is_cheating = os::args().len() > 1 && os::args()[1] == "-cheat";
    if is_cheating {
        println!("The random value is {}, cheater.", random_value);
    }

    let mut guesses = 0;
    loop {
        guesses += 1;

        println!("Enter your guess:");
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Unable to read what you've written.")
                               .trim()
                               .parse::<u32>();
        let num = match input {
            Some(n) => n,
            None    => {
                println!("Yeah, that wasn't a number. Clearly, you don't want to play.");
                return;
            }
        };

        match compare(num, random_value) {
            Ordering::Less    => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal   => {
                println!("You win! It only took you {} guesses.", guesses);
                return;
            },
        };
    }
}
