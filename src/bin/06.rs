#![allow(unused_imports)]

use std::cmp::max;
use advent_of_code::*;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let mut results: Vec<u64> = Vec::new();

    let operations = lines[lines.len() - 1].split_whitespace().collect_vec();
    let length = operations.len();

    for line_idx in 0..lines.len() - 1 {
        if line_idx == 0 {
            lines[line_idx].split_whitespace()
                .for_each(|num_str| results.push(num_str.parse::<u64>().unwrap()));
            continue;
        }
        let line_split = lines[line_idx].split_whitespace().map(|str| str.parse::<u64>().unwrap()).collect_vec();
        for i in 0..length {
            if operations[i] == "+" {
                results[i] += line_split[i];
            } else {
                results[i] *= line_split[i];
            }
        }
    }

    let mut result = 0;
    for num in results {
        result += num;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let mut all_numbers: Vec<Vec<u64>> = Vec::new();

    let operations = lines[lines.len() - 1].split_whitespace().collect_vec();
    let mut left_indexes = Vec::new();
    let bytes = lines[lines.len() - 1].as_bytes();
    let mut byte_length = bytes.len();
    for i in 0..byte_length {
        let byte = bytes[i];
        if byte != b' ' {
            left_indexes.push(i);
            all_numbers.push(Vec::new())
        }
    }

    let mut lines_bytes = Vec::new();
    let lines_length = lines.len() - 1;
    for line_idx in 0..lines_length {
        let line_bytes = lines[line_idx].as_bytes();
        lines_bytes.push(line_bytes);
        byte_length = max(byte_length, line_bytes.len());
    }
    left_indexes.push(byte_length + 1);

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
        all_numbers[current_column].push(num);
    }

    let mut results = Vec::new();

    for i in 0..all_numbers.len() {
        if operations[i] == "+" {
            let mut number = 0;
            for n in &all_numbers[i] {
                number += n;
            }
            results.push(number);
        } else {
            let mut number = 1;
            for n in &all_numbers[i] {
                number *= n;
            }
            results.push(number);
        }
    }

    let mut result = 0;
    for num in results {
        result += num;
    }

    Some(result)
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
