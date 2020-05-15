use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io;
use std::io::Write;
use std::process;

#[macro_use]
extern crate clap;

fn parse_help() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");

    let _matches = clap_app!((name) =>
        (version: version)
        (about: description)
    )
    .get_matches();
}
}

fn start_game() {
    println!("Guess the number between 1 and 100!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut prev_guess = None;

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if Some(guess) == prev_guess {
            println!("Doubling down on that last one?");
        } else {
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
            prev_guess = Some(guess);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    parse_help();
    start_game();
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running guessing_game: {}", err);
        process::exit(1);
    }
}
