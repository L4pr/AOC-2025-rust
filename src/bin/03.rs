#![allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let mut result = 0;
    for line in lines {
        let mut joltage = 0;
        let mut bytes= line.as_bytes().to_vec();
        let mut first_location = 0;
        for i in (b'1'..=b'9').rev() {
            if let Some(location) = &bytes.iter().position(|element| *element == i) {
                let mut number = i;
                if *location == bytes.len() - 1 {
                    for j in (b'1'..i).rev() {
                        if let Some(location2) = &bytes.iter().position(|element| *element == j) {
                            number = j;
                            for byte in 0..=*location2 {
                                bytes[byte] = 0;
                            }
                            first_location = *location2;
                            break;
                        }
                    }
                } else {
                    for byte in 0..=*location {
                        bytes[byte] = 0;
                    }
                    first_location = *location;
                }
                joltage = number - b'0';
                break;
            }
        }

        if let Some(location) = &bytes.iter().position_max() {
            let max_number = &bytes.iter().max().unwrap();
            if *location < first_location {
                joltage += (**max_number - b'0') * 10;
            } else {
                joltage *= 10;
                joltage += **max_number - b'0';
            }
        }
        result += joltage as u64;
    }

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
