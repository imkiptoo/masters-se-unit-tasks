pub(crate) fn email_check() {
    let mut email_address: String = String::new();

    println!("Please enter an email address to validate: ");

    std::io::stdin().read_line(&mut email_address).expect("Empty Email Address");

    let email_address = email_address.trim();

    if validate_email(email_address) {
        println!("Email {} is valid :)", email_address)
    } else {
        println!("Email {} is INVALID :(", email_address)
    }
}

fn validate_email(email_address: &str) -> bool {
    let mut count_before_at = 0;
    let mut count_after_at = 0;
    let mut count_after_dot = 0;
    let mut running_counter = 0;

    for character in email_address.chars() {
        running_counter += 1;
        if character == '@' {
            count_before_at = running_counter;
            running_counter = 0;
        }
        if character == '.' {
            count_after_at = running_counter;
            running_counter = 0;
        }
    }

    count_after_dot = running_counter;

    count_before_at > 0 && count_after_at > 0 && (count_after_dot >= 2 && count_after_dot <= 3)
}

#[test]
fn email_without_at() {
    let result = validate_email("hello");
    assert_eq!(result, false);
}