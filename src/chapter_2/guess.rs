use rand::Rng;
use std::{cmp::Ordering, io};

pub fn guessing_game() {
    println!("Guess the number!");

    let num: i32 = rand::thread_rng().gen_range(1, 101);

    struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value > 100 || value < 0 {
                panic!("Guess must be between 1 and 100")
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
    loop {
        println!("Enter a guess!");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Guess a number bozo!");
                continue;
            }
        };

        let guess = Guess::value(&guess);

        println!("You guessed {}", guess);

        match guess.cmp(&num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Well Done!");
                break;
            }
        }
    }
}
