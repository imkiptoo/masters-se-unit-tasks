use std::collections::HashMap;
use std::io;

pub(crate) fn validate_password() {
    let mut policy = HashMap::from([
        ("length", false),
        ("uppercase", false),
        ("lowercase", false),
        ("digit", false),
        ("special_character", false),
    ]);

    println!("Please enter your password:");

    // Read password input
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password.");
    let password = password.trim(); // Trim any trailing newline

    // Check each policy requirement
    if password.len() >= 8 {
        policy.insert("length", true);
    }

    for character in password.chars() {
        if character.is_ascii_uppercase() {
            policy.insert("uppercase", true);
        }
        if character.is_ascii_lowercase() {
            policy.insert("lowercase", true);
        }
        if character.is_ascii_digit() {
            policy.insert("digit", true);
        }
        if "!*".contains(character) {
            policy.insert("special_character", true);
        }
    }

    // Report unmet policies
    let unmet_policies: Vec<_> = policy
        .iter()
        .filter(|&(_, &is_met)| !is_met)
        .map(|(&policy_name, _)| policy_name)
        .collect();

    if unmet_policies.is_empty() {
        println!("Password is valid and meets all policy requirements.");
    } else {
        println!("Password does not meet the following policies: {:?}", unmet_policies);
    }
}
