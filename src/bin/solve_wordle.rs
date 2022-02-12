use wordle_solver::{solvers, wordle::{self, Word, Solver}};
use std::io;

fn main() {
    solve_wordle::<solvers::Version1>(
        wordle::get_word_list("targets.txt").unwrap(),
        wordle::get_word_list("pool.txt").unwrap(),
    );
}

fn solve_wordle<S>(targets: Vec<Word>, pool: Vec<Word>)
where
    S: Solver,
{
    let mut solver = S::new(targets, pool);
    println!("Output from Wordle formatted with [B]lack, [Y]ellow, [G]reen");
    while solver.options() > 1 {
        println!("Guess \"{}\" ({} options)", solver.guess(), solver.options());
        let mut input = String::new();
        while !solver.narrow_from_string(input.trim()) {
            println!("What output did Wordle give you?");
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
        solver.update_guess();
        match solver.options() {
            0 => println!("No options remain."),
            1 => println!("The word is \"{}\"", solver.guess()),
            _ => (),
        }
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
