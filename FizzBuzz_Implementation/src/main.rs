fn main() {
    // Iterate through numbers 1 to 100
    for number in 1..=100 {
        if number % 3 == 0 {
            println!("Fizz");
    
        // If divisible by 5, print "Buzz"
        } else if number % 5 == 0 {
            println!("Buzz");
    
        // If divisible by both, print "FizzBuzz"
        } else if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else {
            println!("{}", number);
        }
    }     
}