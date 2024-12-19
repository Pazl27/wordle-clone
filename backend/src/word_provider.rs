use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                *count += 1;
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

pub fn find_not_containe(word: &str, guess: &str) -> Vec<char> {
    guess.chars().filter(|c| !word.contains(*c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_word() {
        let words = vec!["test".to_string(), "test".to_string()];
        let word = get_random_word(words);
        assert_eq!(word, "test".to_string());
    }

    #[test]
    fn test_is_right_word() {
        let word = "test";
        let guess = "test";
        let result = is_right_word(word, guess);
        assert_eq!(result, Valid::Pass);
    }

    #[test]
    fn test_find_same_letters() {
        let word = "test";
        let guess = "tset";
        let result = find_same_letters(word, guess);
        let mut map = HashMap::new();
        map.insert(0, 't');
        map.insert(1, 's');
        map.insert(2, 'e');
        map.insert(3, 't');
        assert_eq!(result, map);
    }

    #[test]
    fn test_find_right_place() {
        let word = "test";
        let guess = "tset";
        let result = find_right_place(word, guess);
        let mut map = HashMap::new();
        map.insert(0, 't');
        map.insert(3, 't');
        assert_eq!(result, map);
    }

    #[test]
    fn test_edge_case_find_same_letters1() {
        let word = "test";
        let guess = "tett";
        let result = find_same_letters(word, guess);
        let mut map = HashMap::new();
        map.insert(2, 't');
        map.insert(0, 't');
        map.insert(1, 'e');

        assert_eq!(result, map);
    }

    #[test]
    fn test_edge_case_find_same_letters2() {
        let word = "dance";
        let guess = "teddy";
        let result = find_same_letters(word, guess);
        let mut map = HashMap::new();
        map.insert(1, 'e');
        map.insert(2, 'd');

        assert_eq!(result, map);
    }

    #[test]
    fn test_find_not_containe() {
        let word = "test";
        let guess = "teddy";
        let result = find_not_containe(word, guess);
        let mut vec = Vec::new();
        vec.push('d');
        vec.push('d');
        vec.push('y');
        assert_eq!(result, vec);
    }
}
