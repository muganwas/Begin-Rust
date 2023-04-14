use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    loop {
        println!("Please input your guess!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {guess}");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if (guess < 1) || (guess > 100) {
            println!("Please input a number between 1 and 100");
            continue;
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
