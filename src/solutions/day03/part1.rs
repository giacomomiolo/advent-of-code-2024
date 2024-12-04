use anyhow::Result;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // "mul(" followed by 1-3 digits, a comma, 1-3 digits, and ")"
    static ref MUL_PATTERN: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let mut total: i64 = 0;
    
    for line in input.lines() {
        for cap in MUL_PATTERN.captures_iter(line) {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if let (Ok(x), Ok(y)) = (x.as_str().parse::<i64>(), y.as_str().parse::<i64>()) {
                    total += x * y;
                }
            }
        }
    }
    
    Ok(total)
}