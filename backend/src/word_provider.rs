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

    for (i, g_char) in guess.chars().enumerate() {
        if word.contains(g_char) {
            map.insert(i as i8, g_char);
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

pub fn remove_duplicates(same_letters: &mut HashMap<i8, char>, word: &str) {
    let mut occurences = HashMap::<char, i8>::new();

    for &value in same_letters.values() {
        *occurences.entry(value).or_insert(0) += 1;
    }

    for (key, value) in occurences.iter() {

        if *value > 1 {
            let mut count = 0;
            for c in word.chars() {
                if c == *key {
                    count += 1;
                }
            }

            if count < *value {
                // remove the key from the HashMap which has not the same key as the position in
                // the word if both letters are not at the right position delete the one with the
                // higher index

                // count how often the letter is in the right place in the word
                let mut occurence_guess = *value;
                let mut occurence_word = count;
                // find the indexes of the letter which are in the correct place in the word
                let mut remove_keys: Vec<i8> = Vec::new();
                for (key_2, value_2) in same_letters.iter() {
                    // this should also equal to the key 
                    let char = word.chars().nth(*key_2 as usize);
                    if char == Some(*value_2) && char == Some(*key) {
                        occurence_guess -= 1;
                        occurence_word -= 1;
                        remove_keys.push(*key_2);
                    }
                }

                // add the from higher to lower the indexes which have the key as the value until
                // occurence_word == occurence_guess add the indexes to the remove_keys list
                let mut i = word.len() as i8 - 1;
                while occurence_word != occurence_guess && i >= 0 {
                    if let Some(value) = same_letters.get(&i) {
                        if word.chars().nth(i as usize) == Some(*value) {
                            occurence_word -= 1;
                        } else {
                            occurence_guess -= 1;
                        }
                        remove_keys.push(i);
                    }
                    i -= 1;
                }

                // remove the keys from the map
                same_letters.retain(|key, _| !remove_keys.contains(key));
            }
        }
    }

    let keys_to_remove: Vec<i8> = same_letters.iter()
        .filter_map(|(&key, &value)| {
            if (key as usize) < word.len() && word.chars().nth(key as usize) == Some(value) {
                Some(key)
            } else {
                None
            }
        })
        .collect();

    // Remove the keys from the HashMap
    same_letters.retain(|key, _| !keys_to_remove.contains(key));
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
    fn test_find_not_containe() {
        let word = "test";
        let guess = "teddy";
        let result = find_not_containe(word, guess);
        let vec = vec!['d', 'd', 'y'];
        assert_eq!(result, vec);
    }

    #[test]
    fn test_remove_duplicates1() {
        let word = "dance";
        let guess = "danca";
        let mut result = find_same_letters(word, guess);
        remove_duplicates(&mut result, word);

        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_duplicates2() {
        let word = "pleat";
        let guess = "treat";
        let mut result = find_same_letters(word, guess);
        remove_duplicates(&mut result, word);

        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_duplicates3() {
        let word = "test";
        let guess = "tett";
        let mut result = find_same_letters(word, guess);
        remove_duplicates(&mut result, word);

        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_duplicates4() {
        let word =  "yayayay";
        let guess = "ayayyay";
        let mut result = find_same_letters(word, guess);
        remove_duplicates(&mut result, word);
        let mut map = HashMap::new();
        map.insert(0, 'a');
        map.insert(1, 'y');
        map.insert(2, 'a');
        map.insert(3, 'y');

        assert_eq!(result, map);
    }

    #[test]
    fn test_remove_duplicates5() {
        let word =  "taat";
        let guess = "atta";
        let mut result = find_same_letters(word, guess);
        remove_duplicates(&mut result, word);
        let mut map = HashMap::new();
        map.insert(0, 'a');
        map.insert(1, 't');
        map.insert(2, 't');
        map.insert(3, 'a');

        assert_eq!(result, map);
    }

    #[test]
    fn test_remove_duplicates6() {
        let word =  "taatz";
        let guess = "attaa";
        let mut result = find_same_letters(word, guess);
        remove_duplicates(&mut result, word);
        let mut map = HashMap::new();
        map.insert(0, 'a');
        map.insert(1, 't');
        map.insert(2, 't');
        map.insert(3, 'a');

        assert_eq!(result, map);
    }
}
