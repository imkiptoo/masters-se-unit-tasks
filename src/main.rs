mod the_evil_nine;
mod can_divide_array;
mod happy_numbers;
mod pangram;

use std::collections::HashSet;
use std::io;
use colored::*;

fn main() {
    println!("Please Select a Task:\n\
    1. Task I   - Can Divide Array\n\
    2. Task II  - The Evil Nine\n\
    3. Task III - Happy Numbers\n\
    4. Task IV  - Pangram\n");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");

    // Parse the input as an integer
    match task.trim().parse::<u32>() {
        Ok(1) => {
            println!("Task I : \"Can Divide Array\" Selected");
            can_divide_array::can_divide_array(); // Call your function
        }
        Ok(2) => {
            println!("The Evil Nine task selected.");
            the_evil_nine::the_evil_nine();
        }
        Ok(3) => {
            println!("Happy Numbers task selected.");
            happy_numbers::find_happy_number();
        }
        Ok(4) => {
            println!("Pangram task selected.");
            pangram::pangram();
        }
        _ => {
            println!("{}", "Please input a valid number!".red());
        }
    }
}
