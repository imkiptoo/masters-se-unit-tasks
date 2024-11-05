mod the_evil_nine;
mod can_divide_array;
mod happy_numbers;
mod pangram;
mod iban;
mod place_swap;
mod minimal_distance;
mod minimum_point_distance;
mod strong_password;
mod email_check;
mod encryption;

use colored::*;
use std::io;
use crate::can_divide_array::can_divide_array;
use crate::email_check::email_check;
use crate::encryption::encryption;
use crate::happy_numbers::find_happy_number;
use crate::iban::generate_german_iban;
use crate::minimal_distance::minimal_distance;
use crate::minimum_point_distance::minimum_point_distance;
use crate::pangram::pangram;
use crate::place_swap::reverse_array;
use crate::strong_password::validate_password;
use crate::the_evil_nine::the_evil_nine;

fn main() {
    println!("Please Select a Task:\n\
    1.  Task I       :   Can Divide Array\n\
    2.  Task II      :   The Evil Nine\n\
    3.  Task III     :   Happy Numbers\n\
    4.  Task IV      :   Pangram\n\
    5.  Task V       :   IBAN \n\
    6.  Task VI      :   Array Place Swap\n\
    7.  Task VII     :   Minimal Distance\n\
    8.  Task VIII    :   Minimum Distance Between Two Points\n\
    9.  Task IX      :   Strong Password\n\
    10. Task X       :   Email Address Check\n\
    11. Task XI      :   Encryption\n\
    12. Task XII     :   Twitter Wall\n");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");

    // Parse the input as an integer
    match task.trim().parse::<u32>() {
        Ok(1) => {
            println!("Task I : \"Can Divide Array\" Selected");
            can_divide_array(); // Call your function
        }
        Ok(2) => {
            println!("The Evil Nine task selected.");
            the_evil_nine();
        }
        Ok(3) => {
            println!("Happy Numbers task selected.");
            find_happy_number();
        }
        Ok(4) => {
            println!("Pangram task selected.");
            pangram();
        }
        Ok(5) => {
            let bank_code = "70090100";
            let account_number = "1234567890";
            generate_german_iban(bank_code, account_number);
        }
        Ok(6) => {
            let mut arr = [1, 2, 3, 4, 5];
            println!("Original array: {:?}", arr);

            reverse_array(&mut arr);
            println!("Reversed array: {:?}", arr);
        }
        Ok(7) => {
            minimal_distance();
        }
        Ok(8) => {
            minimum_point_distance();
        }
        Ok(9) => {
            validate_password();
        }
        Ok(10) => {
            email_check();
        }
        Ok(11) => {
            encryption();
        }
        _ => {
            println!("{}", "Please input a valid number!".red());
        }
    }
}
