use super::Wordle;

pub fn solver(id: u8) -> Option<impl Fn(&Wordle) -> Option<u16>> {
    [one, two].get(id as usize)
}

fn one(wordle: &Wordle) -> Option<u16> {
    if let Some(id) = wordle.only_remaining() {
        return Some(id);
    }
    let mut lowest = (0, usize::MAX);
    let mut t = 0;
    for id in wordle.words() {
        let mut patterns = vec![0; 243];
        for &target in wordle.targets() {
            patterns[wordle.get_pattern(id, target)? as usize] += 1;
        }
        let mut score = 0;
        for i in 0..243 {
            score += patterns[i] * patterns[i];
        }
        if wordle.targets().get(t) == Some(&id) {
            score -= 1;
            t += 1;
        }
        if score < lowest.1 {
            lowest = (id, score);
        }
    }
    Some(lowest.0)
}

fn two(wordle: &Wordle) -> Option<u16> {
    if let Some(id) = wordle.only_remaining() {
        return Some(id);
    }
    let total = wordle.targets().len() as f64;
    let mut highest = (0, 0.0);
    let mut t = 0;
    for id in wordle.words() {
        let mut patterns = vec![0; 243];
        for &target in wordle.targets() {
            patterns[wordle.get_pattern(id, target)? as usize] += 1;
        }
        let mut entropy = 0.0;
        for i in 0..243 {
            if patterns[i] > 0 {
                let p = patterns[i] as f64 / total;
                entropy -= p * p.log2();
            }
        }
        if wordle.targets().get(t) == Some(&id) {
            entropy += 1.0 / total;
            t += 1;
        }
        if entropy > highest.1 {
            highest = (id, entropy);
        }
    }
    Some(highest.0)
}