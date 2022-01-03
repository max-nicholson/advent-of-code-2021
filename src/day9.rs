use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

use aoc_runner_derive::aoc;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn part_1(grid: &[Vec<u32>]) -> u32 {
    let troughs = find_troughs(grid);
    troughs.iter().map(|&(x, y)| grid[y][x] + 1).sum()
}

#[aoc(day9, part2)]
pub fn part_2(grid: &[Vec<u32>]) -> u32 {
    let neighbours: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let troughs = find_troughs(grid);
    let (max_x, max_y) = get_bounds(grid);
    let basins = troughs.iter().map(|&(basin_x, basin_y)| {
        // Each basin has a minimum size of 3-5 (itself, plus all neighbours in bounds)
        let mut size: u32 = 0;
        let mut stack: VecDeque<(usize, usize)> = VecDeque::from([(basin_x, basin_y)]);
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        while !stack.is_empty() {
            let (x, y) = stack.pop_front().unwrap();

            if !seen.insert((x, y)) || grid[y][x] == 9{
                continue;
            };
            size += 1;

            stack.extend(
                neighbours
                    .iter()
                    .map(|(delta_x, delta_y)| (x as i32 + delta_x, y as i32 + delta_y)) // Bounds check
                    .filter(|&(x, y)| x >= 0 && y >= 0 && x < max_x && y < max_y)
                    .map(|(x, y)| (x as usize, y as usize)),
            );
        }
        size
    });
    select_k(basins, 3).into_iter().product()
}

fn select_k(iter: impl Iterator<Item = u32>, k: usize) -> Vec<u32> {
    let mut h = BinaryHeap::new();
    for item in iter {
        h.push(Reverse(item));
        if h.len() > k {
            h.pop();
        }
    }
    h.into_iter().map(|rev| rev.0).collect()
}

fn get_bounds(grid: &[Vec<u32>]) -> (i32, i32) {
    let max_x: i32 = grid[0].len().try_into().unwrap();
    let max_y: i32 = grid.len().try_into().unwrap();

    (max_x, max_y)
}

fn find_troughs(grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let neighbours: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut troughs: Vec<(usize, usize)> = Vec::new();

    let (max_x, max_y) = get_bounds(grid);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if neighbours
                .iter()
                // Find neighbours
                .map(|(delta_x, delta_y)| (x as i32 + delta_x, y as i32 + delta_y))
                // Bounds check
                .filter(|&(x, y)| x >= 0 && y >= 0 && x < max_x && y < max_y)
                // Find smaller neighbour
                .find(|&(x, y)| grid[y as usize][x as usize] <= cell)
                == None
            {
                troughs.push((x, y));
            }
        }
    }
    troughs
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)), 1134);
    }
}
