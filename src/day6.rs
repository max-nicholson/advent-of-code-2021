use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

#[aoc_generator(day6)]
fn parse_ages(input: &str) -> Vec<usize> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| usize::from_str_radix(x, 10).unwrap())
        .collect()
}

#[cached]
fn fish_after(timer: usize, days: usize) -> usize {
    if days <= timer {
        return 1;
    }

    let days = days - 1;

    if timer == 0 {
        return fish_after(6, days) + fish_after(8, days);
    }

    fish_after(timer - 1, days)
}

#[aoc(day6, part1)]
pub fn part_1(ages: &[usize]) -> usize {
    ages.iter().map(|&fish| fish_after(fish, 80)).sum()
}

#[aoc(day6, part2)]
pub fn part_2(ages: &[usize]) -> usize {
    ages.iter().map(|&fish| fish_after(fish, 256)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_fish_after() {
        assert_eq!(fish_after(1, 1), 1);
        assert_eq!(fish_after(1, 2), 2);
        assert_eq!(fish_after(1, 9), 3);
        assert_eq!(fish_after(1, 11), 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_ages(EXAMPLE)), 5934);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_ages(EXAMPLE)), 26984457539);
    }
}
