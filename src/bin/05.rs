#![allow(unused_imports)]

use std::cmp::max;
use std::collections::HashSet;
use std::path::Component::ParentDir;
use advent_of_code::*;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut at_ranges = true;
    for line in input.lines() {
        if line.is_empty() {
            at_ranges = false;
            continue;
        }
        if at_ranges {
            let bytes = line.as_bytes();
            let mut split_point_found = false;
            let mut number1 = 0;
            let mut number2 = 0;
            for &b in bytes {
                if b == b'-' {
                    split_point_found = true;
                    continue;
                }
                if !split_point_found {
                    number1 = number1 * 10 + (b - b'0') as u64
                } else {
                    number2 = number2 * 10 + (b - b'0') as u64
                }
            }
            ranges.push((number1, number2));
        } else {
            let bytes = line.as_bytes();
            let mut n = 0;
            for &b in bytes {
                n = n * 10 + (b - b'0') as u64;
            }
            ids.push(n);
        }
    }

    let mut new_ranges: Vec<(u64, u64)> = Vec::new();

    ranges.sort_by_key(|r| r.0);

    for range in &ranges {
        if new_ranges.is_empty() {
            new_ranges.push((range.0, range.1));
            continue;
        }

        let last_index = new_ranges.len() - 1;
        let last_range = &mut new_ranges[last_index];

        if range.0 <= last_range.1 + 1 {
            last_range.1 = max(last_range.1, range.1);
        } else {
            new_ranges.push((range.0, range.1));
        }
    }

    let mut result = 0;
    for id in ids {
        let mut low = 0;
        let mut high = new_ranges.len() - 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let mid_val = new_ranges[mid];


            if id >= mid_val.0 && id <= mid_val.1 {
                result += 1;
                break;
            } else if mid_val.1 < id {
                low = mid + 1;
            } else {
                if mid == 0 { break; }
                high = mid - 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let bytes = line.as_bytes();
        let mut split_point_found = false;
        let mut number1 = 0;
        let mut number2 = 0;
        for i in 0..bytes.len() {
            if bytes[i] == b'-' {
                split_point_found = true;
                continue;
            }
            if !split_point_found {
                number1 = number1 * 10 + (bytes[i] - b'0') as u64
            } else {
                number2 = number2 * 10 + (bytes[i] - b'0') as u64
            }
        }
        ranges.push((number1, number2));
    }

    let mut result = 0;
    let mut new_ranges: Vec<(u64, u64)> = Vec::new();

    ranges.sort_by_key(|r| r.0);

    for range in &ranges {
        if new_ranges.is_empty() {
            new_ranges.push((range.0, range.1));
            continue;
        }

        let last_index = new_ranges.len() - 1;
        let last_range = &mut new_ranges[last_index];

        if range.0 <= last_range.1 + 1 {
            last_range.1 = max(last_range.1, range.1);
        } else {
            new_ranges.push((range.0, range.1));
        }
    }

    for range in new_ranges {
        result += range.1 - range.0 + 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
