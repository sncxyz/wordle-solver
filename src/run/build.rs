use std::time::Instant;
use crate::wordle;

pub fn build(pool: &str, targets: &str, solver: u8) -> Result<(), wordle::Error> {
    let start = Instant::now();
    wordle::Environment::build(pool, targets, solver)?;
    println!("Build complete in {}ms", start.elapsed().as_millis());
    Ok(())
}