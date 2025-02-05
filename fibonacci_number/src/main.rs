fn fibonacci(n: u32) -> u32 {
    // Handle base cases
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // Initialize first two numbers
    let mut a = 0;
    let mut b = 1;

    // Iterate n-1 times, calculating next number
    for _ in 2..=n {//The _ is used when we don't need the loop variable. ..= means "up to and including
        let next = a + b;
        a = b;
        b = next;
    }

    b
}

fn main() {
    // Example usage
    let n = 10;
    println!("Fibonacci number at position {} is: {}", n, fibonacci(n));
}