#![allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse_with(input, |byte| byte);
    let mut result = 0;
    for i in 0..grid.width() {
        for j in 0..grid.height() {
            if grid.get(i, j)? == &b'.' {
                continue;
            }
            let num_free = grid.neighbors_with_diagonals(i, j).filter(|&location| {
                match grid.get(location.0, location.1) {
                    Some(&value) => value == b'@',
                    None => false,
                }
            }).count();
            if num_free < 4 {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::parse_with(input, |byte| byte);
    let dimensions = grid.height();
    let mut result = 0;
    let mut last_iter_result = 1;
    while result != last_iter_result {
        last_iter_result = result;

        for i in 0..dimensions {
            for j in 0..dimensions {
                if grid.get(i, j)? == &b'.' {
                    continue;
                }
                let num_not_free = grid.neighbors_with_diagonals(i, j).filter(|&location| {
                    match grid.get(location.0, location.1) {
                        Some(&value) => value == b'@',
                        None => false,
                    }
                }).count();
                if num_not_free < 4 {
                    result += 1;
                    grid.set((i, j), b'.');
                }
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
