use std::fs::File;
use std::io::{self, BufRead, BufReader};

use words::{Rule, Word};

fn main() {
    println!("Output from Wordle formatted with 0 - black, 1 - gold, 2 - green e.g. 12001");
    println!("Hard mode? (type \"y\" if so)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let hard = input.trim() == "y";
    let mut words = all_words("words.txt");
    let all_words = words.clone();
    let mut guess = Word::new(String::from("tares"));
    while words.len() > 1 {
        if words.len() < 10 {
            println!("Guess \"{}\" ({} options: {:?})", guess, words.len(), display_words(&words));
        } else {
            println!("Guess \"{}\" ({} options)", guess, words.len());
        }
        loop {
            println!("What output did Wordle give you?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if let Some(rule) = Rule::new(guess, input.trim()) {
                words.retain(|word| word.fits_rule(&rule));
                match words.len() {
                    0 => println!("No options remain."),
                    1 => println!("Guess \"{}\" (1 option)", words[0]),
                    _ => guess = best_guess(&words, if hard { &words } else { &all_words }),
                }
                break;
            }
            println!("Invalid input.");
        }
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn best_guess(words: &Vec<Word>, pool: &Vec<Word>) -> Word {
    let mut lowest = (Word::new(String::from("     ")), usize::MAX);
    let mut lowest_remaining = (Word::new(String::from("     ")), usize::MAX);
    for &guess in pool {
        let mut score = 0;
        for &target in words {
            let rule = Rule::calculate(guess, target);
            for &word in words {
                if word.fits_rule(&rule) {
                    score += 1;
                }
            }
        }
        if score < lowest.1 {
            lowest = (guess, score);
        }
        if score < lowest_remaining.1 && words.contains(&guess) {
            lowest_remaining = (guess, score);
        }
    }
    if lowest_remaining.1 <= lowest.1 {
        lowest_remaining.0
    } else {
        lowest.0
    }
}

fn display_words(words: &Vec<Word>) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

fn all_words(name: &str) -> Vec<Word> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|word| Word::new(word.unwrap()))
        .collect()
}

mod words {
    use std::fmt::Display;
    use Colour::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Word {
        letters: [char; 5],
    }

    impl Word {
        pub fn new(word: String) -> Word {
            let mut chars = word.chars();
            Word {
                letters: [
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                ],
            }
        }

        pub fn fits_rule(&self, rule: &Rule) -> bool {
            *rule == Rule::calculate(rule.guess, *self)
        }
    }

    impl Display for Word {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", String::from_iter(self.letters))
        }
    }

    #[derive(PartialEq, Eq)]
    pub struct Rule {
        guess: Word,
        colours: [Colour; 5],
    }

    impl Rule {
        pub fn new(guess: Word, values: &str) -> Option<Rule> {
            if values.chars().count() != 5 {
                return None;
            }
            let mut colours = [Black; 5];
            for (i, colour) in values.chars().enumerate() {
                colours[i] = match colour {
                    '0' => Black,
                    '1' => Gold,
                    '2' => Green,
                    _ => return None,
                };
            }
            Some(Rule { guess, colours })
        }

        pub fn calculate(guess: Word, target: Word) -> Rule {
            let mut colours = [Black; 5];
            let mut used = [false; 5];
            for i in 0..5 {
                if guess.letters[i] == target.letters[i] {
                    colours[i] = Green;
                } else {
                    for j in 0..5 {
                        if i != j
                            && guess.letters[j] != target.letters[j]
                            && guess.letters[i] == target.letters[j]
                            && !used[j]
                        {
                            colours[i] = Gold;
                            used[j] = true;
                            break;
                        }
                    }
                }
            }
            Rule { guess, colours }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Colour {
        Green,
        Gold,
        Black,
    }
}
