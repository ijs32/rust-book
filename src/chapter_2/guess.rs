use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn _guessing_game() {
    println!("Guess the number!");

    let num: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter a guess!");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Guess a number bozo!");
                continue;
            }
        };

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
