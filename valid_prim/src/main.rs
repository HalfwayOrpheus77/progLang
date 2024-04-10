// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Rust program with unit test to compute the
// following expression: (* 3 4 5 (+ 2 2))
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Defining the function such that there exists a computation
fn computables() -> i32 {
    let sum = 2 + 2;
    let product = 3 * 4 * 5 * sum;
    product
}

#[cfg(test)]
// Unit Test
mod united_testables {
    use super::*;

    // Defining function for unit test
    #[test]
    fn test_of_computables() {
        assert_eq!(computables(), 3 * 4 * 5 * (2 + 2));
    }
}

fn main() {
    println!("Computable and observable: {}", computables());
}

