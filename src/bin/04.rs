#![allow(unused_imports)]

use std::collections::VecDeque;
use advent_of_code::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mask: u8 = 64;
    let grid = process_grid(input);
    let len = grid.len();
    let mut result = 0;

    unsafe {
        for i in 1..len - 1 {
            for j in 1..len - 1 {
                if grid.get_unchecked(i).get_unchecked(j) & mask == 0 {
                    continue;
                }

                let roll_amount = count_neighbors_with_mask(&grid, i, j, mask);

                if roll_amount < 4 {
                    result += 1;
                }
            }
        }
    }

    Some(result as u64)
}

#[inline(always)]
fn process_grid(input: &str) -> Vec<Vec<u8>> {
    let lines: Vec<&str> = input.lines().collect();

    let length = lines.len();

    let final_length = length + 2;

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(final_length);

    let boundary_row = vec![b'.'; final_length];

    grid.push(boundary_row.clone());

    for line in lines {
        let mut row = Vec::with_capacity(final_length);

        row.push(b'.');

        row.extend_from_slice(line.as_bytes());

        row.push(b'.');

        grid.push(row);
    }

    grid.push(boundary_row);

    grid
}

#[inline(always)]
fn count_neighbors_with_mask(grid: &[Vec<u8>], r: usize, c: usize, mask: u8) -> usize {
    Dir::ALL_WITH_DIAGONALS.iter()
        .map(|&dir| move_in_direction(dir, &(r as isize, c as isize)))
        .filter(|&(k0, k1)| {
            unsafe {
                *grid.get_unchecked(k0 as usize).get_unchecked(k1 as usize) & mask != 0
            }
        })
        .count()
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::parse_with(input, |byte| byte >> 6);
    let mut todo: Vec<(isize, isize)> = Vec::new();
    let mut padded = Grid::new(grid.width + 2, grid.height + 2, vec![15; (grid.height + 2) * (grid.width + 2)]);

    let length = grid.height() as isize;
    for x in 0..length {
        for y in 0..length {

            if *grid.get_isize(x, y) == 1 {
                let count = grid.neighbors_with_diagonals_locations(x as usize, y as usize).filter(|&location| grid[location] == 1).count();

                if count < 4 {
                    todo.push((x + 1, y + 1));
                }
                padded.set(((x + 1) as usize, (y + 1) as usize), count as u8);
            }
        }
    }

    let mut result = 0;

    while let Some((x, y)) = todo.pop() {
        result += 1;

        for next in Dir::ALL_WITH_DIAGONALS.map(|d| (x + d.delta().0, y + d.delta().1)) {
            let current_amount = *padded.get_isize(next.0, next.1);
            if current_amount == 4 {
                todo.push(next);
            }
            padded.set((next.0 as usize, next.1 as usize), current_amount - 1);
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
