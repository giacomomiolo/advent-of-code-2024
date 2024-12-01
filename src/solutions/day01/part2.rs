use anyhow::Result;
use std::collections::HashMap;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let estimated_size = input.lines().count();
    
    let (left_numbers, right_numbers): (Vec<_>, Vec<_>) = {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for line in input.lines() {
            let mut nums = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok());
                
            if let (Some(l), Some(r)) = (nums.next(), nums.next()) {
                left.push(l);
                right.push(r);
            }
        }
        (left, right)
    };

    let mut right_freq = HashMap::with_capacity(estimated_size);
    
    for &num in &right_numbers {
        *right_freq.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i64 = left_numbers
        .iter()
        .map(|&num| num * right_freq.get(&num).copied().unwrap_or(0) as i64)
        .sum();

    Ok(similarity_score)
}