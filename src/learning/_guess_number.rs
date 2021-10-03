use std::{
    cmp::Ordering,
    io,
};

use rand::{self, Rng};

use crate::learning::_guess_number::guess::Guess;

pub fn main() {}

fn guess_number() {
    let range = 1..=100;
    let secret_number = rand::thread_rng()
        .gen_range(range);

    loop {
        println!("guess or quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("read line failure");

        println!("you guess {}", input);

        let input: i32 = input.trim().parse()
            .expect("parse failure");

        let guess = Guess::new(input);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }

    println!("game over!");
}


mod guess {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 0 || value > 100 {
                panic!("are you sb!")
            }
            return Guess { value };
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
