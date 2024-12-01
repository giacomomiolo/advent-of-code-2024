pub mod part1;
pub mod part2;

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn read_input() -> Result<String> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/solutions/day01/input.txt");
    Ok(fs::read_to_string(d)?)
}
