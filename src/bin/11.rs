#![allow(unused_imports)]

use std::collections::{HashMap, VecDeque};
use std::path::Path;
use advent_of_code::*;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut in_outputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut positions_been: HashMap<&str, u64> = HashMap::new();
    for line in input.lines() {
        let (into, out) = line.split(": ").collect_tuple().unwrap();
        let output = out.split_ascii_whitespace().collect_vec();
        in_outputs.insert(into, output);
    }

    Some(do_part_one("you", &in_outputs, &mut positions_been))
}

fn do_part_one<'a>(input: &str, in_outputs: &HashMap<&'a str, Vec<&str>>, positions_been: &mut HashMap<&'a str, u64>) -> u64 {
    if input == "out" {
        return 1;
    }
    if let Some(&value) = positions_been.get(input) {
        return value;
    }
    let (stored_key, togo) = in_outputs.get_key_value(input).unwrap();

    let mut result: u64 = 0;
    for path in togo {
        let value = do_part_one(path, in_outputs, positions_been);
        result += value;
    }
    positions_been.insert(*stored_key, result);
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut in_outputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut positions_been: HashMap<(&str, (bool, bool)), u64> = HashMap::new();
    for line in input.lines() {
        let (into, out) = line.split(": ").collect_tuple().unwrap();
        let output = out.split_ascii_whitespace().collect_vec();
        in_outputs.insert(into, output);
    }

    Some(do_part_two("svr", (false, false), &in_outputs, &mut positions_been))
}

fn do_part_two<'a>(input: &str, been: (bool, bool), in_outputs: &HashMap<&'a str, Vec<&str>>, positions_been: &mut HashMap<(&'a str, (bool, bool)), u64>) -> u64 {
    if input == "out" {
        return if been.0 && been.1 {
            1
        } else {
            0
        }
    }
    if let Some(&value) = positions_been.get(&(input, been)) {
        return value;
    }
    let (stored_key, togo) = in_outputs.get_key_value(input).unwrap();
    let mut new_been = been;
    if input == "fft" {
        new_been.0 = true;
    } else if input == "dac" {
        new_been.1 = true;
    }
    let mut result: u64 = 0;
    for path in togo {
        let value = do_part_two(path, new_been, in_outputs, positions_been);
        result += value;
    }
    positions_been.insert((*stored_key, been), result);
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
