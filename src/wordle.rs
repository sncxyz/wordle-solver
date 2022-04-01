use crate::solvers;
use crate::words;

pub use words::get_pattern;
use words::*;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub struct Environment {
    words: Vec<WordInfo>,
    targets: Vec<u16>,
    patterns: Vec<u8>,
    solver: u8,
    starting_guess: u16,
}

impl Environment {
    pub fn build(pool: &str, targets: &str, solver: u8) -> Result<(), Error> {
        let mut pool = Environment::get_word_list(pool).ok_or_else(|| Error::PoolRead)?;
        let target_words = Environment::get_word_list(targets).ok_or_else(|| Error::TargetsRead)?;
        let target_words =
            Environment::parse_word_list(target_words).ok_or_else(|| Error::TargetsFormat)?;

        pool.sort_unstable();
        let mut pool = Environment::parse_word_list(pool).ok_or_else(|| Error::PoolFormat)?;
        pool.dedup();
        let pool = pool;

        if pool.len() > u16::MAX as usize {
            return Err(Error::PoolLength);
        }

        let mut words = Vec::with_capacity(pool.len());
        let mut targets = Vec::with_capacity(target_words.len().min(pool.len()));

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

        let mut patterns = Vec::with_capacity(words.len() * targets.len());
        for word in &words {
            for &target in &targets {
                patterns.push(calculate_pattern(
                    word.get_word(),
                    words[target as usize].get_word(),
                ));
            }
        }

        let e = Environment {
            words: words.clone(),
            targets: targets.clone(),
            patterns: patterns.clone(),
            solver,
            starting_guess: 0,
        };
        let wordle = Wordle::new(&e);
        let starting_guess = wordle.next_guess().ok_or_else(|| Error::SolverID)?;

        let mut data: Vec<u8> =
            Vec::with_capacity(7 + words.len() * 7 + targets.len() * 2 + patterns.len());
        data.push(solver); // solver
        data.extend(starting_guess.to_be_bytes()); // starting guess
        data.extend((words.len() as u16).to_be_bytes()); // length of word list
        data.extend((targets.len() as u16).to_be_bytes()); // length of target list
        data.extend(words.into_iter().map(|word| word.to_bytes()).flatten()); // word list
        data.extend(
            targets
                .into_iter()
                .map(|target| target.to_be_bytes())
                .flatten(),
        ); // target list
        data.extend(patterns.into_iter()); // pattern list

        fs::write("saved/data.bin", data).map_err(|_| Error::DataWrite)?;

        Ok(())
    }

    pub fn new() -> Option<Environment> {
        let data = fs::read("saved/data.bin").ok()?;

        if data.len() < 7 {
            return None;
        }

        let solver = data[0];
        let starting_guess = u16::from_be_bytes([data[1], data[2]]);
        let words_len = u16::from_be_bytes([data[3], data[4]]) as usize;
        let targets_len = u16::from_be_bytes([data[5], data[6]]) as usize;

        if data.len() != 7 + words_len * 7 + targets_len * 2 + words_len * targets_len
            || starting_guess as usize >= words_len
            || targets_len > words_len
        {
            return None;
        }

        let mut i = 7;
        let j = i + words_len * 7;
        let words: Vec<_> = data[i..j]
            .chunks(7)
            .map(|bytes| WordInfo::from_bytes(bytes))
            .collect::<Option<_>>()?;
        i = j + targets_len * 2;
        let targets: Vec<_> = data[j..i]
            .chunks(2)
            .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
            .collect();
        let patterns = data[i..].to_vec();

        for i in 0..targets_len {
            if targets[i] as usize >= words_len
                || words[targets[i] as usize].get_target() != Some(i)
                || (i > 0 && targets[i] <= targets[i - 1])
            {
                return None;
            }
        }

        for &pattern in &patterns {
            if pattern >= 243 {
                return None;
            }
        }

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
        Some(self.words.get(id as usize)?.get_word())
    }

    fn get_pattern(&self, guess: u16, target: u16) -> Option<u8> {
        Some(*self.patterns.get(
            guess as usize * self.targets.len() + self.words.get(target as usize)?.get_target()?,
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
    words: Vec<(u16, bool)>,
}

impl<'a> Wordle<'a> {
    pub fn new(e: &Environment) -> Wordle {
        let mut i = 0;
        let mut words = Vec::with_capacity(e.words.len());
        for id in 0..e.words.len() as u16 {
            words.push((
                id,
                if e.targets.get(i) == Some(&id) {
                    i += 1;
                    true
                } else {
                    false
                },
            ));
        }
        Wordle {
            e,
            targets: e.targets.clone(),
            words,
        }
    }

    pub fn words(&self) -> &[(u16, bool)] {
        &self.words
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

    pub fn cull(&mut self, guess: u16, pattern: u8) {
        self.targets
            .retain(|&target| self.e.get_pattern(guess, target) == Some(pattern));
        let mut i = 0;
        for (id, is_target) in self.words.iter_mut() {
            if *is_target {
                if self.targets.get(i) == Some(id) {
                    i += 1;
                } else {
                    *is_target = false;
                }
            }
        }
    }

    pub fn next_guess(&self) -> Option<u16> {
        Some(solvers::solver(self.e.solver)?(&self))
    }

    pub fn get_pattern(&self, guess: u16, target: u16) -> Option<u8> {
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
}

pub enum Error {
    PoolRead,
    TargetsRead,
    PoolFormat,
    TargetsFormat,
    PoolLength,
    SolverID,
    DataWrite,
    DataRead,
}
