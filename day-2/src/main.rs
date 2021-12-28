mod parser;

use parser::parse_command;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(PartialEq, Debug)]
pub struct Command {
    direction: Direction,
    units: u32,
}

fn main() {
    let input = include_str!("../input.txt");
    println!("a: {}", a(input));
    println!("b: {}", b(input))
}

fn a(input: &str) -> u32 {
    let commands = read_commands(input);
    let (horizontal, depth) = commands.fold((0, 0), |(x, y), Command { direction, units }| {
        match direction {
            Direction::Forward => (x + units, y),
            // Convert y-position to depth (inverse)
            Direction::Up => (x, y - units),
            Direction::Down => (x, y + units),
        }
    });
    horizontal * depth
}

fn b(input: &str) -> u32 {
    let commands = read_commands(input);
    let (horizontal, depth, _aim) = commands.fold(
        (0, 0, 0),
        |(x, y, z), Command { direction, units }| match direction {
            Direction::Down => (x, y, z + units),
            Direction::Up => (x, y, z - units),
            Direction::Forward => (x + units, y + z * units, z),
        },
    );
    horizontal * depth
}

fn read_commands(input: &str) -> impl Iterator<Item = Command> + '_ {
    input
        .lines()
        .map(parse_command)
        .filter_map(|l| l.ok())
        .map(|res| res.1)
}

#[cfg(test)]
mod tests {
    use super::{a, b};

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_a() {
        assert_eq!(a(EXAMPLE), 150)
    }

    #[test]
    fn test_b() {
        assert_eq!(b(EXAMPLE), 900)
    }
}
