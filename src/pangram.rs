use std::collections::HashSet;
use std::io;

fn check_if_pangram(mut sentence: String) -> bool {
    let mut characters: HashSet<char> = HashSet::new();

    sentence = sentence.to_lowercase();

    if sentence.is_empty() {
        return false;
    }

    sentence.chars().for_each(|c| {
        if c > 'a' || c < 'z' {
            characters.insert(c);
        }
    });

    if characters.is_empty() || characters.len() < 26 {
        return false;
    }

    true
}

pub(crate) fn pangram() {
    println!("Enter your word:");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    word = word.trim().to_string();

    if check_if_pangram(word.to_string()) {
        println!("{} is a Pangram :)", word);
    } else {
        println!("{} is NOT A PANGRAM :(", word);
    }
}