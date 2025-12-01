#![allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    // normal_part_one(input)
    fast_part_one(input)
}

#[allow(dead_code)]
fn normal_part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let lines = get_lines(input);
    let mut current_dial = 50;
    for line in lines {
        let bytes = line.as_bytes();
        let sign = ((bytes[0] & 2) as i32) - 1;
        current_dial = (current_dial + sign * fast_parse(&bytes[1..])) % 100;

        if current_dial == 0 {
            result += 1;
        }
    }
    Some(result)
}

#[allow(dead_code)]
fn fast_part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut current_dial = 50;
    while i < len {
        let change = parse_instruction(bytes, &mut i);

        current_dial = (current_dial + change) % 100;

        if current_dial == 0 {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // normal_part_two(input)
    fast_part_two(input)
}

#[allow(dead_code)]
fn normal_part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let lines = get_lines(input);
    let mut current_dial = 50;
    let mut last_was_zero = false;
    for line in lines {
        let bytes = line.as_bytes();
        let sign =
            ((bytes[0] & 2) as i32) - 1;
        current_dial += sign * fast_parse(&bytes[1..]);

        if current_dial >= 100 || current_dial <= 0 {
            result += (current_dial / 100).abs();
            if current_dial <= 0 && !last_was_zero {
                result += 1;
            }
            current_dial %= 100;
            if current_dial < 0 {
                current_dial += 100;
            }
        }
        last_was_zero = current_dial == 0;
    }
    Some(result as u64)
}

#[allow(dead_code)]
fn fast_part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut current_dial = 50;
    let mut last_was_zero = false;
    while i < len {
        let change = parse_instruction(bytes, &mut i);

        current_dial += change;

        if current_dial >= 100 || current_dial <= 0 {
            result += (current_dial / 100).abs();
            if current_dial <= 0 && !last_was_zero {
                result += 1;
            }
            current_dial %= 100;
            if current_dial < 0 {
                current_dial += 100;
            }
        }
        last_was_zero = current_dial == 0;
    }

    Some(result as u64)
}

#[inline(always)]
fn parse_instruction(bytes: &[u8], i: &mut usize) -> i32 {
    let sign = ((bytes[*i] & 2) as i32) - 1;
    *i += 1;

    let mut num = 0;
    while *i < bytes.len() {
        let b = bytes[*i];
        if !b.is_ascii_digit() { break; }
        num = num * 10 + (b - b'0') as i32;
        *i += 1;
    }

    while *i < bytes.len() && bytes[*i] < b'0' {
        *i += 1;
    }

    sign * num
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
        assert_eq!(result, Some(6));
    }
}
