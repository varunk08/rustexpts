extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game.");

    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("Secret number: {}", secret_number);

    loop {
	// Get user guess
	println!("Please input your guess:");
	let mut guess= String::new();
	io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

	// Shadow the "guess" variable into a different type.
	let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
	println!("You guessed: {}", guess);

	// Check user guess with generated random number
	match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too Small!"),
	    Ordering::Greater => println!("Too big!"),
	    Ordering::Equal => {
		println!("You Win!");
		break;
	    }
	}
    }
}
