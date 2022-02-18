use std::io;
use std::time::Instant;
use wordle_solver::wordle::*;

fn main() {
    let e = Environment::new().unwrap();
    let mut total_guesses = 0;
    let (mut min, mut max) = (usize::MAX, 0);
    let mut total_time = 0;
    for &target in e.targets() {
        let mut wordle = Wordle::new(&e);
        let mut guess = wordle.starting_guess();
        let now = Instant::now();
        // guess = wordle.next_guess();
        let mut guesses = 1;
        while guess != target {
            guesses += 1;
            wordle.cull(guess, wordle.get_pattern(guess, target).unwrap());
            guess = wordle.next_guess();
        }
        let word_time = now.elapsed().as_micros();
        total_time += word_time;
        println!("{}: {} guesses, {}ms", e.get_word(target).unwrap(), guesses, (word_time as f64) / 1000f64);
        total_guesses += guesses;
        min = min.min(guesses);
        max = max.max(guesses);
    }
    let total_time_millis = (total_time as f64) / 1000f64;
    println!();
    println!("{} wordles solved in {:.3?}s", e.targets().len(), total_time_millis / 1000f64);
    println!("average time per word: {:.3?}ms", total_time_millis / (e.targets().len() as f64));
    println!("average time per guess: {:.3?}ms", total_time_millis / (total_guesses as f64));
    println!("between {} and {} guesses, average {:.5?}", min, max, (total_guesses as f64) / (e.targets().len() as f64));

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
