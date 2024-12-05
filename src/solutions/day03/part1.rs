use anyhow::Result;

pub fn solve() -> Result<i64> {
    let input = super::read_input()?;
    solve_for_input(&input)
}

pub fn solve_for_input(input: &str) -> Result<i64> {
    let mut total: i64 = 0;
    
    for line in input.lines() {
        let mut start = 0;
        while let Some(idx) = line[start..].find("mul(") {
            let pos = start + idx;
            if let Some(end_pos) = line[pos+4..].find(')') {
                let args = &line[pos+4..pos+4+end_pos];
                if let Some(comma_idx) = args.find(',') {
                    let (x_str, y_str) = (&args[..comma_idx], &args[comma_idx+1..]);
                    if let (Ok(x), Ok(y)) = (x_str.parse::<i64>(), y_str.parse::<i64>()) {
                        total += x * y;
                    }
                }
                start = pos + 4 + end_pos + 1;
            } else {
                break;
            }
        }
    }
    
    Ok(total)
}
