use rand::Rng;
use std::{io, cmp::Ordering, time::{Instant, Duration}};

struct Highscore {
    name: String,
    guesses: u32,
    duration: Duration,
    difficulty: i32,
}

fn main() {
    println!("Hello, let's play a guessing game!");
    
    let mut highscores: Vec<Highscore> = Vec::new();
    
    loop {
        let difficulty_range = difficulty_level();
        let (secret_number, range) = secret_number_generator(difficulty_range);
        
        println!("Guess a number between 1 and {}!", range);
        
        let mut guess_count = 0;
        let start_time = Instant::now();
        
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            
            if guess.trim().to_lowercase() == "quit" {
                println!("Thanks for playing!");
                return;
            }
            
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                },
            };
            
            guess_count += 1;
            println!("You guessed {}", guess);
            
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {    
                    let duration = start_time.elapsed();
                    println!("You win! You took {} guesses and {} seconds.", 
                             guess_count, duration.as_secs());
                    
                    println!("Enter your name for the highscore:");
                    let player_name = get_input();
                    
                    // Add new highscore
                    highscores.push(Highscore {
                        name: player_name,
                        guesses: guess_count,
                        duration,
                        difficulty: difficulty_range,
                    });
                    
                    // Display highscores
                    display_highscores(&highscores);
                    break;
                }
            }
        }
        
        println!("Do you want to play again? (yes/no)");
        let answer = get_input().to_lowercase();
        
        if answer != "yes" && answer != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
}

fn difficulty_level() -> i32 {
    loop {
        println!("Choose a difficulty level (1-3):");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input to an integer
        match input.trim().parse::<i32>() {
            Ok(1) => {
                println!("You selected Easy. The secret number will be between 1 and 50.");
                return 50;
            },
            Ok(2) => {
                println!("You selected Medium. The secret number will be between 1 and 100.");
                return 100;
            },
            Ok(3) => {
                println!("You selected Hard. The secret number will be between 1 and 200.");
                return 200;
            },
            _ => {
                println!("Invalid input. Please enter a valid difficulty level (1, 2, 3).");
                // Continue the loop to ask for input again
                continue;
            }
        };
    }
}

fn secret_number_generator(range: i32) -> (i32, i32) {
    let secret_number = rand::thread_rng().gen_range(1..=range);
    (secret_number, range)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn display_highscores(highscores: &[Highscore]) {
    if highscores.is_empty() {
        println!("No highscores yet!");
        return;
    }
    
    println!("\n===== HIGHSCORES =====");
    
    // Create copies of highscores grouped by difficulty
    let mut easy_scores: Vec<&Highscore> = highscores.iter().filter(|s| s.difficulty == 50).collect();
    let mut medium_scores: Vec<&Highscore> = highscores.iter().filter(|s| s.difficulty == 100).collect();
    let mut hard_scores: Vec<&Highscore> = highscores.iter().filter(|s| s.difficulty == 200).collect();
    
    // Sort each difficulty group by guesses (primarily) and duration (secondary)
    for scores in [&mut easy_scores, &mut medium_scores, &mut hard_scores] {
        scores.sort_by(|a, b| {
            match a.guesses.cmp(&b.guesses) {
                Ordering::Equal => a.duration.cmp(&b.duration),
                other => other,
            }
        });
    }
    
    // Display easy highscores
    if !easy_scores.is_empty() {
        println!("\nEasy Mode:");
        println!("{:<15} {:<10} {:<10}", "Name", "Guesses", "Time (s)");
        println!("{:-<35}", "");
        for (idx, score) in easy_scores.iter().take(5).enumerate() {
            println!("{:<2}. {:<12} {:<10} {:<10}", 
                     idx + 1, 
                     score.name, 
                     score.guesses, 
                     score.duration.as_secs());
        }
    }
    
    // Display medium highscores
    if !medium_scores.is_empty() {
        println!("\nMedium Mode:");
        println!("{:<15} {:<10} {:<10}", "Name", "Guesses", "Time (s)");
        println!("{:-<35}", "");
        for (idx, score) in medium_scores.iter().take(5).enumerate() {
            println!("{:<2}. {:<12} {:<10} {:<10}", 
                     idx + 1, 
                     score.name, 
                     score.guesses, 
                     score.duration.as_secs());
        }
    }
    
    // Display hard highscores
    if !hard_scores.is_empty() {
        println!("\nHard Mode:");
        println!("{:<15} {:<10} {:<10}", "Name", "Guesses", "Time (s)");
        println!("{:-<35}", "");
        for (idx, score) in hard_scores.iter().take(5).enumerate() {
            println!("{:<2}. {:<12} {:<10} {:<10}", 
                     idx + 1, 
                     score.name, 
                     score.guesses, 
                     score.duration.as_secs());
        }
    }
    
    println!("\n");
}