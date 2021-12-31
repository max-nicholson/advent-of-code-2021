use cached::proc_macro::cached;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("part_1: {}", part_1(INPUT));
    println!("part_2: {}", part_2(INPUT));
}

fn parse_ages(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| usize::from_str_radix(x, 10).unwrap())
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

pub fn part_1(input: &str) -> usize {
    let ages = parse_ages(input);

    ages.map(|fish| fish_after(fish, 80)).sum()
}

pub fn part_2(input: &str) -> usize {
    let ages = parse_ages(input);

    ages.map(|fish| fish_after(fish, 256)).sum()
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
        assert_eq!(part_1(EXAMPLE), 5934);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 26984457539);
    }
}
