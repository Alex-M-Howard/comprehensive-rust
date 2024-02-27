use std::time::Instant;

fn main() {
    // Fibonacci Generator
    let term = 15;
    let collatz_term = 1_000_000;
    let nested_array = [
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ];

    // Fibonacci Recursively
    let start = Instant::now();
    println!(
        "Day 1 Morning: Fibonacci - {}th Term = {} Duration: {} seconds",
        term,
        fib(term),
        (Instant::now() - start).as_secs_f64()
    );

    // Fibonacci Non-Recursively
    let start = Instant::now();
    println!(
        "Day 1 Morning: Fibonacci (Non-Recursive) - {}th Term = {} Duration: {} seconds",
        term,
        fib2(term),
        (Instant::now() - start).as_secs_f64()
    );

    // Collatz
    let start = Instant::now();
    println!(
        "Day 1 Morning: Collatz length of {} = {} Duration: {} seconds",
        collatz_term,
        collatz_length(collatz_term),
        (Instant::now() - start).as_secs_f64()
    );

    // Nested Arrays
    let start = Instant::now();
    println!(
        "Day 1 Afternoon: {:?} transposed is {:?} Duration: {} seconds",
        nested_array,
        transpose_array(nested_array),
        (Instant::now() - start).as_secs_f64()
    );
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

fn collatz_length(mut num: i32) -> u32 {
    if num < 1 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    let mut length = 0;

    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }

        length += 1;
    }

    length
}

fn transpose_array(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_array: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            new_array[i][j] = matrix[j][i];
        }
    }

    new_array
}

#[cfg(test)]
mod fibonacci_tests {
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

#[cfg(test)]
mod collatz_tests {
    use super::*;

    #[test]
    fn length_5() {
        assert_eq!(collatz_length(3), 7);
    }

    #[test]
    fn length_15() {
        assert_eq!(collatz_length(15), 17);
    }

    #[test]
    fn length_123() {
        assert_eq!(collatz_length(123), 46);
    }

    #[test]
    fn length_123456789() {
        assert_eq!(collatz_length(123456789), 177);
    }
}

#[cfg(test)]
mod transposed_array_tests {
    use super::*;

    #[test]
    fn arr_1() {
        assert_eq!(
            transpose_array([
                [1, 2, 3], //
                [4, 5, 6],
                [7, 8, 9]
            ]),
            [
                [1, 4, 7], //
                [2, 5, 8],
                [3, 6, 9]
            ]
        );
    }

    #[test]
    fn arr_2() {
        assert_eq!(
            transpose_array([
                [101, 102, 103], //
                [201, 202, 203],
                [301, 302, 303]
            ]),
            [
                [101, 201, 301], //
                [102, 202, 302],
                [103, 203, 303]
            ]
        );
    }
}
