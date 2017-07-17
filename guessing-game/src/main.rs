extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // This variable keeps count of how many tries was needed to guess
    let mut count: u32 = 0;

    loop {
        println!("Please input your guess between 1 and 101.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => {
              println!("Please enter a valid number");
              continue
            }
        };
    
        match guess.cmp(&secret_number) {
          Ordering::Less    => {
            count = count + 1;
            println!("Too small!") },
          Ordering::Greater => {
            count = count + 1;
            println!("Too big!") },
          Ordering::Equal   => {
              println!("You win! But you needed {} tries!! Better luck next time!", count);
              break;
            }
        }

        println!("You guessed: {}", guess);
    }
}