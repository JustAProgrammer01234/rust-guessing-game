use std::cmp::Ordering; 
use std::io; 

use rand::Rng; 

fn main() {
    println!("Scripto's guessing game!");
    println!("------------------------"); 
    println!("Guess the right number that's from 1-100."); 
    println!("Take note that you only have 5 tries for this."); 
    println!("Good luck!\n"); 

    let right_number = rand::thread_rng().gen_range(1..=100); 

    let mut tries = 0; 
    let mut tries_left; 

    loop {
        println!("Please input your guess:"); 

        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue, 
        };  

        match guess.cmp(&right_number) {
            Ordering::Less => println!("Your answer is too small!"), 
            Ordering::Equal => { 
                println!("You got the right answer! Congrats!"); 
                break;
            }, 
            Ordering::Greater => println!("Your answer is too big!")
        }

        tries += 1; 
        tries_left = 5 - tries; 

        if tries == 5 {
            println!("Already in 5 tries! Game over!"); 
            println!("The answer is {right_number}!"); 
            break; 
        } else {
            println!("You only have {tries_left} tries left!"); 
        }
    }
}
