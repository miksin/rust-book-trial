use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guessing Game");

    let min_value = 1;
    let max_value = 10;
    let secret_number = rand::thread_rng().gen_range(min_value..=max_value);

    loop {
        println!("Guess a number between {min_value}-{max_value}:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("read line failed!");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) if (min_value..=max_value).contains(&n) => n,
            Ok(_) | Err(_) => {
                println!("Please enter a number between {min_value}-{max_value}");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("win!");
                break;
            },
        }
    }
}
