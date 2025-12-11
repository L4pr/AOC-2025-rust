#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};
use regex::Regex;
use advent_of_code::*;

advent_of_code::solution!(10);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StateLights {
    pub cost: u64,
    pub lights: u64,
}

impl Ord for StateLights {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.lights.cmp(&other.lights))
    }
}

impl PartialOrd for StateLights {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(clippy::explicit_counter_loop)]
pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut result = 0;

    let mut visited: Vec<u32> = vec![0; 1 << 20];
    let mut generation: u32 = 0;

    for line in lines {
        generation += 1;

        let first_split: (&str, &str)  = line.split("] ").collect_tuple().unwrap();
        let mut goal_lights: u32 = 0;
        for &byte in first_split.0.as_bytes().iter().skip(1).rev() {
            goal_lights <<= 1;
            if byte == b'#' {
                goal_lights |= 1;
            }
        }
        let masks = first_split.1
            .split_ascii_whitespace()
            .rev()
            .skip(1)
            .map(|change| {
                let mut mask: u32 = 0;
                let bytes = change.as_bytes();
                let inner = &bytes[1..bytes.len() - 1];
                for &b in inner {
                    if b != b',' {
                        mask += 1 << (b - b'0');
                    }
                }
                mask
            })
            .collect_vec();
        let mut deque: VecDeque<(u32, u32)> = VecDeque::new();
        visited[0] = generation;
        deque.push_back((0, 0));
        while let Some((cost, lights)) = deque.pop_front() {
            if lights == goal_lights {
                result += cost as u64;
                break;
            }
            let next_cost = cost + 1;
            masks.iter().for_each(|&mask| {
                let next_state = lights ^ mask;
                let idx = next_state as usize;

                if visited[idx] != generation {
                    visited[idx] = generation;
                    deque.push_back((next_cost, next_state));
                }
            })
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;

    let re_curly = Regex::new(r"\{([^}]+)}").unwrap();
    let re_paren = Regex::new(r"\(([^)]+)\)").unwrap();

    for line in input.lines() {
        let caps = re_curly.captures(line).unwrap();
        let targets: Vec<f64> = caps[1].split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let num_equations = targets.len();

        let mut buttons: Vec<Vec<f64>> = Vec::new();

        for caps in re_paren.captures_iter(line) {
            let content = &caps[1];
            let mut col = vec![0.0; num_equations];
            for idx_str in content.split(',') {
                let idx: usize = idx_str.trim().parse().unwrap();
                col[idx] = 1.0;
            }
            buttons.push(col);
        }

        let mut vars = variables!();

        let press_counts: Vec<_> = (0..buttons.len())
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        let objective: Expression = press_counts.iter().sum();

        let mut problem = vars
            .minimise(objective)
            .using(default_solver);

        for row_idx in 0..num_equations {
            let mut lhs = Expression::from(0);
            for (btn_idx, button_vec) in buttons.iter().enumerate() {
                let coeff = button_vec[row_idx];
                if coeff > 0.0 {
                    lhs += press_counts[btn_idx] * coeff;
                }
            }

            problem = problem.with(lhs.eq(targets[row_idx]));
        }

        if let Ok(solution) = problem.solve() {
            let presses: f64 = press_counts.iter()
                .map(|&v| solution.value(v))
                .sum();

            result += presses.round() as u64;
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
