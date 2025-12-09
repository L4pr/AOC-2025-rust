#![allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist_sq(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) +
            (self.y - other.y).pow(2) +
            (self.z - other.z).pow(2)
    }
}

fn find(p: &mut Vec<usize>, i: usize) -> usize {
    if p[i] != i {
        p[i] = find(p, p[i]);
    }
    p[i]
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = input.lines().map(|line| {
        let coords: (i64, i64, i64) = line.split(',').map(|n| n.parse::<i64>().unwrap()).collect_tuple().unwrap();
        Point {x: coords.0, y: coords.1, z: coords.2}
    }).collect_vec();
    let num_points = points.len();
    let mut edges = Vec::new();

    for i in 0..num_points {
        for j in (i + 1)..num_points {
            let distance = points[i].dist_sq(&points[j]);
            edges.push((distance, i, j));
        }
    }

    edges.sort_unstable_by_key(|key| key.0);

    let mut parent: Vec<usize> = (0..num_points).collect();
    let mut group_size: Vec<usize> = vec![1; num_points];

    let max_edges_limit = if cfg!(test) {
        10
    } else {
        1000
    };

    for &(_, u, v) in edges.iter().take(max_edges_limit) {
        let root_u = find(&mut parent, u);
        let root_v = find(&mut parent, v);

        if root_u != root_v {
            if group_size[root_u] < group_size[root_v] {
                parent[root_u] = root_v;
                group_size[root_v] += group_size[root_u];
            } else {
                parent[root_v] = root_u;
                group_size[root_u] += group_size[root_v];
            }
        }
    }

    let mut final_sizes: Vec<u64> = parent.iter().enumerate()
        .filter(|&(i, &p)| i == p)
        .map(|(i, _)| group_size[i] as u64)
        .collect();

    final_sizes.sort_unstable_by(|a, b| b.cmp(a));

    Some(final_sizes[0] * final_sizes[1] * final_sizes[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input.lines().map(|line| {
        let coords: (i64, i64, i64) = line.split(',').map(|n| n.parse::<i64>().unwrap()).collect_tuple().unwrap();
        Point {x: coords.0, y: coords.1, z: coords.2}
    }).collect_vec();
    let num_points = points.len();
    let mut edges = Vec::new();

    for i in 0..num_points {
        for j in (i + 1)..num_points {
            let distance = points[i].dist_sq(&points[j]);
            edges.push((distance, i, j));
        }
    }

    edges.sort_unstable_by_key(|key| key.0);

    let mut parent: Vec<usize> = (0..num_points).collect();
    let mut group_size: Vec<usize> = vec![1; num_points];

    for &(_, u, v) in edges.iter() {
        let root_u = find(&mut parent, u);
        let root_v = find(&mut parent, v);

        if root_u != root_v {
            parent[root_u] = root_v;
            group_size[root_v] += group_size[root_u];
        }

        if group_size[root_v] == num_points {
            return Some((points[u].x * points[v].x) as u64);
        }
    }

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
