#![allow(unused_imports)]

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::absolute;
use advent_of_code::*;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = input.lines().map(|line| line.split(',').map(|n| n.parse::<i64>().unwrap()).collect_tuple::<(i64, i64)>().unwrap()).collect_vec();
    let mut largest_size = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let size = ((tiles[i].0 - tiles[j].0) + 1) * ((tiles[i].1 - tiles[j].1) + 1);
            if size > largest_size {
                largest_size = size;
            }
        }
    }
    Some(largest_size as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = input.lines().map(|line| line.split(',').map(|n| n.parse::<i64>().unwrap()).collect_tuple::<(i64, i64)>().unwrap()).collect_vec();
    let x_max = tiles.iter().map(|(x, _)| x).max().unwrap();
    let y_max = tiles.iter().map(|(_, y)| y).max().unwrap();
    let mut x_coords: HashSet<i64> = HashSet::new();
    let mut y_coords: HashSet<i64> = HashSet::new();
    x_coords.insert(0);
    y_coords.insert(0);
    for &(x, y) in tiles.iter() {
        x_coords.insert(x);
        y_coords.insert(y);
    }

    x_coords.insert(x_max + 1);
    y_coords.insert(y_max + 1);

    let sorted_x: Vec<i64> = x_coords.into_iter().sorted().collect();
    let sorted_y: Vec<i64> = y_coords.into_iter().sorted().collect();

    let x_map: HashMap<i64, usize> = sorted_x.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    let y_map: HashMap<i64, usize> = sorted_y.into_iter().enumerate().map(|(i, y)| (y, i)).collect();

    let mut compressed_grid: Vec<Vec<u8>> = vec![vec![0; y_map.len()]; x_map.len()];

    let num_tiles = tiles.len();
    for i in 0..num_tiles {
        let tile1 = tiles[i];
        let tile2 = tiles[(i + 1) % tiles.len()];

        let (x1, y1) = tile1;
        let (x2, y2) = tile2;

        let (x_min, x_max) = (min(x1, x2), max(x1, x2));
        let (y_min, y_max) = (min(y1, y2), max(y1, y2));

        if y1 == y2 {
            let y_idx = *y_map.get(&y1).unwrap();
            let x_idx_min = *x_map.get(&x_min).unwrap();
            let x_idx_max = *x_map.get(&x_max).unwrap();

            (x_idx_min..=x_idx_max).for_each(|x_idx| {
                compressed_grid[x_idx][y_idx] = 1;
            });
        }
        else if x1 == x2 {
            let x_idx = *x_map.get(&x1).unwrap();
            let y_idx_min = *y_map.get(&y_min).unwrap();
            let y_idx_max = *y_map.get(&y_max).unwrap();

            (y_idx_min..=y_idx_max).for_each(|y_idx| {
                compressed_grid[x_idx][y_idx] = 1;
            });
        }
    }
    fill_polygon((0,0), &mut compressed_grid);
    let mut largest_size = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let size = ((tiles[i].0 - tiles[j].0).abs() + 1) * ((tiles[i].1 - tiles[j].1).abs() + 1);
            if size > largest_size && is_inside_polygon(tiles[i], tiles[j], &compressed_grid, &x_map, &y_map) {
                largest_size = size;
            }
        }
    }
    Some(largest_size as u64)
}

#[inline(always)]
fn is_inside_polygon(tile1: (i64, i64), tile2: (i64, i64), tiles_covered: &[Vec<u8>], x_map: &HashMap<i64, usize>, y_map: &HashMap<i64, usize>) -> bool {
    let (x1, y1) = tile1;
    let (x2, y2) = tile2;

    let x_min = *x_map.get(&min(x1, x2)).unwrap();
    let x_max = *x_map.get(&max(x1, x2)).unwrap();
    let y_min = *y_map.get(&min(y1, y2)).unwrap();
    let y_max = *y_map.get(&max(y1, y2)).unwrap();

    for y_axis in tiles_covered.iter().take(x_max + 1).skip(x_min) {
        for &value in y_axis.iter().take(y_max + 1).skip(y_min) {
            if value == 2 {
                return false;
            }
        }
    }
    true
}

fn fill_polygon(location: (usize, usize), tiles_covered: &mut [Vec<u8>]) {
    let mx = tiles_covered.len();
    let my = tiles_covered[0].len();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(location);

    if tiles_covered[location.0][location.1] == 0 {
        tiles_covered[location.0][location.1] = 2;
    }

    let neighbors: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((cx, cy)) = queue.pop_front() {
        for &(dx, dy) in neighbors.iter() {
            let nx = (cx as isize + dx) as usize;
            let ny = (cy as isize + dy) as usize;

            if nx < mx && ny < my && tiles_covered[nx][ny] == 0 {
                tiles_covered[nx][ny] = 2;
                queue.push_back((nx, ny));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
