#![allow(unused_imports)]

use std::collections::HashSet;
use advent_of_code::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let mut result = 0;

    for (start, end) in ranges {
        let start_len = start.checked_ilog10().unwrap() + 1;
        let end_len = end.checked_ilog10().unwrap() + 1;

        for len in start_len..=end_len {
            if len & 1 != 0 { continue }

            let seed_len = len / 2;
            let shift = 10_u64.pow(len - seed_len);
            let min_seed = if len == start_len { start / shift } else { 10_u64.pow(seed_len - 1) };
            let max_seed = if len == end_len { end / shift } else { 10_u64.pow(seed_len) - 1 };
            let multiplier = 10_u64.pow(seed_len);

            for seed in min_seed..=max_seed {
                let candidate = seed * multiplier + seed;

                if candidate >= start && candidate <= end {
                    result += candidate;
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);

    let mut found_numbers = Vec::with_capacity(1000);

    for (start, end) in ranges {
        let start_len = start.checked_ilog10().unwrap() + 1;
        let end_len = end.checked_ilog10().unwrap() + 1;

        for len in start_len..=end_len {
            for k in 2..=len {
                if len % k != 0 { continue }

                let seed_len = len / k;
                let shift = 10_u64.pow(len - seed_len);
                let min_seed = if len == start_len { start / shift } else { 10_u64.pow(seed_len - 1) };
                let max_seed = if len == end_len { end / shift } else { 10_u64.pow(seed_len) - 1 };
                let multiplier = 10_u64.pow(seed_len);

                for seed in min_seed..=max_seed {
                    let mut candidate = seed;
                    for _ in 1..k {
                        candidate = candidate * multiplier + seed;
                    }

                    if candidate >= start && candidate <= end {
                        found_numbers.push(candidate);
                    }
                }
            }
        }
    }

    found_numbers.sort_unstable();
    found_numbers.dedup();

    Some(found_numbers.iter().sum())
}

#[inline(always)]
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input.split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let (start, end) = range.trim().split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
