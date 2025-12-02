#![allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(',').map(|range| {
        let (start, end) = range.split('-').collect_tuple().unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    }).collect_vec();
    let mut result = 0;
    for range in ranges {
        for number in range.0..=range.1 {
            if is_invalid(&number.to_string()) {
                result += number;
            }
        }
    }
    Some(result)
}

fn is_invalid(number: &str) -> bool {
    if !number.len().is_multiple_of(2) {
        return false;
    }
    let parts = number.split_at(number.len() / 2);
    if  parts.0 == parts.1 {
        return true;
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split(',').map(|range| {
        let (start, end) = range.split('-').collect_tuple().unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    }).collect_vec();
    let mut result = 0;
    for range in ranges {
        for number in range.0..=range.1 {
            if part2_is_invalid(&number.to_string()) {
                result += number;
            }
        }
    }
    Some(result)
}

fn part2_is_invalid(number: &str) -> bool {
    let len = number.len();

    for k in 2..=len {
        if len % k == 0 {
            let chunk_size = len / k;
            let pattern = &number.as_bytes()[..chunk_size];

            if number.as_bytes().chunks(chunk_size).all(|chunk| chunk == pattern) {
                return true;
            }
        }
    }
    false
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
