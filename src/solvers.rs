use crate::wordle::*;

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

    fn cull(&mut self, pattern: Pattern) {
        self.targets.retain(|word| word.fits_pattern(self.guess, pattern));
    }

    fn update_guess(&mut self) {
        if self.targets.len() == 1 {
            self.guess = self.targets[0];
            return;
        }
        let mut lowest_pool = (self.pool[0], usize::MAX);
        let mut lowest_target = (self.pool[0], usize::MAX);
        let mut target_count = 0;
        let mut current_target = self.targets.get(0);
        for &guess in &*self.pool {
            let mut patterns = vec![0; 243];
            for &target in &self.targets {
                patterns[Pattern::calculate(guess, target).index()] += 1;
            }
            let mut score = 0;
            for &count in &patterns {
                if count > 0 {
                    score += count * count;
                }
            }
            
            if score < lowest_pool.1 {
                lowest_pool = (guess, score);
            }
            if Some(&guess) == current_target {
                target_count += 1;
                current_target = self.targets.get(target_count);
                if score < lowest_target.1 {
                    lowest_target = (guess, score);
                }
            }
        }
        self.guess = if lowest_target.1 == lowest_pool.1 { lowest_target.0 } else { lowest_pool.0 };
    }

    fn guess(&self) -> Word {
        self.guess
    }

    fn options(&self) -> usize {
        self.targets.len()
    }
}