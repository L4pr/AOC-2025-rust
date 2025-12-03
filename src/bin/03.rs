#![allow(unused_imports)]

use std::cmp::max;
use std::thread::current;
use advent_of_code::*;

advent_of_code::solution!(3);

#[allow(clippy::needless_range_loop)]
pub fn part_one(input: &str) -> Option<u64> {
    Some(input
        .lines()
        .map(|line| {
            let digits = line.as_bytes();

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

            ((max_digit1 - b'0') * 10 + (max_digit2 - b'0')) as u64
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let bytes= line.as_bytes();

            let mut numbers: [u8; 12] = [0; 12];
            let length = bytes.len();

            let mut position = 0;

            for i in 0..12 {
                let last_valid_index = length - (12 - i);
                for j in position..=last_valid_index {
                    if bytes[j] > numbers[i] {
                        numbers[i] = bytes[j];
                        position = j + 1;
                        if bytes[j] == b'9' {
                            break;
                        }
                    }
                }
            }

            let mut number = 0;
            for byte in numbers {
                number = number * 10 + (byte - b'0') as u64;
            }
            number
        })
        .sum();

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
