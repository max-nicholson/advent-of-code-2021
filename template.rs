const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("part_1: {}", part_1(INPUT));
    println!("part_2: {}", part_2(INPUT));
}

pub fn part_1(input: &str) -> usize {
    todo!();
}

pub fn part_2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "x";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 1);
    }
}
