use crate::wordle::*;
use std::collections::HashMap;

pub struct Version1 {
    targets: Vec<Word>,
    pool: Vec<Word>,
    guess: Word,
}

impl Solver for Version1 {
    fn new(targets: Vec<Word>, pool: Vec<Word>) -> Self {
        Version1 {
            targets,
            pool,
            guess: Word::new(String::from("roate")).unwrap(),
        }
    }

    fn narrow_from_string(&mut self, input: &str) -> bool {
        if let Some(pattern) = Pattern::new(input) {
            self.targets.retain(|word| word.fits_pattern(self.guess, pattern));
            return true;
        }
        false
    }

    fn narrow_from_pattern(&mut self, pattern: Pattern) {
        self.targets.retain(|word| word.fits_pattern(self.guess, pattern));
    }

    fn update_guess(&mut self) {
        let mut lowest = (self.pool[0], usize::MAX);
        for &guess in &self.pool {
            let mut patterns = HashMap::new();
            for &word in &self.targets {
                *patterns.entry(Pattern::calculate(guess, word)).or_insert(0) += 1;
            }
            let score = patterns.into_values().map(|count| count * count).sum();
            if score < lowest.1
                || (score == lowest.1
                    && self.targets.contains(&guess)
                    && !self.targets.contains(&lowest.0))
            {
                lowest = (guess, score);
            }
        }
        self.guess = lowest.0;
    }

    fn guess(&self) -> Word {
        self.guess
    }

    fn options(&self) -> usize {
        self.targets.len()
    }
}
