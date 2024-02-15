// Function for fizz buzz baz
// Cannot use modulo in implementation
fn main() {
    // Mutable values for fizz buzz baz
    // Checks their value, then concatenates/pushes
    // the expected string as output for each line
        let mut fizz = 0;
        let mut buzz = 0;
        let mut baz = 0;
        // Iterates from 1-->100, adds 1 to each
             for i in 1..=100 {
                 fizz += 1;
                 buzz += 1;
                 baz += 1;
                // Checking value of each
                 let mut output = String::new();
                 if fizz == 3 {
                     output.push_str("Fizz");
                     fizz = 0;
                 }
                 if buzz == 5 {
                     output.push_str("Buzz");
                     buzz = 0;
                 }
                 if baz == 7 {
                     output.push_str("Baz");
                     baz = 0;
                 }
                 if output.is_empty() {
                     output = i.to_string();
                 }
                 // Concatenation of output
                 println!("{}", output);
             }
            }