use std::io;

fn square_digits(mut number: u128) -> u128 {
    let mut sum_of_digits_squared = 0;

    while number > 0 {
        let digit = number % 10;
        sum_of_digits_squared += digit * digit;
        number /= 10;
    }

    sum_of_digits_squared
}


pub(crate) fn find_happy_number() {
    println!("Please enter your number:");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: u128 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    loop_until_happiness_determined(user_input, &mut Vec::new())
}

fn loop_until_happiness_determined(number: u128, previous_sums: &mut Vec<u128>){
    let mut current_number = number;

    loop {
        let sum_of_digits_squared = square_digits(current_number);

        if sum_of_digits_squared == 1 {
            println!("The Number: {} is HAPPY :)", number);
            println!("Process took {} iteration(s)", previous_sums.len() + 1);
            break;
        } else if previous_sums.contains(&sum_of_digits_squared) {
            println!("The Number: {} is UNHAPPY :(", number);
            println!("Process took {} iteration(s)", previous_sums.len() + 1);

            print!("Previous Sums: ");
            println!("{:?}, ", previous_sums);

            break;
        } else {
            previous_sums.push(sum_of_digits_squared);
            current_number = sum_of_digits_squared;
        }
    }
}
