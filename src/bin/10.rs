#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
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

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let first_split: (&str, &str)  = line.split("] ").collect_tuple().unwrap();
        let mut goal_lights: u64 = 0;
        first_split.0.as_bytes().iter().skip(1).rev().for_each(|&byte| {
            goal_lights <<= 1;
            if byte == b'#' {
                goal_lights += 1;
            }
        });
        let masks = first_split.1
            .split_ascii_whitespace()
            .rev()
            .skip(1)
            .map(|change| {
                let mut mask: u64 = 0;
                change.as_bytes().iter().skip(1).rev().skip(1).for_each(|&byte| {
                    if byte != b',' {
                        mask += 1 << (byte - b'0');
                    }
                });
                mask
            })
            .collect_vec();
        let mut priority_queue: BinaryHeap<StateLights> = BinaryHeap::new();
        priority_queue.push(StateLights {cost: 0, lights: 0});
        let mut positions_been: HashSet<u64> = HashSet::new();
        while let Some(state) = priority_queue.pop() {
            if state.lights == goal_lights {
                result += state.cost;
                break;
            }
            if !positions_been.insert(state.lights) {
                continue;
            }
            masks.iter().for_each(|&mask| {
                priority_queue.push(StateLights {cost: state.cost + 1, lights: state.lights ^ mask});
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
