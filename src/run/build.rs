use crate::wordle;
use std::time::Instant;

pub fn build(words: &str, targets: &str, solver: u8) -> Result<(), wordle::Error> {
    let start = Instant::now();
    wordle::Environment::build(words, targets, solver)?;
    println!("Build complete in {}ms", start.elapsed().as_millis());
    Ok(())
}
