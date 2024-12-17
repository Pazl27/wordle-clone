use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use crate::router::Valid;

pub fn get_word() -> Result<String, io::Error> {
    let words = read_file()?;
    let word = get_random_word(words);
    Ok(word)
}

fn read_file() -> Result<Vec<String>, io::Error> {
    let path = Path::new("resources/words.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let words: Result<Vec<String>, io::Error> = reader.lines().collect();

    words
}

fn get_random_word(words: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    words[index].clone()
}

pub fn is_right_word(word: &str, guess: &str) -> Valid {
    if word == guess {
        Valid::Pass
    } else {
        Valid::Fail
    }
}

pub fn find_same_letters(word: &str, guess: &str) -> HashMap<i8, char> {
    let mut map = HashMap::new();
    let mut letter_counts = HashMap::new();

    for c in word.chars() {
        *letter_counts.entry(c).or_insert(0) += 1;
    }

    let mut added_counts = HashMap::new();

    for (i, g_char) in guess.chars().enumerate() {
        if word.contains(g_char) {
            let count = added_counts.entry(g_char).or_insert(0);

            if *count < *letter_counts.get(&g_char).unwrap_or(&0) {
                map.insert(i as i8, g_char);
                *count += 1; // Increment the added count for this character
            }
        }
    }

    map
}

pub fn find_right_place(word: &str, guess: &str) -> HashMap<i8, char> {
    let mut map = HashMap::new();

    for (i, g_char) in guess.chars().enumerate() {
        if word.chars().nth(i) == Some(g_char) {
            map.insert(i as i8, g_char);
        } 
    }
    map
}
