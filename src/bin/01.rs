#![allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let lines = get_lines(input);
    let mut current_dial = 50;
    for line in lines {
        let parts = line.split_at(1);
        if parts.0 == "L" {
            current_dial = (current_dial - parts.1.parse::<i32>().unwrap()) % 100
        } else {
            current_dial = (current_dial + parts.1.parse::<i32>().unwrap()) % 100
        }
        if current_dial == 0 {
            result += 1;
        }
    }



    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let lines = get_lines(input);
    let mut current_dial = 50;
    let mut last_was_zero = false;
    for line in lines {
        let parts = line.split_at(1);
        if parts.0 == "L" {
            current_dial = (current_dial - parts.1.parse::<i32>().unwrap());
        } else {
            current_dial = (current_dial + parts.1.parse::<i32>().unwrap());
        }
        if current_dial >= 100 || current_dial <= 0 {
            result += (current_dial / 100).abs();
            if current_dial <= 0 && !last_was_zero {
                result += 1;
            }
            current_dial = current_dial.rem_euclid(100);
        }
        last_was_zero = false;
        if current_dial == 0 {
            last_was_zero = true;
        }
    }



    Some(result as u64)
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
