// Function to get square of number
fn square(x: i32) -> i32 {
    x * x
}

// Function to sum squares of two numbers
fn sum_of_squares(x: i32, y: i32) -> i32 {
    square(x) + square(y)
}

// Function to get FizzBuzz of the Buzz Fizz Fizz Buzz Fizzinator Buzzitrator
fn fizz_buzz() {
    for i in 1..=100 {
// Use modulo for the Fizz Fizz Buzz ShowBizz
        match (i % 3, i % 5) {
// Empty string if modulo does not work
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    // Call sum_of_squares function
    let a = 5;
    let result = sum_of_squares(a+1, a*2);
    println!("Sum of squares: {}", result);

    // Call FizzBuzz function
    fizz_buzz();
}