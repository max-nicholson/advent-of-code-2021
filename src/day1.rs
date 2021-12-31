use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn read_depths(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn a(depths: &Vec<usize>) -> usize {
    depths.iter().zip(depths.iter().skip(1)).fold(
        0,
        |acc, (prev, curr)| {
            if curr > prev {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[aoc(day1, part2)]
fn part2(depths: &Vec<usize>) -> usize {
    let windows: Vec<usize> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    windows
        .iter()
        .zip(windows.iter().skip(1))
        .fold(0, |acc, (prev, curr)| acc + (curr > prev) as usize)
}
