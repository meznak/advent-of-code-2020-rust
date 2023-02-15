use crate::RunError;

#[derive(Debug, Clone, PartialEq)]
struct Password <'a> {
    min: i32,
    max: i32,
    character: char,
    password: &'a str
}

pub fn main(part: u8, data: String) -> Result<i32, RunError> {
    let parsed_data = parse_data(&data)?;

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
        // How do I safely extract a character?
        //let character: char = parts[1].chars().next().ok_or(RunError::ParseString)?;
        let character: char = match parts[1].chars().next() {
            Some(c) => c,
            None => {return Err(RunError::ParseString(parts[1].to_string()));}
        };
        //let character: char = parts[1].split(':').collect();

        passwords.push(Password {
            min: min_max[0].parse::<i32>()?,
            max: min_max[1].parse::<i32>()?,
            character,
            password: parts[2]
        });
    }

    Ok(passwords)

}

fn part1(values: &[Password]) -> Result<i32, RunError> {
    // What's the goal?

    todo!();

    println!("The puzzle failed!");
    Err(RunError::PartFailed)
}

fn part2(values: &[Password]) -> Result<i32, RunError> {
    // What's the goal?

    todo!();

    println!("The puzzle failed!");
    Err(RunError::PartFailed)
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

    const SAMPLE_GOALS: [i32; 2] = [542, 0];

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
