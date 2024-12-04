use anyhow::Result;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MUL_PATTERN: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref TOGGLE_PATTERN: Regex = Regex::new(r"(do|don't)\(\)").unwrap();
}

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

fn process_line(line: &str, enabled: &mut bool, total: &mut i64) {
    let mut operations: Vec<(usize, &str)> = Vec::new();
    
    for m in MUL_PATTERN.find_iter(line) {
        operations.push((m.start(), m.as_str()));
    }
    
    for m in TOGGLE_PATTERN.find_iter(line) {
        operations.push((m.start(), m.as_str()));
    }
    
    operations.sort_by_key(|&(pos, _)| pos);
    
    for (_, op) in operations {
        match op {
            "do()" => *enabled = true,
            "don't()" => *enabled = false,
            mul_str => {
                if *enabled {
                    if let Some(cap) = MUL_PATTERN.captures(mul_str) {
                        if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                            if let (Ok(x), Ok(y)) = (x.as_str().parse::<i64>(), y.as_str().parse::<i64>()) {
                                *total += x * y;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let mut enabled = true;  // multiplications start enabled
    
    for line in input.lines() {
        process_line(line, &mut enabled, &mut total);
    }
    
    Ok(total)
}