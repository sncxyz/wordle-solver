use crate::wordle;
use std::time::Instant;

pub fn build(full: &str, targets: &str, solver: u8) -> Result<(), wordle::Error> {
    let start = Instant::now();
    wordle::Environment::build(full, targets, solver)?;
    println!("Build complete in {}ms", start.elapsed().as_millis());
    Ok(())
}
