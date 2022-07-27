use std::cmp::Ordering; 
use std::io; 

use rand::Rng; 

fn main() {
    println!("Guessing game!");
    println!("Guess the right number that's from 1-100. Good luck!"); 

    let right_number = rand::thread_rng().gen_range(1..=100); 

    loop {
        println!("Please input your guess:"); 

        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); 

        let guess: u32 = guess.trim().parse().expect("Type a number brah."); 

        match guess.cmp(&right_number) {
            Ordering::Less => println!("It's too small!"), 
            Ordering::Equal => { 
                println!("You got the right answer! Congrats!"); 
                break;
            }, 
            Ordering::Greater => println!("It's too big!")
        }
    }
}
