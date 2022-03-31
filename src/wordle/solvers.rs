use super::Wordle;

use rayon::prelude::*;

pub fn solver(id: u8) -> Option<impl Fn(&Wordle) -> u16> {
    [zero, one].get(id as usize)
}

fn zero(wordle: &Wordle) -> u16 {
    if let Some(id) = wordle.only_remaining() {
        return id;
    }
    wordle
        .words()
        .par_iter()
        .map(|&(id, is_target)| {
            let mut patterns = vec![0usize; 243];
            for &target in wordle.targets() {
                patterns[wordle.get_pattern(id, target).unwrap() as usize] += 1;
            }
            let mut score = 0;
            for &count in &patterns {
                score += count * count;
            }
            if is_target && score > 0 {
                score -= 1;
            }
            (id, score)
        })
        .min_by(|&(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0
}

fn one(wordle: &Wordle) -> u16 {
    if let Some(id) = wordle.only_remaining() {
        return id;
    }
    let total = wordle.targets().len() as f64;
    wordle
        .words()
        .par_iter()
        .map(|&(id, is_target)| {
            let mut patterns = vec![0usize; 243];
            for &target in wordle.targets() {
                patterns[wordle.get_pattern(id, target).unwrap() as usize] += 1;
            }
            let mut entropy = if is_target { 1.0 / total } else { 0.0 };
            for &count in &patterns {
                if count > 0 {
                    let p = count as f64 / total;
                    entropy -= p * p.log2();
                }
            }
            (id, entropy)
        })
        .max_by(|&(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0
}
