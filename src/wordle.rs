use crate::solvers;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub struct Environment {
    words: Vec<WordInfo>,
    targets: Vec<u16>,
    patterns: Vec<Pattern>,
    solver: u8,
    starting_guess: u16,
}

impl Environment {
    pub fn rebuild(pool: &str, targets: &str, solver: u8) -> Option<()> {
        let start = Instant::now();

        println!("Reading input...");
        let now = Instant::now();

        let mut pool = Environment::get_word_list(pool)?;
        let target_words = Environment::parse_word_list(Environment::get_word_list(targets)?)?;

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Sorting list...");
        let now = Instant::now();

        pool.sort_unstable();

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Removing duplicates...");
        let now = Instant::now();

        let mut pool = Environment::parse_word_list(pool)?;
        pool.dedup();

        if pool.len() > u16::MAX as usize {
            return None;
        }

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Processing words...");
        let now = Instant::now();

        let mut words = Vec::with_capacity(pool.len());
        let mut targets = Vec::with_capacity(target_words.len());

        let mut i = 0;
        for (id, word) in pool.into_iter().enumerate() {
            words.push(WordInfo::new(
                word,
                if target_words.contains(&word) {
                    targets.push(id as u16);
                    i += 1;
                    Some(i - 1)
                } else {
                    None
                },
            ));
        }

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Generating patterns...");
        let now = Instant::now();

        let mut patterns = Vec::with_capacity(words.len() * targets.len());
        for word in &words {
            for &target in &targets {
                patterns.push(Pattern::calculate(word.word, words[target as usize].word));
            }
        }

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Calculating starting guess...");
        let now = Instant::now();

        let e = Environment {
            words: words.clone(),
            targets: targets.clone(),
            patterns: patterns.clone(),
            solver,
            starting_guess: 0,
        };
        let wordle = Wordle::new(&e);
        let starting_guess = wordle.next_guess();

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Forming data...");
        let now = Instant::now();

        let mut data: Vec<u8> = Vec::with_capacity(
            1 + 2 + 2 + 2 + words.len() * 7 + targets.len() * 2 + patterns.len(),
        );
        data.push(solver); // solver
        data.extend(starting_guess.to_be_bytes()); // starting guess
        data.extend((words.len() as u16).to_be_bytes()); // length of word list
        data.extend((targets.len() as u16).to_be_bytes()); // length of target list
        data.extend(words.into_iter().map(|word| word.to_bytes()).flatten()); // word list
        data.extend(targets.into_iter().map(|target| target.to_be_bytes()).flatten()); // target list
        data.extend(patterns.into_iter().map(|pattern| pattern.value)); // pattern list

        println!("Complete in {}ms", millis(now));
        println!();
        println!("Writing file...");
        let now = Instant::now();

        fs::write("saved/data.bin", data).unwrap();

        println!("Complete in {}ms", millis(now));
        println!();
        println!("All complete in {}ms", millis(start));
        println!();

        Some(())
    }

    pub fn new() -> Option<Environment> {
        let data = fs::read("saved/data.bin").ok()?;

        let solver = data[0];
        let starting_guess = u16::from_be_bytes([data[1], data[2]]);
        let words_len = u16::from_be_bytes([data[3], data[4]]) as usize;
        let targets_len = u16::from_be_bytes([data[5], data[6]]) as usize;
        let mut i = 7;
        let j = i + words_len * 7;
        let words: Vec<_> = data[i..j]
            .chunks(7)
            .map(|bytes| WordInfo::from_bytes(bytes))
            .collect();
        i = j + targets_len * 2;
        let targets: Vec<_> = data[j..i]
            .chunks(2)
            .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
            .collect();
        let patterns: Vec<_> = data[i..].iter().map(|&value| Pattern { value }).collect();

        Some(Environment {
            words,
            targets,
            patterns,
            solver,
            starting_guess,
        })
    }

    pub fn targets(&self) -> &[u16] {
        &self.targets
    }

    pub fn get_word(&self, id: u16) -> Option<Word> {
        Some(self.words.get(id as usize)?.word)
    }

