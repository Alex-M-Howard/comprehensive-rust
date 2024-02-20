use std::time::Instant;

fn main() {

    // Fibonacci Generator
    let term = 46;

    // Fibonacci Recursively
    let start = Instant::now();
    println!(
        "Day 1: Fibonacci - {}th Term = {} Duration: {} seconds",
        term,
        fib(term),
        (Instant::now() - start).as_secs_f64()
    );

    // Fibonacci Non-Recursively
    let start = Instant::now();
    println!(
        "Day 1: Fibonacci (Non-Recursive) - {}th Term = {} Duration: {} seconds",
        term,
        fib2(term),
        (Instant::now() - start).as_secs_f64()
    );

    // Collatz
}

fn fib(n: u128) -> u128 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib2(n: u128) -> u128 {
    if n <= 1 {
        n
    } else {
        // let mut result: u128 = 0;
        let mut a: u128 = 0;
        let mut b: u128 = 1;

        for _ in 1..n {
            let result = a + b;
            a = b;
            b = result;
        }

        b
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn tenth_term_recursively_is_55() {
        let result = fib(10);
        assert_eq!(result, 55)
    }
    
    #[test]
    fn thirtieth_term_recursively_is_832040() {
        let result = fib(30);
        assert_eq!(result, 832040)
    }
    
    #[test]
    fn tenth_term_is_55() {
        let result = fib2(10);
        assert_eq!(result, 55)
    }
    
    #[test]
    fn thirtieth_term_is_832040() {
        let result = fib2(30);
        assert_eq!(result, 832040)
    }    
}
