use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::collections::HashMap;
use words::{Rule, Word};

fn main() {
    println!("Output from Wordle formatted with [B]lack, [Y]ellow, [G]reen");
    let mut words = get_word_list("words.txt").unwrap();
    let pool = get_word_list("guessable.txt").unwrap();
    let mut guess = Word::new(String::from("roate")).unwrap();
    while words.len() > 1 {
        println!("Guess \"{}\" ({} options)", guess, words.len());
        loop {
            println!("What output did Wordle give you?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if let Some(rule) = Rule::new(input.trim()) {
                words.retain(|word| word.fits_rule(guess, rule));
                match words.len() {
                    0 => println!("No options remain."),
                    1 => println!("The word is \"{}\"", words[0]),
                    _ => guess = best_guess(&words, &pool),
                }
                break;
            }
            println!("Invalid input.");
        }
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn best_guess(words: &[Word], pool: &[Word]) -> Word {
    let mut lowest = (pool[0], usize::MAX);
    for &guess in pool {
        let mut rules = HashMap::new();
        for &word in words {
            *rules.entry(Rule::calculate(guess, word)).or_insert(0) += 1;
        }
        let score = rules.into_values().map(|count| count * count).sum();
        if score < lowest.1 || (score == lowest.1 && words.contains(&guess) && !words.contains(&lowest.0))
        {
            lowest = (guess, score);
        }
    }
    lowest.0
}

fn get_word_list(name: &str) -> Option<Vec<Word>> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|word| Word::new(word.unwrap()))
        .collect()
}

mod words {
    use std::fmt::Display;

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Letter {
        value: u8,
    }

    impl Letter {
        fn from_char(c: char) -> Option<Letter> {
            if !c.is_ascii_lowercase() {
                return None;
            }
            Some(Letter { value: c.to_ascii_uppercase() as u8 })
        }

        fn to_char(&self) -> char {
            self.value as char
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Word {
        letters: [Letter; 5],
    }

    impl Word {
        pub fn new(word: String) -> Option<Word> {
            if word.chars().count() != 5 {
                return None;
            }
            let mut chars = word.chars();
            Some(Word {
                letters: [
                    Letter::from_char(chars.next().unwrap())?,
                    Letter::from_char(chars.next().unwrap())?,
                    Letter::from_char(chars.next().unwrap())?,
                    Letter::from_char(chars.next().unwrap())?,
                    Letter::from_char(chars.next().unwrap())?,
                ],
            })
        }

        pub fn fits_rule(&self, guess: Word, rule: Rule) -> bool {
            rule == Rule::calculate(guess, *self)
        }
    }

    impl Display for Word {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", String::from_iter(self.letters.map(|letter| letter.to_char())))
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Rule {
        colours: [Colour; 5],
    }

    impl Rule {
        pub fn new(values: &str) -> Option<Rule> {
            if values.chars().count() != 5 {
                return None;
            }
            let mut colours = [Colour::Black; 5];
            for (i, colour) in values.chars().enumerate() {
                colours[i] = match colour {
                    'B' | 'b' => Colour::Black,
                    'Y' | 'y' => Colour::Yellow,
                    'G' | 'g' => Colour::Green,
                    _ => return None,
                };
            }
            Some(Rule { colours })
        }

        pub fn calculate(guess: Word, target: Word) -> Rule {
            let mut colours = [Colour::Black; 5];
            let mut used = [false; 5];
            for i in 0..5 {
                if guess.letters[i] == target.letters[i] {
                    colours[i] = Colour::Green;
                } else {
                    for j in 0..5 {
                        if i != j
                            && guess.letters[j] != target.letters[j]
                            && guess.letters[i] == target.letters[j]
                            && !used[j]
                        {
                            colours[i] = Colour::Yellow;
                            used[j] = true;
                            break;
                        }
                    }
                }
            }
            Rule { colours }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum Colour {
        Black,
        Yellow,
        Green,
    }
}
