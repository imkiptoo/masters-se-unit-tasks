use std::io::stdin;

pub(crate) fn encryption() {
    let mut clear_text: String = String::new();

    println!("Enter the password to encrypt: ");
    stdin().read_line(&mut clear_text).expect("Failed to read input");
    clear_text = clear_text.trim().to_string();

    let mut shifting_key: String = String::new();

    println!("Enter the shifting key (number): ");
    stdin().read_line(&mut shifting_key).expect("Failed to read input");
    shifting_key = shifting_key.trim().to_string();
    let shifting_key: u8 = match shifting_key.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };

    let mut encrypted_text = String::new();

    for character in clear_text.chars() {
        let character = character as u8;

        let shifted_char = if character.is_ascii_lowercase() {
            // Shift the character and wrap around within 'a' to 'z'
            (((character - b'a' + shifting_key) % 26) + b'a') as char
        } else if character.is_ascii_uppercase() {
            // Shift the character and wrap around within 'a' to 'z'
            (((character - b'A' + shifting_key) % 26) + b'A') as char
        } else {
            character as char
        };
        encrypted_text.push(shifted_char);
    }

    println!("Cleartext Value is: {}", clear_text);
    println!("Encrypted Value is: {}", encrypted_text);
}
