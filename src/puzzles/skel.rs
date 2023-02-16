use crate::RunError;

pub fn main(part: u8, data: String) -> Result<i32, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: String) -> Result<Vec<i32>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    todo!();
}

fn part1(values: Vec<i32>) -> Result<i32, RunError> {
    // What's the goal?

    todo!();
}

fn part2(values: Vec<i32>) -> Result<i32, RunError> {
    // What's the goal?

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="";
    static SAMPLE_DATA: &'static [i32] = &[];
    static SAMPLE_GOALS: [usize; 2] = [0, 0];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(SAMPLE_INPUT.to_string()).unwrap(), SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_DATA.to_vec()).unwrap(), SAMPLE_GOALS.0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_DATA.to_vec()).unwrap(), SAMPLE_GOALS.1);
    }
}
