use crate::RunError;
use std::{thread, sync::{Arc, Mutex}};

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &str) -> Result<Vec<Vec<char>>, RunError> {
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
    // Count trees along various slopes, return product

    let height = values.len();
    let width = values[0].len();

    let grid = Arc::new(values);
    let trees_hit_product = Arc::new(Mutex::new(1));
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut handles: Vec<thread::JoinHandle<_>> = vec![];
    for (dx, dy) in slopes {
        let trees_hit_product_shared = Arc::clone(&trees_hit_product);
        let grid_shared = Arc::clone(&grid);

        handles.push(thread::spawn(move || {
            let (mut x, mut y) = (0, 0);
            let mut trees_hit = 0;

            while y < height {
                if grid_shared[y][x] == '#' {
                    trees_hit += 1
                }

                x = (x + dx) % width;
                y += dy;
            }

            *trees_hit_product_shared.lock().unwrap() *= trees_hit;
        }));
    }

    let ret_val = Ok(*trees_hit_product.lock().unwrap());
    ret_val
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

    static SAMPLE_GOALS: [usize; 2] = [7, 336];

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_data(SAMPLE_INPUT).unwrap(),
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
