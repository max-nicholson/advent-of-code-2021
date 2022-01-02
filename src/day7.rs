use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

#[aoc_generator(day7)]
fn parse_positions(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| str::parse(x).unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part_1(positions: &[usize]) -> usize {
    let median = median(&mut positions.to_vec());

    let deltas = positions
        .iter()
        .map(|x| (*x as i32 - median as i32).abs() as usize);

    deltas.sum()
}

#[aoc(day7, part2)]
pub fn part_2(positions: &[usize]) -> usize {
    let mut counters = HashMap::new();
    positions.iter().for_each(|&position| {
        let counter = counters.entry(position).or_insert(0);
        *counter += 1;
    });

    let min = *counters.keys().min().unwrap();
    let max = *counters.keys().max().unwrap();

    let mut least_fuel: usize = usize::MAX;

    for position in min..=max {
        let fuel: usize = counters
            .iter()
            .map(|(k, count)| {
                let fuel_for_position = match *k == position {
                    true => 0,
                    false => triangular_number_sum(abs_diff(k, &position)),
                };
                fuel_for_position * count
            })
            .sum();

        if fuel < least_fuel {
            least_fuel = fuel
        }
    }

    least_fuel
}

fn average(numbers: &[usize]) -> f32 {
    numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [usize]) -> usize {
    numbers.sort_unstable();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        average(&[numbers[mid - 1], numbers[mid]]) as usize
    } else {
        numbers[mid]
    }
}

fn abs_diff(a: &usize, b: &usize) -> usize {
    (*a as i32 - *b as i32).abs() as usize
}

#[cached]
pub fn triangular_number_sum(num: usize) -> usize {
    (0..num + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_positions(EXAMPLE)), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_positions(EXAMPLE)), 168);
    }
}
