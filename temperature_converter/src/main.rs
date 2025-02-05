use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Enter temperature in Fahrenheit:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Convert input string to float
    match input.trim().parse::<f64>() {
        Ok(fahrenheit) => {
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{:.1}°F is equal to {:.1}°C", fahrenheit, celsius);
        }
        Err(_) => {
            println!("Please enter a valid number!");
        }
    }
}