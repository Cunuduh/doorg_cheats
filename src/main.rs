mod auto_send;
use std::io::*;
use std::fs::File;
use rand::Rng;
fn main() {
    let dictionary = parse_dictionary();
    println!("Welcome to the Black Tea and Create a Word program.");
    println!("Enter a string of letters to find a word that contains the letters.");
    println!("Or enter a letter and a number to find a word that starts with the letter and has the length of the number. Example: A5");
    println!("Enter _quit to quit.");
    println!("Enter _autosend to enter the autosend mode.");
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        if s.trim().eq_ignore_ascii_case("_quit") {
            break;
        }
        if s.trim().eq_ignore_ascii_case("_autosend") {
            auto_send::send();
            continue;
        }
        if s.trim().chars().all(|c| c.is_ascii_alphabetic()) {
            black_tea(s, &dictionary);
            continue;
        }
        create_a_word(s, &dictionary);
    }
}
fn black_tea(s: String, dictionary: &Vec<String>) {
    let mut possible_words: Vec<String> = Vec::new();
    let mut min_word_len = 0;
    for word in dictionary {
        let word_len = word.len();
        if word.contains(s.to_lowercase().as_str().trim()) {
            possible_words.push(word.to_string());
            if min_word_len == 0 {
                min_word_len = word_len;
            }
            if word_len < min_word_len {
                min_word_len = word_len;
            }
        }
    }
    if possible_words.is_empty() {
        println!("No words found.");
        return;
    }
    let words: Vec<&String> = possible_words.iter().filter(|&w| w.len() == min_word_len).collect();
    let random_word = rand::thread_rng().gen_range(0..words.len());
    let possible_word = words[random_word];
    println!("{possible_word}");
}
fn create_a_word(s: String, dictionary: &Vec<String>) {
    let word_start: char = s.as_str()[0..1].parse().unwrap();
    let word_len: usize = s.as_str()[1..].trim().parse().unwrap();
    let mut possible_words: Vec<String> = Vec::new();
    for word in dictionary {
        if word.len() == word_len && word.to_uppercase().starts_with(word_start.to_ascii_uppercase()) {
            possible_words.push(word.to_string());
        }
    }
    if possible_words.is_empty() {
        println!("No words found.");
        return;
    }
    let random_word = rand::thread_rng().gen_range(0..possible_words.len());
    let possible_word = &possible_words[random_word];
    println!("{possible_word}");
}
fn parse_dictionary() -> Vec<String> {
    let exe_dir = std::env::current_exe().unwrap();
    let mut file = File::open(format!("{}/dictionary.txt", exe_dir.parent().unwrap().display())).expect("dictionary.txt not found in the same directory as the executable");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split_whitespace().filter(|&s| s.chars().all(|c| c.is_alphabetic())).map(|s| s.to_string()).collect::<>()
}