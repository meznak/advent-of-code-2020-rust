use std::collections::{HashSet, HashMap};

use crate::RunError;

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<&str>, RunError> {
    Ok(data[..].split("\n\n").collect())
}

fn part1(values: &[&str]) -> Result<usize, RunError> {
    // Count unique questions answered per group
    // Return sum of those counts

    Ok(values.iter()
    .map(|group| {
        let mut questions: HashSet<char> = HashSet::new();

        group.replace('\n', "")
            .chars()
            .for_each(|c| {questions.insert(c);});

        questions
    })
    .map(|group| group.len())
    .sum::<usize>())
}

fn part2(values: &[&str]) -> Result<usize, RunError> {
    // Count questions questions answered by all in each group
    // Return sum of those counts

    Ok(values.iter()
    .map(|group| {
        let mut questions: HashMap<char, usize> = HashMap::new();

        group.chars()
            .for_each(|c| {
                if let Some(x) = questions.get_mut(&c) {
                    *x += 1;
                } else {
                    questions.insert(c, 1);
                }
            });

        let line_count = questions.get(&'\n').unwrap_or(&0) + 1;
        // println!("line count: {line_count}");
        questions.iter()
            .filter(|(k, _)| *k != &'\n')
            .filter(|(_, v)| *v == &line_count)
            .map(|(k, _)| *k)
            .collect::<Vec<char>>()
            .len()
    })
    .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="abcx
abcy
abcz";
    static SAMPLE_INPUT_PARTS: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    static SAMPLE_DATA: &'static [[&str; 3]; 1] = &[["abcx", "abcy", "abcz"]];
    static SAMPLE_GOALS: [usize; 2] = [11, 6];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(&SAMPLE_INPUT).unwrap(),
            SAMPLE_DATA[0]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_data(SAMPLE_INPUT_PARTS).unwrap())
                .unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_data(SAMPLE_INPUT_PARTS).unwrap())
                .unwrap(),
            SAMPLE_GOALS[1]);
    }
}
