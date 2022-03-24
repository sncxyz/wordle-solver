use std::time::Instant;
use crate::wordle;
use std::path::PathBuf;

pub fn build(pool: PathBuf, targets: PathBuf, solver: u8) -> Result<(), wordle::Error> {
    let start = Instant::now();
    wordle::Environment::build(pool, targets, solver)?;
    println!("Build complete in {}ms", start.elapsed().as_millis());
    Ok(())
}