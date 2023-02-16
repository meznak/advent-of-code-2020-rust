use crate::RunError;

#[derive(Debug, Clone, PartialEq)]
struct Password <'a> {
    min: usize,
    max: usize,
    character: char,
    password: &'a str
}

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data<'a>(data: &'a str) -> Result<Vec<Password <'a>>, RunError> {
    let lines: Vec<&str> = data.split('\n').collect();

    // sample line:
    // 1-3 a: abcde

    let mut passwords: Vec<Password <'a>> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let min_max: Vec<&str> = parts[0].split('-').collect();
        let character: char = match parts[1].chars().next() {
            Some(c) => c,
            None => {return Err(RunError::ParseString(parts[1].to_string()));}
        };

        passwords.push(Password {
            min: min_max[0].parse::<usize>()?,
            max: min_max[1].parse::<usize>()?,
            character,
            password: parts[2]
        });
    }

    Ok(passwords)

}

fn part1(values: &[Password]) -> Result<usize, RunError> {
    // Count valid passwords, given allowed counts of a specified character

    let mut valid_count: usize = 0;

    for value in values {
        let character_count = value.password.chars()
            .filter(|character| *character == value.character)
            .count();
        if character_count >= value.min && character_count <= value.max {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn part2(values: &[Password]) -> Result<usize, RunError> {
    // Check given positions for a given character

    let mut valid_count: usize = 0;

    for value in values {
        if value.password.len() < value.max {
            return Err(RunError::InputBounds);
        }

        let first = value.password.get(value.min - 1..value.min)
            .ok_or(RunError::PartFailed)?.chars().next().ok_or(RunError::PartFailed)?;
        let last = value.password.get(value.max - 1..value.max)
            .ok_or(RunError::PartFailed)?.chars().next().ok_or(RunError::PartFailed)?;
        if  first != last && (first == value.character || last == value.character ) {
            valid_count += 1;
        }
    }

    Ok(valid_count)

}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    static SAMPLE_DATA: [Password; 3] = [
        Password {min: 1, max: 3, character: 'a', password: "abcde"},
        Password {min: 1, max: 3, character: 'b', password: "cdefg"},
        Password {min: 2, max: 9, character: 'c', password: "ccccccccc"},
        ];

    const SAMPLE_GOALS: [usize; 2] = [2, 1];

    #[test]
    fn test_parse() {
        assert!(parse_data(&SAMPLE_INPUT.to_string()).unwrap()
            .iter().all(|item| SAMPLE_DATA.contains(item)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE_DATA.to_vec()).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE_DATA.to_vec()).unwrap(), SAMPLE_GOALS[1]);
    }
}
