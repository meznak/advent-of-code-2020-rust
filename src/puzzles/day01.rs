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

    match lines.iter()
        .map(|x| x.trim().parse::<i32>())
        .collect() {
            Ok(parsed_data) => Ok(parsed_data),
            Err(e) => Err(RunError::ParseInt(e))
        }
}

fn part1(values: Vec<i32>) -> Result<i32, RunError> {
    // Find two entries that sum to 2020 and return their product.
    for i in values.iter() {
        for j in values.iter() {
            if i != j && *i + *j == 2020 {
                return Ok(i * j);
            }
        }
    }

    println!("No pairs found!");
    Err(RunError::PartFailed)
}

fn part2(values: Vec<i32>) -> Result<i32, RunError> {
    // Find three entries that sum to 2020 and return their product.
    for i in values.iter() {
        for j in values.iter() {
            if i != j {
                for k in values.iter() {
                    if k != j && *i + *j + *k == 2020 {
                        return Ok(i * j * k);
                    }
                }
            }
        }
    }

    println!("No triples found!");
    Err(RunError::PartFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="1721
    979
    366
    299
    675
    1456";

    static SAMPLE_DATA: &'static [i32] = &[1721,
    979,
    366,
    299,
    675,
    1456
    ];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(SAMPLE_INPUT.to_string()).unwrap(), SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_DATA.to_vec()).unwrap(), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_DATA.to_vec()).unwrap(), 241861950);
    }
}
