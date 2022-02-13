use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub trait Solver {
    fn new(targets: Rc<Vec<Word>>, pool: Rc<Vec<Word>>) -> Self;
    fn cull(&mut self, pattern: Pattern);
    fn update_guess(&mut self);
    fn guess(&self) -> Word;
    fn options(&self) -> usize;
}

pub fn get_word_list(name: &str) -> Option<Vec<Word>> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|word| Word::new(word.unwrap()))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Letter {
    value: u8,
}

impl Letter {
    fn from_char(c: char) -> Option<Letter> {
        if !c.is_ascii_lowercase() {
            return None;
        }
        Some(Letter {
            value: c.to_ascii_uppercase() as u8,
        })
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

    pub fn fits_pattern(&self, guess: Word, pattern: Pattern) -> bool {
        pattern == Pattern::calculate(guess, *self)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_iter(self.letters.map(|letter| letter.to_char())))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pattern {
    value: u8,
}

impl Pattern {
    pub fn new(input: &str) -> Option<Pattern> {
        if input.chars().count() != 5 {
            return None;
        }
        let mut value = 0;
        let mut multiplier = 1;
        for char in input.chars() {
            value += match char {
                'B' | 'b' => 0,
                'Y' | 'y' => multiplier,
                'G' | 'g' => multiplier * 2,
                _ => return None,
            };
            multiplier *= 3;
        }
        Some(Pattern { value })
    }

    pub fn calculate(guess: Word, target: Word) -> Pattern {
        let mut value = 0;
        let mut multiplier = 1;
        let mut used = [false; 5];
        for i in 0..5 {
            if guess.letters[i] == target.letters[i] {
                value += multiplier * 2;
            } else {
                for j in 0..5 {
                    if i != j
                        && guess.letters[j] != target.letters[j]
                        && guess.letters[i] == target.letters[j]
                        && !used[j]
                    {
                        value += multiplier;
                        used[j] = true;
                        break;
                    }
                }
            }
            multiplier *= 3;
        }
        Pattern { value }
    }

    pub fn index(&self) -> usize {
        self.value as usize
    }
}
