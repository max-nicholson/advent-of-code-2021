use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (usize, usize);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

struct Visit {
    point: Point,
    distance: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: Reverse order so we get MinHeap behaviour
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Eq for Visit {}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn possible_moves(from: Point, bounds: (isize, isize)) -> Vec<Point> {
    let point = (from.0 as isize, from.1 as isize);
    DIRECTIONS
        .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
        .into_iter()
        .filter(|(x, y)| x >= &0 && x <= &bounds.0 && y >= &0 && y <= &bounds.1)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn dijkstra(grid: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let end = (grid[0].len() - 1, grid.len() - 1);
    let bounds = (end.0 as isize, end.1 as isize);

    let mut distances = vec![vec![u32::MAX; end.0 + 1]; end.1 + 1];
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances[0][0] = 0;
    to_visit.push(Visit {
        point: (0, 0),
        distance: 0,
    });

    while let Some(Visit { point, distance }) = to_visit.pop() {
        if !visited.insert(point) {
            continue;
        }

        for neighbour in possible_moves(point, bounds) {
            let cost = grid[neighbour.1][neighbour.0];
            let new_distance = distance + cost;

            if new_distance < distances[neighbour.1][neighbour.0] {
                distances[neighbour.1][neighbour.0] = new_distance;
                to_visit.push(Visit {
                    point: neighbour,
                    distance: new_distance,
                })
            }
        }
    }

    distances
}

#[aoc(day15, part1)]
pub fn part_1(grid: &[Vec<u32>]) -> u32 {
    let distances = dijkstra(grid);

    distances[distances[0].len() - 1][distances.len() - 1]
}

#[aoc(day15, part2)]
pub fn part_2(grid: &[Vec<u32>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut larger_grid = vec![vec![0; cols * 5]; rows * 5];

    for row in 0..rows {
        for col in 0..cols {
            for i in 0..5usize {
                for j in 0..5usize {
                    let increment = i + j;
                    let mut new_value = grid[row][col] + (increment as u32);
                    if new_value > 9 {
                        new_value -= 9;
                    }

                    larger_grid[rows * i + row][cols * j + col] = new_value;
                }
            }
        }
    }

    let distances = dijkstra(&larger_grid);

    distances[distances.len() - 1][distances[0].len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)), 315);
    }
}
