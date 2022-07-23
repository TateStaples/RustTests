use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use crate::tic_tac_toe::user_input;

pub(crate) fn main() {
    let mut g = Game::random();
    while !g.over {
        g.recap();
        let guess = user_input(String::from("What is your guess: "));
        g.guess(guess)
    }
    println!("You win!")
}


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct YellowGuess(usize, char);
struct Game {
    word: String,
    dictionary: Vec<String>,
    missing: HashSet<char>,
    yellow: Vec<YellowGuess>,
    correct: String,
    previous_guesses: Vec<String>,
    over: bool

} impl Game {
    fn word_freq(filename: String) -> HashMap<String, f32> {
        let mut map: HashMap<String, f32> = HashMap::new();

        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename)
            .expect("Something went wrong reading the freq file");
        let reader = BufReader::new(file);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for line in reader.lines() {
            if line.is_err() {continue}
            let line = line.unwrap();
            let mut pieces = line.split_whitespace();
            let word = pieces.next()
                .expect("word not found in line");
            let mut sum: f32 = 0.0;
            for i in 0..5usize {
                sum += pieces.nth_back(i)
                    .expect("failed to find freq on line")
                    .parse::<f32>()
                    .expect("parsing of float freq failed");
            }
            let mean_freq = sum / 5.0;
            map.insert(word.to_string(), mean_freq);
        }
        map
    }
    fn dictionary(filename: String) -> Vec<String> {
        let file = File::open(filename)
            .expect("Something went wrong reading the dictionary file");
        let reader = BufReader::new(file);
        reader.lines().filter_map(|l|l.ok()).collect()
    }
    fn random() -> Box<Game> {
        let dictionary = Game::dictionary(String::from("src/dictionary.txt"));
        let word = dictionary
            .choose(&mut rand::thread_rng())
            .expect("failed to get random word from dictionary")
            .to_string();
        Box::new(Game {
            word,
            dictionary,
            missing: HashSet::new(),
            yellow: Vec::new(),
            correct: "_____".to_string(),
            previous_guesses: Vec::new(),
            over: false
        })
    }
    fn test(word: String) -> Box<Game> {
        Box::new(Game {
            word,
            dictionary: Game::dictionary(String::from("src/dictionary.txt")),
            missing: HashSet::new(),
            yellow: Vec::new(),
            correct: "_____".to_string(),
            previous_guesses: Vec::new(),
            over: false
        })
    }
    fn guess(&mut self, guess: String) {
        if guess.len() != 5 {
            println!("Warnging: {} is not 5 letters", guess);
            return
        }
        if !self.dictionary.contains(&guess) {
            println!("Invalid word: {} is not in our dictionary", guess);
            return
        }

        self.previous_guesses.push(guess.to_string());
        // game over
        if guess == self.word {
            self.over = true;
            self.correct = guess;
            return
        }
        let mut correct_iter = self.correct.chars();
        let mut answer_iter = self.word.chars();
        let mut new_correct = String::new();
        let mut reserved: [bool; 5] = [false; 5];
        for (index, letter) in guess.chars().enumerate() {
            // correct guess
            let answer_char = answer_iter.next().expect("failed to open pattern");
            let pattern_char = correct_iter.next().expect("failed to open pattern");
            println!("Check: guess-{}, was-{}", letter, answer_char);
            if answer_char == letter {
                new_correct.push(letter);
                reserved[index] = true;
            }
            else {
                println!("{}, {}", self.correct, index);
                new_correct.push(pattern_char);
                // yellow (edit for double behavior)
                if self.word.chars()
                    .enumerate()
                    .any(|(i, c)| c==letter && !reserved[index]) {
                    self.yellow.push(YellowGuess(index, letter));
                }

                else {
                    self.missing.insert(letter);
                }
            }
        }
        self.correct = new_correct;
        if self.correct == self.word {
            self.over = true
        }
    }
    fn recap(&self) {
        // previous guess
        println!("Previous guesses:");
        for guess in &self.previous_guesses {
            println!("{}", guess);
        }
        // wrong letters
        println!("Wrong letters: {:?}", self.missing.iter());
        // yellow letters
        println!("Misplaced letters: {:?}", self.yellow
            .iter()
            .filter(|y| self.correct.chars().nth(y.0).unwrap() != '_')
        );
        // correct pattern
        println!("Correct pattern: {}", self.correct);
    }
}

fn sigmoid(x: f32) -> f32{
    1.0 / (1.0+f32::exp(-x))
}
struct Solver {
    game: Box<Game>,
    freq: HashMap<String, f32>,
    ply: u8
} impl Solver {
    fn new() -> Solver {
        Solver {
            game: Game::random(),
            freq: Game::word_freq(String::from("src/wordle_words_freqs_full.txt")),
            ply: 1
        }
    }

}