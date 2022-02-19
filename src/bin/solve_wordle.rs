use std::io;
use wordle_solver::wordle::*;

fn main() {
    if solve().is_none() {
        println!("Data file missing or corrupted. Please rebuild.");
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn solve() -> Option<()> {
    let e = Environment::new()?;
    let mut wordle = Wordle::new(&e);
    let mut guess = wordle.starting_guess();
    println!("Output from Wordle formatted with [B]lack, [Y]ellow, [G]reen");
    while wordle.options() > 1 {
        println!("Guess \"{}\" ({} options)", wordle.get_word(guess)?, wordle.options());
        loop {
            println!("What output did Wordle give you?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap_or_else(|_| 0);
            if let Some(pattern) = get_pattern(input.trim()) {
                wordle.cull(guess, pattern);
                break;
            }
            println!("Invalid input.");
        }
        match wordle.options() {
            0 => println!("No options remain."),
            1 => println!("The word is \"{}\"", wordle.get_word(wordle.only_remaining().unwrap())?),
            _ => guess = wordle.next_guess()?,
        }
    }
    Some(())
}