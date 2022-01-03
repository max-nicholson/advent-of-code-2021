use std::collections::HashSet;

use aoc_runner_derive::aoc;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn boundaries(grid: &[Vec<u32>]) -> (usize, usize) {
    let width = grid[0].len();
    let height = grid.len();
    (width, height)
}

#[aoc(day11, part1)]
pub fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = parse_input(input);
    let (_final_grid, flashes) =
        (0..100)
            .into_iter()
            .fold((grid, 0usize), |(mut grid, total_flashes), _step| {
                let flashes = compute_flashes(&mut grid);
                (grid, total_flashes + flashes)
            });
    flashes
}

#[aoc(day11, part2)]
pub fn part_2(input: &str) -> usize {
    let mut grid: Vec<Vec<u32>> = parse_input(input);
    let (width, height) = boundaries(&grid);
    let mut steps_until_simultaneous_flash: usize = 0;

    for i in 0.. {
        let flashes = compute_flashes(&mut grid);
        if flashes == width * height {
            steps_until_simultaneous_flash = i + 1;
            break;
        }
    }
    steps_until_simultaneous_flash
}

fn compute_flashes(grid: &mut [Vec<u32>]) -> usize {
    let (width, height) = boundaries(grid);

    let mut stack: Vec<(usize, usize)> = Vec::new();
    // Need to avoid borrowing grid here
    #[allow(clippy::needless_range_loop)]
    for row in 0..height {
        for column in 0..width {
            grid[row][column] += 1;
            if grid[row][column] > 9 {
                stack.push((column, row));
            }
        }
    }

    let mut flashes = 0usize;
    let mut has_flashed: HashSet<(usize, usize)> = HashSet::new();

    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();

        if !has_flashed.insert((x, y)) {
            // We might have put a point onto the stack, which since its addition has already flashed
            continue;
        }

        flashes += 1;
        grid[y][x] = 0;

        let neighbours = get_neighbours((x, y), (width, height));
        neighbours.into_iter().for_each(|(x, y)| {
            if grid[y][x] != 0 {
                grid[y][x] += 1;
                if grid[y][x] > 9 {
                    stack.push((x, y));
                }
            };
        });
    }
    flashes
}

fn get_neighbours((x, y): (usize, usize), (width, height): (usize, usize)) -> Vec<(usize, usize)> {
    let deltas: Vec<(i32, i32)> = vec![
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    deltas
        .into_iter()
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|&(x, y)| x >= 0 && x < width as i32 && y >= 0 && y < height as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_get_neighbours() {
        assert_eq!(
            HashSet::<(usize, usize)>::from_iter(get_neighbours((2, 0), (10, 10))),
            HashSet::<(usize, usize)>::from_iter([(1, 0), (3, 0), (1, 1), (2, 1), (3, 1)])
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 1656);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 195);
    }
}
