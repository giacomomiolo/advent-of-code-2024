use anyhow::Result;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

fn is_safe_sequence(numbers: &[i64]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let first_diff = numbers[1] - numbers[0];
    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let increasing = first_diff > 0;
    
    for window in numbers.windows(2) {
        let diff = window[1] - window[0];
        
        if (increasing && diff <= 0) || (!increasing && diff >= 0) {
            return false;
        }
        
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
    }

    true
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let result = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            
            is_safe_sequence(&numbers)
        })
        .count();

    Ok(result as i64)
}