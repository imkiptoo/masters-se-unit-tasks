use std::fmt::Write; // For formatting the string in a mutable manner

// Function to calculate the IBAN checksum using standard library
fn generate_iban_checksum(bank_code: &str, account_number: &str) -> String {
    // Step 1: Concatenate the bank code, account number, and country code with placeholder checksum "131400"
    let combined = format!("{}{}131400", bank_code, account_number);

    // Step 2: Convert the combined string to a numerical string by replacing letters with digits
    let mut numeric_string = String::new();
    for c in combined.chars() {
        if c.is_ascii_digit() {
            numeric_string.push(c);
        } else if c.is_ascii_alphabetic() {
            let numeric_value = (c.to_ascii_uppercase() as u32 - 'A' as u32 + 10).to_string();
            numeric_string.push_str(&numeric_value);
        }
    }

    // Step 3: Calculate the checksum by iteratively taking chunks and performing mod 97
    let mut remainder = 0;
    for chunk in numeric_string.as_bytes().chunks(9) {
        // Convert the current remainder concatenated with the chunk to an integer
        let part = format!("{}{}", remainder, std::str::from_utf8(chunk).unwrap());
        remainder = part.parse::<u32>().unwrap() % 97;
    }

    // Step 4: Calculate the final checksum (98 - remainder)
    let checksum = 98 - remainder;

    // Step 5: Format the checksum with two digits
    format!("{:02}", checksum)
}

// Function to generate the full German IBAN
pub(crate) fn generate_german_iban(bank_code: &str, account_number: &str) {
    // Generate the checksum
    let checksum = generate_iban_checksum(bank_code, account_number);

    // Format the IBAN in the form: DE<checksum><bank_code><account_number>
    let generated_iban = format!("DE{}{}{}", checksum, bank_code, account_number);

    println!("Generated IBAN: {}", generated_iban);
}
