use std::io;

fn usd_to_kes(usd: u32) -> u32 {
    usd * 130
}

fn main() {
    println!("Please input USD amount");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input and handle the result
    match input.trim().parse::<u32>() {
        Ok(usd) => {
            let kes = usd_to_kes(usd);
            println!("The converted amount is: {} KES", kes);
        }
        Err(_) => {
            println!("Please enter a valid number!");
        }
    };
}