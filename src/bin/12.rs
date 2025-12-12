#![allow(unused_imports)]

use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use advent_of_code::*;

advent_of_code::solution!(12);

#[inline(always)]
fn count_hashes(bytes: &[u8]) -> usize {
    let mut c = 0_usize;
    for &b in bytes {
        c += (b == b'#') as usize;
    }
    c
}

#[allow(clippy::needless_range_loop)]
pub fn part_one(input: &str) -> Option<u64> {
    let mut parts = input.split("\n\n");
    let mut sizes = [0usize; 6];

    for i in 0..6 {
        if let Some(part) = parts.next() {
            sizes[i] = count_hashes(part.as_bytes());
        }
    }
    let mut result =  0;

    for line in parts.next()?.lines() {
        let b = line.as_bytes();
        let len = b.len();
        let mut cursor = 0;

        let mut w = 0;
        while cursor < len && b[cursor].is_ascii_digit() {
            w = w * 10 + (b[cursor] - b'0') as usize;
            cursor += 1;
        }
        cursor += 1;

        let mut h = 0;
        while cursor < len && b[cursor].is_ascii_digit() {
            h = h * 10 + (b[cursor] - b'0') as usize;
            cursor += 1;
        }
        cursor += 2;

        let total_size = w * h;

        let mut needed_size = 0;
        let mut size_idx = 0;
        let mut current_num = 0;

        while cursor < len {
            let byte = b[cursor];
            if byte == b' ' {
                needed_size += current_num * sizes[size_idx];
                size_idx += 1;
                current_num = 0;
            } else {
                current_num = current_num * 10 + (byte - b'0') as usize;
            }
            cursor += 1;
        }
        needed_size += current_num * sizes[size_idx];

        if needed_size < total_size {
            result += 1;
        }
    }
    Some(result)
}


#[derive(Clone, Debug)]
struct RawShape {
    id: usize,
    coords: Vec<(usize, usize)>
}

#[derive(Clone, Debug)]
struct Query {
    width: usize,
    height: usize,
    counts: Vec<usize>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ShapeMask {
    rows: Vec<u64>,
    area: usize,
}

#[derive(Clone)]
struct SolverState {
    grid: Vec<u64>,
    height: usize,
    width: usize
}

impl SolverState {
    fn new(width: usize, height: usize) -> Self {
        Self { grid: vec![0; height], height, width }
    }

