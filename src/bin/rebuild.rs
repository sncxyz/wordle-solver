fn main() {
    wordle_solver::wordle::Environment::rebuild("input/pool.txt", "input/targets.txt", 0).unwrap();
}
