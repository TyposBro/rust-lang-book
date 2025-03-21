use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let select = rand::rng().random_range(1..=100);
    println!("Guess the number");

    loop {
        println!("Please input number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        print!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&select) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