    #[inline(always)]
    fn fits(&self, r: usize, c: usize, shape: &ShapeMask) -> bool {
        if r + 3 > self.height || c + 3 > self.width {
            return false;
        }
        for i in 0..3 {
            let shape_row_mask = shape.rows[i] << c;
            if (self.grid[r + i] & shape_row_mask) != 0 {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    fn toggle(&mut self, r: usize, c: usize, shape: &ShapeMask) {
        for i in 0..3 {
            self.grid[r + i] ^= shape.rows[i] << c;
        }
    }
}

fn parse_input(input: &str) -> (Vec<RawShape>, Vec<Query>) {
    let mut shapes = Vec::new();
    let mut queries = Vec::new();

    let lines = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    let mut current_shape_id = None;
    let mut current_shape_lines = Vec::new();

    for line in lines {
        if line.contains(':') {
            if line.contains('x') {
                if let Some(id) = current_shape_id {
                    shapes.push(parse_shape_grid(id, &current_shape_lines));
                    current_shape_id = None;
                    current_shape_lines.clear();
                }

                let parts: Vec<&str> = line.split(": ").collect();
                let dims: Vec<usize> = parts[0].split('x').map(|n| n.parse().unwrap()).collect();

                let counts: Vec<usize> = parts[1]
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();

                queries.push(Query {
                    width: dims[0],
                    height: dims[1],
                    counts,
                });
            } else {
                if let Some(id) = current_shape_id {
                    shapes.push(parse_shape_grid(id, &current_shape_lines));
                    current_shape_lines.clear();
                }
                let idx_str = line.strip_suffix(':').unwrap();
                current_shape_id = Some(idx_str.parse::<usize>().unwrap());
            }
        } else if current_shape_id.is_some() {
            current_shape_lines.push(line);
        }
    }

    (shapes, queries)
}

fn parse_shape_grid(id: usize, lines: &[&str]) -> RawShape {
    let mut coords = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                coords.push((r, c));
            }
        }
    }
    RawShape { id, coords }
}

fn generate_variants(raw: &RawShape) -> Vec<ShapeMask> {
    let mut unique_masks = HashSet::new();
    let mut variants = Vec::new();

    let base_coords: Vec<(i32, i32)> = raw.coords.iter().map(|&(r, c)| (r as i32, c as i32)).collect();
    let mut current = base_coords.clone();

    for _flip in 0..2 {
        for _rot in 0..4 {
            let min_r = current.iter().map(|p| p.0).min().unwrap();
            let min_c = current.iter().map(|p| p.1).min().unwrap();

            let normalized: Vec<(usize, usize)> = current.iter()
                .map(|p| ((p.0 - min_r) as usize, (p.1 - min_c) as usize))
                .collect();

            let mut rows = vec![0u64; 3];
            for (r, c) in &normalized {
                rows[*r] |= 1u64 << c;
            }

            if unique_masks.insert(rows.clone()) {
                variants.push(ShapeMask {
                    rows,
                    area: raw.coords.len(),
                });
            }
            current = current.iter().map(|(r, c)| (*c, -r)).collect();
        }
        current = current.iter().map(|(r, c)| (-r, *c)).collect();
    }
    variants
}

fn solve_part_one_recursive(
    type_idx: usize,
    count_placed: usize,
    start_pos: usize,
    board: &mut SolverState,
    variants_by_type: &[Vec<ShapeMask>],
    requirements: &[(usize, usize)],
) -> bool {
    if type_idx >= requirements.len() {
        return true;
    }

    let (type_id, needed) = requirements[type_idx];

    if count_placed == needed {
        return solve_part_one_recursive(type_idx + 1, 0, 0, board, variants_by_type, requirements);
    }

    let possible_shapes = &variants_by_type[type_id];
    let max_pos = board.width * board.height;

    for pos in start_pos..max_pos {
        let r = pos / board.width;
        let c = pos % board.width;

        for shape in possible_shapes {
            if board.fits(r, c, shape) {
                board.toggle(r, c, shape);

                if solve_part_one_recursive(type_idx, count_placed + 1, pos, board, variants_by_type, requirements) {
                    return true;
                }

                board.toggle(r, c, shape);
            }
        }
    }

    false
}

pub fn part_one_original(input: &str) -> Option<u64> {
    let (raw_shapes, queries) = parse_input(input);

    let mut all_variants = vec![Vec::new(); raw_shapes.len()];
    for shape in &raw_shapes {
        all_variants[shape.id] = generate_variants(shape);
    }

    let success_count = AtomicU64::new(0);

    queries.par_iter().for_each(|query| {
        let area_grid = query.width * query.height;
        let mut area_presents = 0;
        let mut req_list = Vec::new();

        for (id, &count) in query.counts.iter().enumerate() {
            if count > 0 {
                let shape_area = all_variants[id][0].area;
                area_presents += shape_area * count;
                req_list.push((id, count));
            }
        }

        if area_presents > area_grid {
            return;
        }

        req_list.sort_by(|a, b| {
            let area_a = all_variants[a.0][0].area;
            let area_b = all_variants[b.0][0].area;
            area_b.cmp(&area_a)
        });

        let mut board = SolverState::new(query.width, query.height);
        if solve_part_one_recursive(0, 0, 0, &mut board, &all_variants, &req_list) {
            success_count.fetch_add(1, Ordering::Relaxed);
        }
    });

    Some(success_count.load(Ordering::Relaxed))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_original(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
