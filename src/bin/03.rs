#![allow(unused_imports)]

use std::cmp::max;
use std::thread::current;
use advent_of_code::*;

advent_of_code::solution!(3);

#[allow(clippy::needless_range_loop)]
pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .map(|line| {
            let digits= line.as_bytes();

            let length = digits.len();

            let mut max_digit1 = digits[0];
            let mut max_digit1_pos = 0;
            for i in 1..length - 1 {
                let current_digit = digits[i];
                if current_digit > max_digit1 {
                    max_digit1 = current_digit;
                    max_digit1_pos = i;
                }
            }

            let mut max_digit2 = digits[max_digit1_pos + 1];
            for i in max_digit1_pos + 2..length {
                max_digit2 = max(max_digit2, digits[i]);
            }

            (max_digit1 - b'0') as u64 * 10 + (max_digit2 - b'0') as u64
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let mut result = 0;

    for line in lines {
        let bytes= line.as_bytes().to_vec();
        let mut to_remove = bytes.len() - 12;
        let mut stack: Vec<u8> = Vec::new();
        let mut start_pos = 0;
        let max = bytes.iter().max().unwrap();
        let max_pos = bytes.iter().position(|b| b == max).unwrap();
        if bytes.len() - max_pos > 12 {
            start_pos = max_pos;
            to_remove -= start_pos;
        }
        for &digit in bytes.iter().skip(start_pos) {
            while let Some(&last) = stack.last() {
                if to_remove > 0 && digit > last {
                    stack.pop();
                    to_remove -= 1;
                } else {
                    break;
                }
            }
            stack.push(digit);
        }

        stack.truncate(12);

        let mut number = 0;
        for byte in stack {
            number = number * 10 + (byte - b'0') as u64;
        }
        result += number;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
