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

    let rows = grid.len();
    if rows == 0 {
        return Ok(0);
    }
    let cols = grid[0].len();

    let directions = [
        (0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)
    ];
    let word = ['X','M','A','S'];
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            let ch = grid[r][c];
            if ch != 'X' {
                continue;
            }
            for &(dr,dc) in &directions {
                let mut found = true;
                for i in 1..4 {
                    let nr = r as isize + (i as isize * dr);
                    let nc = c as isize + (i as isize * dc);
                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != word[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}
