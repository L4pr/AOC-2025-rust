#![allow(unused_imports)]

use std::cmp::{max, min};
use std::collections::VecDeque;
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
    let max_x = tiles.iter().map(|t| t.0).max().unwrap();
    let max_y = tiles.iter().map(|t| t.1).max().unwrap();
    let mut tiles_covered: Vec<Vec<u8>> = vec![vec![0; (max_y + 2) as usize]; (max_x + 2) as usize];
    for i in 0..tiles.len() {
        let tile1 = tiles[i];
        let tile2 = tiles[(i + 1) % tiles.len()];
        let (x1, y1) = tile1;
        let (x2, y2) = tile2;

        let (x_min, x_max) = (min(x1, x2), max(x1, x2));
        let (y_min, y_max) = (min(y1, y2), max(y1, y2));

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                tiles_covered[x as usize][y as usize] = 1;
            }
        }
    }
    let mut start_tile_filling = (0, max_y / 2);
    for i in 0..max_x {
        if tiles_covered[i as usize][start_tile_filling.1 as usize] == 0 {
            let mut edge_hits = 0;
            for mut j in i..max_x + 1 {
                let is_boundary_tile = tiles_covered[j as usize][start_tile_filling.1 as usize] == 1;
                if is_boundary_tile {
                    let neighbor = tiles_covered[j as usize - 1][(start_tile_filling.1) as usize];

                    if neighbor == 0 {
                        edge_hits += 1;
                    }
                    while j < max_x && tiles_covered[j as usize][start_tile_filling.1 as usize] == 1 {
                        j += 1;
                    }
                }
            }
            if edge_hits % 2 == 1 {
                start_tile_filling.0 = i;
                break;
            }
        }
    }
    fill_polygon(start_tile_filling, &mut tiles_covered);
    println!("filled");
    let mut largest_size = 0;
    for i in 0..tiles.len() {
        println!("i: {}", i);
        for j in i + 1..tiles.len() {
            let size = ((tiles[i].0 - tiles[j].0).abs() + 1) * ((tiles[i].1 - tiles[j].1).abs() + 1);
            if size > largest_size && is_inside_polygon(tiles[i], tiles[j], &tiles_covered) {
                largest_size = size;
            }
        }
    }
    Some(largest_size as u64)
}

fn is_inside_polygon(tile1: (i64, i64), tile2: (i64, i64), tiles_covered: &Vec<Vec<u8>>) -> bool {
    if tile1.0 < tile2.0 {
        if tile1.1 < tile2.1 {
            for k in tile1.0..=tile2.0 {
                if tiles_covered[k as usize][tile1.1 as usize] == 0 {
                    return false;
                }
                if tiles_covered[k as usize][tile2.1 as usize] == 0 {
                    return false;
                }
            }
            for k in tile1.1..=tile2.1 {
                if tiles_covered[tile1.0 as usize][k as usize] == 0 {
                    return false;
                }
                if tiles_covered[tile2.0 as usize][k as usize] == 0 {
                    return false;
                }
            }
        } else {
            for k in tile1.0..=tile2.0 {
                if tiles_covered[k as usize][tile1.1 as usize] == 0 {
                    return false;
                }
                if tiles_covered[k as usize][tile2.1 as usize] == 0 {
                    return false;
                }
            }
            for k in tile2.1..=tile1.1 {
                if tiles_covered[tile1.0 as usize][k as usize] == 0 {
                    return false;
                }
                if tiles_covered[tile2.0 as usize][k as usize] == 0 {
                    return false;
                }
            }
        }
    } else if tile1.1 < tile2.1 {
        for k in tile2.0..=tile1.0 {
            if tiles_covered[k as usize][tile1.1 as usize] == 0 {
                return false;
            }
            if tiles_covered[k as usize][tile2.1 as usize] == 0 {
                return false;
            }
        }
        for k in tile1.1..=tile2.1 {
            if tiles_covered[tile1.0 as usize][k as usize] == 0 {
                return false;
            }
            if tiles_covered[tile2.0 as usize][k as usize] == 0 {
                return false;
            }
        }
    } else {
        for k in tile2.0..=tile1.0 {
            if tiles_covered[k as usize][tile1.1 as usize] == 0 {
                return false;
            }
            if tiles_covered[k as usize][tile2.1 as usize] == 0 {
                return false;
            }
        }
        for k in tile2.1..=tile1.1 {
            if tiles_covered[tile1.0 as usize][k as usize] == 0 {
                return false;
            }
            if tiles_covered[tile2.0 as usize][k as usize] == 0 {
                return false;
            }
        }
    }
    true
}

fn fill_polygon(location: (i64, i64), tiles_covered: &mut Vec<Vec<u8>>) {
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back(location);

    while let Some((x, y)) = queue.pop_front() {

        let x_usize = x as usize;
        let y_usize = y as usize;

        if tiles_covered[x_usize][y_usize] != 0 {
            continue;
        }

        tiles_covered[x_usize][y_usize] = 2;

        queue.push_back((x + 1, y));
        queue.push_back((x - 1, y));
        queue.push_back((x, y + 1));
        queue.push_back((x, y - 1));
    }
}

fn print_grid_pretty(grid: &Vec<Vec<u8>>) {
    println!("\n--- Corrected Grid Map (Rows = Y, Cols = X) ---");

    let max_x = grid.len();
    if max_x == 0 { return; }
    let max_y = grid[0].len();

    for y in 0..max_y {
        let mut line = String::new();
        for x in 0..max_x {

            let char_to_print = match grid[x][y] {
                1 => '#',
                0 => '.',
                _ => '?',
            };
            line.push(char_to_print);
        }
        println!("{}", line);
    }
    println!("----------------------------------------------\n");
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
