use std::fmt::Display;

pub fn calculate_pattern(guess: Word, target: Word) -> u8 {
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
    value
}

pub fn get_pattern(input: &str) -> Option<u8> {
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
    Some(value)
}

#[derive(Clone)]
pub struct WordInfo {
    word: Word,
    target: u16,
}

impl WordInfo {
    pub fn new(word: Word, target: Option<usize>) -> WordInfo {
        WordInfo {
            word,
            target: match target {
                None => u16::MAX,
                Some(index) => index as u16,
            },
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<WordInfo> {
        Some(WordInfo {
            word: Word::new(std::str::from_utf8(&bytes[0..5]).ok()?)?,
            target: u16::from_be_bytes([bytes[5], bytes[6]]),
        })
    }

    pub fn to_bytes(&self) -> [u8; 7] {
        let l = self.word.letters;
        let t = self.target.to_be_bytes();
        [l[0], l[1], l[2], l[3], l[4], t[0], t[1]]
    }

    pub fn get_word(&self) -> Word {
        self.word
    }

    pub fn get_target(&self) -> Option<usize> {
        match self.target {
            u16::MAX => None,
            x => Some(x as usize),
        }
    }

    pub fn is_target(&self) -> bool {
        self.get_target().is_some()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Word {
    letters: [u8; 5],
}

impl Word {
    pub fn new(string: &str) -> Option<Word> {
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