    fn get_pattern(&self, guess: u16, target: u16) -> Option<Pattern> {
        Some(*self.patterns.get(
            guess as usize * self.targets.len()
                + self.words.get(target as usize)?.get_target()? as usize,
        )?)
    }

    fn get_word_list(path: &str) -> Option<Vec<String>> {
        BufReader::new(File::open(path).ok()?)
            .lines()
            .collect::<Result<_, _>>()
            .ok()
    }

    fn parse_word_list(list: Vec<String>) -> Option<Vec<Word>> {
        list.into_iter().map(|line| Word::new(&line)).collect()
    }
}

pub struct Wordle<'a> {
    e: &'a Environment,
    targets: Vec<u16>,
}

impl<'a> Wordle<'a> {
    pub fn new(e: &Environment) -> Wordle {
        Wordle {
            e,
            targets: e.targets.clone(),
        }
    }

    pub fn words(&self) -> impl IntoIterator<Item = u16> {
        0..self.e.words.len() as u16
    }

    pub fn targets(&self) -> &[u16] {
        &self.targets
    }

    pub fn get_word(&self, id: u16) -> Option<Word> {
        self.e.get_word(id)
    }

    pub fn starting_guess(&self) -> u16 {
        self.e.starting_guess
    }

    pub fn cull(&mut self, guess: u16, pattern: Pattern) {
        self.targets
            .retain(|&target| self.e.get_pattern(guess, target).unwrap() == pattern);
    }

    pub fn next_guess(&self) -> u16 {
        solvers::solver(self.e.solver).unwrap()(&self)
    }

    pub fn get_pattern(&self, guess: u16, target: u16) -> Option<Pattern> {
        self.e.get_pattern(guess, target)
    }

    pub fn only_remaining(&self) -> Option<u16> {
        if self.options() == 1 {
            return Some(self.targets[0]);
        }
        None
    }

    pub fn options(&self) -> u16 {
        self.targets.len() as u16
    }

    pub fn is_target(&self, id: u16) -> Option<bool> {
        Some(self.e.words.get(id as usize)?.is_target())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

    fn calculate(guess: Word, target: Word) -> Pattern {
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

#[derive(Clone)]
struct WordInfo {
    word: Word,
    target: u16,
}

impl WordInfo {
    fn new(word: Word, target: Option<usize>) -> WordInfo {
        WordInfo {
            word,
            target: match target {
                None => u16::MAX,
                Some(index) => index as u16,
            },
        }
    }

    fn from_bytes(bytes: &[u8]) -> WordInfo {
        WordInfo {
            word: Word::from_bytes(&bytes[0..5]),
            target: u16::from_be_bytes([bytes[5], bytes[6]]),
        }
    }

    fn to_bytes(&self) -> [u8; 7] {
        let l = self.word.letters;
        let t = self.target.to_be_bytes();
        [l[0], l[1], l[2], l[3], l[4], t[0], t[1]]
    }

    fn get_target(&self) -> Option<usize> {
        match self.target {
            u16::MAX => None,
            x => Some(x as usize),
        }
    }

    fn is_target(&self) -> bool {
        self.get_target().is_some()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Word {
    letters: [u8; 5],
}

impl Word {
    fn new(string: &str) -> Option<Word> {
        if string.chars().count() != 5 {
            return None;
        }
        let mut chars = string.chars();
        Some(Word {
            letters: [
                Word::letter(chars.next().unwrap())?,
                Word::letter(chars.next().unwrap())?,
                Word::letter(chars.next().unwrap())?,
                Word::letter(chars.next().unwrap())?,
                Word::letter(chars.next().unwrap())?,
            ],
        })
    }

    fn from_bytes(b: &[u8]) -> Word {
        Word {
            letters: [b[0], b[1], b[2], b[3], b[4]],
        }
    }

    fn letter(c: char) -> Option<u8> {
        if !c.is_ascii_alphabetic() {
            return None;
        }
        Some(c.to_ascii_uppercase() as u8)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_iter(self.letters.map(|letter| letter as char))
        )
    }
}

fn millis(instant: Instant) -> f64 {
    (instant.elapsed().as_nanos() as f64) / 1000000f64
}