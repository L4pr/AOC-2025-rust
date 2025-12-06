#![allow(unused_imports)]

use std::cmp::max;
use advent_of_code::*;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let operations = lines[lines.len() - 1].split_ascii_whitespace().collect_vec();
    let length = operations.len();
    let mut results: Vec<u64> = Vec::with_capacity(length);

    for line_idx in 0..lines.len() - 1 {
        if line_idx == 0 {
            lines[line_idx]
                .split_ascii_whitespace()
                .for_each(|num_str| results.push(fast_parse_u64(num_str.as_bytes())));
            continue;
        }
        lines[line_idx]
            .split_whitespace()
            .map(|str| fast_parse_u64(str.as_bytes()))
            .enumerate()
            .for_each({|(idx, num)|
            if operations[idx] == "+" {
                results[idx] += num;
            } else {
                results[idx] *= num;
            }
        });
    }

    Some(results.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let mut operations = Vec::new();
    let mut left_indexes = Vec::new();
    let lines_length = lines.len() - 1;

    let bytes = lines[lines_length].as_bytes();
    let mut byte_length = bytes.len();
    for i in 0..byte_length {
        let byte = bytes[i];
        if byte != b' ' {
            left_indexes.push(i);
            operations.push(byte);
        }
    }

    let mut lines_bytes = Vec::new();
    for line_idx in 0..lines_length {
        let line_bytes = lines[line_idx].as_bytes();
        lines_bytes.push(line_bytes);
        byte_length = max(byte_length, line_bytes.len());
    }
    left_indexes.push(byte_length + 1);

    let mut results = Vec::new();
    let mut current_column = 0;
    for i in 0..byte_length {
        if i == left_indexes[current_column + 1] - 1 {
            current_column += 1;
            continue;
        }
        let mut num = 0;
        for j in 0..lines_length {
            let line_bytes = lines_bytes[j];
            if i == line_bytes.len() {
                continue;
            }
            let byte = line_bytes[i];
            if byte != b' ' {
                num = num * 10 + (byte - b'0') as u64;
            }
        }
        if operations[current_column] == b'+' {
            if i == left_indexes[current_column] {
                results.push(num);
            } else {
                results[current_column] += num;
            }
        } else if i == left_indexes[current_column] {
            results.push(num);
        } else {
            results[current_column] *= num;
        }
    }

    Some(results.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
