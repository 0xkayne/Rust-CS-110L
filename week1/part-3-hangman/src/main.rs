// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].

    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut correct_letters: Vec<(char, i32)> = Vec::new();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut incorrect_guesses = 0;
    println!("Welcome to CS110L Hangman!");
    loop {
        if incorrect_guesses == NUM_INCORRECT_GUESSES {
            println!("Sorry, you ran out of guesses!");
            break;
        }

        let mut word_now_chars: Vec<char> = vec!['-'; secret_word_chars.len()];
        for i in 0..correct_letters.len() {
            let (letter, index) = correct_letters[i];
            word_now_chars[index as usize] = letter;
        }
        let word_now: String = word_now_chars.iter().collect();
        if word_now == secret_word {
            println!(
                "Congratulations you guessed the secret word: {}!",
                secret_word
            );
            break;
        }
        println!("The word so far is {}", word_now);
        println!("{}", word_now);
        let guessed_string: String = guessed_letters.iter().collect();
        println!("You have guessed the following letters: {}", guessed_string);
        println!(
            "You have {} guesses left",
            NUM_INCORRECT_GUESSES - incorrect_guesses
        );
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let mut guess = guess.trim().chars().next().unwrap();
        guessed_letters.push(guess);
        let mut flag = false;
        for i in 0..secret_word_chars.len() {
            if secret_word_chars[i] == guess {
                flag = true;
                correct_letters.push((guess, i as i32));
            }
        }
        if !flag {
            println!("Sorry, that letter is not in the word");
            incorrect_guesses += 1;
        } else {
            println!("You guessed a letter in the word!");
        }
        println!("");
    }
}
