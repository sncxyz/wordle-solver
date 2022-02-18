fn main() {
    wordle_solver::wordle::Environment::rebuild("input/pool.txt", "input/targets.txt", 0).unwrap();
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
