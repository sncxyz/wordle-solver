use wordle_solver::{solvers, wordle::{self, Solver}};
use std::time::Instant;
use std::io;
use std::rc::Rc;

fn main() {
    test_solver::<solvers::Version1>();
}

fn test_solver<S>()
where
    S: Solver,
{
    let targets = Rc::new(wordle::get_word_list("targets.txt").unwrap());
    let pool = Rc::new(wordle::get_word_list("pool.txt").unwrap());
    let mut total_guesses = 0;
    let (mut min, mut max) = (usize::MAX, 0);
    let mut total_time = 0;
    for &target in &*targets {
        let mut solver = S::new(Rc::clone(&targets), Rc::clone(&pool));
        let now = Instant::now();
        // solver.update_guess();
        let mut guesses = 1;
        while solver.guess() != target {
            guesses += 1;
            solver.narrow_from_pattern(wordle::Pattern::calculate(solver.guess(), target));
            solver.update_guess();
        }
        let word_time = now.elapsed().as_micros();
        total_time += word_time;
        println!("{}: {} guesses, {}ms", target, guesses, (word_time as f64) / (1000f64));
        total_guesses += guesses;
        min = min.min(guesses);
        max = max.max(guesses);
    }
    let total_time_millis = (total_time as f64) / (1000f64);
    println!();
    println!("{} wordles solved in {:.3?}s", targets.len(), total_time_millis / 1000f64);
    println!("average time per word: {:.3?}ms", total_time_millis / (targets.len() as f64));
    println!("average time per guess: {:.3?}ms", total_time_millis / (total_guesses as f64));
    println!("between {} and {} guesses, average {:.5?}", min, max, (total_guesses as f64) / (targets.len() as f64));

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}