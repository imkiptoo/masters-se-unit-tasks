use std::io;
use colored::*;

fn main() {
    println!("Please Select a Task:\n\
    1. Task I   - Can Divide Array\n\
    2. Task II  - The Evil Nine\n\
    3. Task III - Happy Numbers\n");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");

    // Parse the input as an integer
    match task.trim().parse::<u32>() {
        Ok(1) => {
            println!("Task I : \"Can Divide Array\" Selected");
            can_divide_array(); // Call your function
        },
        Ok(2) => {
            println!("The Evil Nine task selected.");
            the_evil_nine();
        },
        Ok(3) => {
            println!("Happy Numbers task selected.");
            // Add logic for Task III
        },
        _ => {
            println!("{}", "Please input a valid number!".red());
        },
    }
}

fn can_divide_array() {
    let mut divisible = false;

    let array: Vec<i32> = vec![1, 1, 3];

    let array_length = array.len();

    // Ensure array has an even number of elements
    if array_length % 2 != 0 {
        divisible = false
    } else {
        // Split the array into left and right sides
        let (left_side, right_side) = array.split_at(array_length / 2);

        // Sum both sides and compare
        divisible = left_side.iter().sum::<i32>() == right_side.iter().sum::<i32>()
    }

    if divisible {
        println!("array is equal on both sides");
    } else {
        println!("array is not equal on both sides");
    }
}

fn the_evil_nine(){

}
