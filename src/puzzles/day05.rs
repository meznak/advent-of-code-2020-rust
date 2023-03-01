use crate::RunError;

struct Ticket {
    row: usize,
    seat: usize,
}

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<Ticket>, RunError> {
    Ok(data.split('\n')
        .map(|line|
            line.replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1")
        ).map(|line| Ticket {
            row: line[..7], // convert from binary
            line[7..]}
        )
        .collect::<Vec<Ticket>>())
}

fn part1(values: Vec<Ticket>) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

fn part2(values: Vec<Ticket>) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="FBFBBFFRLR";
    static SAMPLE_DATA: &'static [Ticket] = &[Ticket{44, 5}];
    static SAMPLE_GOALS: [usize; 2] = [357, 0];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(SAMPLE_INPUT.to_string()).unwrap(),
            SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(SAMPLE_DATA.to_vec()).unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(SAMPLE_DATA.to_vec()).unwrap(),
            SAMPLE_GOALS[1]);
    }
}
