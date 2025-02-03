use std::io;
use rand::Rng; //import the Rng trait which determines the random number
use std::cmp::Ordering; //import the Ordering trait which determines the order of the numbers

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate a random number between 1 and 100
    
    loop {

    let mut guess = String::new(); //we create a new string instance

    io::stdin() //gives us access to standard input
        .read_line(&mut guess) //The full job of read_line is to take whatever the user types into standard input and append that into the guess string
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() { 
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");

    
    println!("Please input your guess.");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break; // makes the loop stop when the user guesses the right number
        }
    }
    }

}