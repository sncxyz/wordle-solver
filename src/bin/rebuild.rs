use std::time::Instant;
use wordle_solver::wordle::Environment;
use std::io;

fn main() {
    let start = Instant::now();
    if let Err(message) = Environment::rebuild("input/pool.txt", "input/targets.txt", 0) {
        println!("{}", message);
    } else {
        println!("Completed in {}ms", start.elapsed().as_millis());
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}