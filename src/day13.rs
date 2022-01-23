use std::fmt::Error;

use aoc_runner_derive::{aoc, aoc_generator};

type Grid = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Fold {
    axis: char,
    line: u32,
}

impl Fold {
    fn perform_on(&self, grid: &Grid) -> Grid {
        let rows = grid.len();
        let columns = grid[0].len();

        match self.axis {
            'x' => {
                // Fold left
                assert!(self.line * 2 + 1 == columns as u32);
                let width = columns / 2;
                let mut new_grid = vec![vec![false; width]; rows];

                for row in 0..rows {
                    for col in 0..width {
                        new_grid[row][col] = grid[row][col] || grid[row][columns - 1 - col];
                    }
                }
                new_grid
            }
            'y' => {
                // Fold up
                assert!(self.line * 2 + 1 == rows as u32);
                let height = rows / 2;
                let mut new_grid = vec![vec![false; columns]; height];

                for row in 0..height {
                    for col in 0..columns {
                        new_grid[row][col] = grid[row][col] || grid[rows - 1 - row][col];
                    }
                }
                new_grid
            }
            _ => unimplemented!(),
        }
    }
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let mut lines = input.lines();

    let mut coordinates: Vec<(usize, usize)> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    loop {
        match lines.next() {
            Some(line) => match line {
                "" => break,
                _ => {
                    let mut tokens = line.split(',');
                    let x = tokens.next().and_then(|x| x.parse().ok()).expect("x");
                    let y = tokens.next().and_then(|y| y.parse().ok()).expect("y");
                    if y > max_y {
                        max_y = y
                    }
                    if x > max_x {
                        max_x = x
                    }
                    coordinates.push((x, y))
                }
            },
            None => panic!("We shouldn't run out of lines before we get to the fold"),
        }
    }

    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];

    coordinates.into_iter().for_each(|(x, y)| grid[y][x] = true);

    let folds: Vec<Fold> = lines
        .map(|l| {
            let words = l.split(' ');
            let mut tokens = words
                .last()
                .expect("Line format of `fold along axis=line`")
                .split('=');
            let axis = tokens
                .next()
                .expect("Expected cartesian axis")
                .chars()
                .next()
                .unwrap();
            let line = tokens
                .next()
                .and_then(|x| x.parse().ok())
                .expect("Expected line number");
            Fold { axis, line }
        })
        .collect();

    (grid, folds)
}

#[aoc(day13, part1)]
pub fn part_1((grid, folds): &(Grid, Vec<Fold>)) -> Result<usize, Error> {
    let first_fold = &folds[0];
    let new_grid = first_fold.perform_on(grid);
    let dots = new_grid.iter().flatten().filter(|&is_dot| *is_dot).count();
    Ok(dots)
}

#[aoc(day13, part2)]
pub fn part_2((grid, folds): &(Grid, Vec<Fold>)) -> Result<String, Error> {
    let final_grid: Grid = folds
        .iter()
        .fold(grid.to_vec(), |acc, fold| fold.perform_on(&acc));

    // Need this weird vec! to pad the output with a leading newline
    // otherwise the output can't be read
    let mut result = vec!["\n".to_string()];
    result.extend(final_grid.into_iter().map(|row| {
        String::from_iter(row.into_iter().map(|is_dot| if is_dot { '#' } else { '.' }))
    }));

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), Ok(17));
    }
}
