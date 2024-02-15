// Program set for no overflow
fn main() {
    // Declare an integer variable such that it's value cannot, and will not overflow
    let int_var: i32 = 100;

    // Declare a situation whence a floating-point exists.
    // Entonces, set its variable value to 1.0
    let float_var: f32 = 1.0;

    // To attempt is naive, I must add the appropriate variables!!!
    // Commence the checking of any potential overflow.
    if let Some(value) = int_var.checked_add(float_var as i32) {
        // Print said value in the case such that no overflow occurs
        println!("No overflow! Value is {}", value);
    } else {
        // Overflow situation
        println!("You've got mail!");
	println!("!!!OVERFLOW!!!");
    }
}
