use anyhow::Result;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let grid: Vec<Vec<char>> = input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    let rows = grid.len();
    if rows == 0 {
        return Ok(0);
    }
    let cols = grid[0].len();

    for r in 0..rows.saturating_sub(2) {
        for c in 0..cols.saturating_sub(2) {
            let a = grid[r+1][c+1] == 'A';

            if a {
                let top_left = grid[r][c];
                let top_right = grid[r][c+2];
                let bottom_left = grid[r+2][c];
                let bottom_right = grid[r+2][c+2];

                if (top_left == 'M' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'S') ||
                   (top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S') ||
                   (top_left == 'S' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'M') ||
                   (top_left == 'S' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'M') {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}
