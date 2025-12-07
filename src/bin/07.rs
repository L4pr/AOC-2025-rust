#![allow(unused_imports)]

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use advent_of_code::*;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let grid_length = grid.len();
    let mut amount_split = 0;
    let initial_state = State {cost: 0, position: (0, grid[0].len() / 2)};
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(initial_state);
    let mut last_state = State {cost: 0, position: (0, 0)};
    while let Some(state) = queue.pop_front() {
        if state.eq(&last_state) {
            continue;
        }
        last_state = state;
        if state.position.0 == grid_length - 1 {
            continue;
        }
        let new_position = (state.position.0 + 1, state.position.1);
        if grid[new_position.0][new_position.1] == b'^' {
            queue.push_back(State {cost: state.cost + 1, position: (new_position.0, new_position.1 - 1)});
            queue.push_back(State {cost: state.cost + 1, position: (new_position.0, new_position.1 + 1)});
            amount_split += 1;
        } else {
            queue.push_back(State {cost: state.cost + 1, position: new_position});
        }
    }


    Some(amount_split)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let initial_state = (0_usize, grid[0].len() / 2);
    let mut library: HashMap<(usize, usize), u64> = HashMap::new();

    Some(do_part_two(&grid, initial_state, &mut library))
}

fn do_part_two(grid: &Vec<&[u8]>, position: (usize, usize), library: &mut HashMap<(usize, usize), u64>) -> u64 {
    if position.0 == grid.len() - 1 {
        return 1;
    }
    if let Some(library_timelines) = library.get(&position) {
        return *library_timelines;
    }
    if grid[position.0][position.1] == b'^' {
        let timeline_amount = do_part_two(grid, (position.0, position.1 - 1), library) + do_part_two(grid, (position.0, position.1 + 1), library);
        library.insert(position, timeline_amount);
        return timeline_amount;
    }
    let timeline_amount = do_part_two(grid, (position.0 + 1, position.1), library);
    timeline_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
