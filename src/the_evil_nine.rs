use std::io;
pub(crate) fn the_evil_nine() {
    println!("Please enter a number to proceed, or [0] to exit: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();
    let trimmed_input_number: u32 = match trimmed_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!\n");
            the_evil_nine();
            return;
        }
    };

    if trimmed_input.len() != 0 {
        if trimmed_input_number != 0 {
            if trimmed_input.contains("9") || trimmed_input_number % 9 == 0 {
                println!("The number contains a 9, IT IS EVIL. DARTH VADER EVIL!");
                the_evil_nine();
            } else {
                println!("That is A GOOD NUMBER, may the force be with you!");
                the_evil_nine();
            }
        } else {
            println!("You entered [0], program exiting..., Goodbye!");
            return;
        }
    } else {
        return;
    }
}