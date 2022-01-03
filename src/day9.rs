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
    let row = &grid[0];
    let mut troughs = Vec::new();
    let neighbours: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    let max_x: i32 = row.len().try_into().unwrap();
    let max_y: i32 = grid.len().try_into().unwrap();

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
                troughs.push(cell);
            }
        }
    }
    troughs.iter().map(|height| height + 1).sum()
}

#[aoc(day9, part2)]
pub fn part_2(input: &[Vec<u32>]) -> u32 {
    todo!();
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
        //assert_eq!(part_2(&parse_input(EXAMPLE)), 61229);
    }
}
