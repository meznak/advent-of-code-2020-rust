use crate::RunError;

#[derive(Debug, PartialEq)]
struct Ticket {
    row: usize,
    col: usize,
}

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<Ticket>, RunError> {
    let mut tickets: Vec<Ticket> = vec![];

    data.split('\n')
        .map(|line: &str|
            line.replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1")
        )
        .for_each(|line: String| {
            tickets.push(Ticket {
                    row: usize::from_str_radix(&line[..7], 2).unwrap(),
                    col: usize::from_str_radix(&line[7..], 2).unwrap(),
            });
        });

    Ok(tickets)
}

fn part1(values: &[Ticket]) -> Result<usize, RunError> {
    // Seat ID: row * 8 + seat
    // Return highest seat ID seen

    calculate_seats(values)?
    .into_iter()
    .max()
    .ok_or(RunError::PartFailed)

}

fn part2(values: &[Ticket]) -> Result<usize, RunError> {
    // Find the seat not on the list

    let mut seats = calculate_seats(values)?;
    seats.sort();

    for position in 0..seats.len() {
        if seats[position + 1] != seats[position] + 1 {
            return Ok(seats[position] + 1);
        }
    }

    Err(RunError::PartFailed)
}

fn calculate_seats(values: &[Ticket]) -> Result<Vec<usize>, RunError> {
    // Calculate all seat numbers

    Ok(values.iter().map(|ticket|
        ticket.row * 8 + ticket.col
    )
    .collect::<Vec<usize>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "FBFBBFFRLR";
    static SAMPLE_PART_1: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
    static SAMPLE_DATA: &'static [Ticket] = &[Ticket{ row: 44, col: 5}];
    static SAMPLE_GOALS: [usize; 2] = [820, 0];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(SAMPLE_INPUT).unwrap(),
            SAMPLE_DATA);

        assert_eq!(
            parse_data("BFFFBBFRRR").unwrap(),
            &[Ticket{ row: 70, col: 7}]);

        assert_eq!(
            parse_data("FFFBBBFRRR").unwrap(),
            &[Ticket{ row: 14, col: 7}]);

        assert_eq!(
            parse_data("BBFFBBFRLL").unwrap(),
            &[Ticket{ row: 102, col: 4}]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_data(SAMPLE_PART_1).unwrap()).unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(SAMPLE_DATA).unwrap(),
            SAMPLE_GOALS[1]);
    }
}
