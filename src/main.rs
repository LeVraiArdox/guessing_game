use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MAX: u32 = 100; // const is a constant (no way), u32 is an unsigned 32-bit integer

fn main() {
    println!("Guess the number!");
    println!("The secret number is between 1 and {MAX}");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut is for mutable variable

        io::stdin()  
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() { // trim() removes whitespace, parse() converts string to number
            Ok(num) => num,
            Err(_) => { // _ is a catchall value
                println!("Please enter a number!");
                continue;
            },
        };

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
