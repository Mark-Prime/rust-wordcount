use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    if env::args().len() <= 1{
        println!("Program requires at least 1 argument.");
        return;
    };

    let input = match fs::read_to_string(env::args().nth(1).unwrap()) {
        Ok(input) => input,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut word_counts = HashMap::new();

    let mut word = String::new();
    for mut char in input.chars() {
        char = char.to_lowercase().next().unwrap();

        if char.is_alphabetic() {
            word.push(char);
            continue;
        }

        if char == '\'' {
            word.push(char);
            continue;
        }

        if word.len() > 0 {
            let count = word_counts.entry(word.clone()).or_insert(0);
            *count += 1;
            word.clear();
        }
    }

    let return_count: i32 = match env::args().nth(2) {
        Some(input) => match input.parse::<i32>() {
            Ok(count) => count,
            Err(_) => {
                println!("Error: {} is not a valid number.", input);
                10
            }
        },
        None => {
            println!("Error: {} is not a valid input.", input);
            10
        },
    };

    let mut sorted_word_counts = word_counts.iter().collect::<Vec<_>>();
    sorted_word_counts.sort_by(|a, b| b.1.cmp(a.1));

    for index in 0..return_count as usize {
        println!("{} - {}", sorted_word_counts[index].0, sorted_word_counts[index].1);
    }
}
