use super::Wordle;

pub fn solver(id: u8) -> Option<impl Fn(&Wordle) -> Option<u16>> {
    [one].get(id as usize)
}

fn one(wordle: &Wordle) -> Option<u16> {
    if let Some(id) = wordle.only_remaining() {
        return Some(id);
    }
    let mut lowest = (0, usize::MAX);
    for id in wordle.words() {
        let mut patterns = vec![0; 243];
        for &target in wordle.targets() {
            patterns[wordle.get_pattern(id, target)? as usize] += 1;
        }
        let score = patterns.into_iter().map(|count| count * count).sum();
        if score < lowest.1
            || (score == lowest.1
                && wordle.is_target(id)?
                && wordle.targets().contains(&id))
        {
            lowest = (id, score);
        }
    }
    Some(lowest.0)
}
