use crate::RunError;

pub fn main(part: u8, data: String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: String) -> Result<Vec<Vec<char>>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    let mut grid: Vec<Vec<char>> = vec![];

    lines.iter()
        .for_each(|line| grid.push(line.chars()
        .collect::<Vec<char>>()));

    Ok(grid)
}

fn part1(values: Vec<Vec<char>>) -> Result<usize, RunError> {
    // Count trees along slope -1/3

    let (mut x, mut y) = (0, 0);
    let height = values.len();
    let width = values[0].len();
    let mut trees_hit = 0;

    while y < height {
        if values[y][x] == '#' {
            trees_hit += 1
        }

        x = (x + 3) % width;
        y += 1;
    }

    Ok(trees_hit)
}

fn part2(values: Vec<Vec<char>>) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str ="..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    static SAMPLE_DATA: &'static [[char; 11]; 11] = &[
        ['.','.','#','#','.','.','.','.','.','.','.'],
        ['#','.','.','.','#','.','.','.','#','.','.'],
        ['.','#','.','.','.','.','#','.','.','#','.'],
        ['.','.','#','.','#','.','.','.','#','.','#'],
        ['.','#','.','.','.','#','#','.','.','#','.'],
        ['.','.','#','.','#','#','.','.','.','.','.'],
        ['.','#','.','#','.','#','.','.','.','.','#'],
        ['.','#','.','.','.','.','.','.','.','.','#'],
        ['#','.','#','#','.','.','.','#','.','.','.'],
        ['#','.','.','.','#','#','.','.','.','.','#'],
        ['.','#','.','.','#','.','.','.','#','.','#'],];
    static SAMPLE_GOALS: [usize; 2] = [7, 0];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(SAMPLE_INPUT.to_string()).unwrap(),
            SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(SAMPLE_DATA.map(|line| line.to_vec()).to_vec()).unwrap(),
            SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(SAMPLE_DATA.map(|line| line.to_vec()).to_vec()).unwrap(),
            SAMPLE_GOALS[1]);
    }
}
