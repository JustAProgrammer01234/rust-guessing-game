// bringing the io library to scope that comes from the standard library which is std.
use std::io; 

fn main() {
    println!("Guessing game!");
    println!("Guess the right number that's from 1-100. Good luck!"); 

    loop {
        println!("Please input your guess:"); 
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); 

        println!("You guessed: {guess}"); 
    }
}
