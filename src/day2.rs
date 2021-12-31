use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::tuple,
    IResult,
};

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

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, (direction, _, units)) = tuple((parse_direction, tag(" "), parse_units))(input)?;
    Ok((input, Command { direction, units }))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("forward"), |_| Direction::Forward),
        map(tag("up"), |_| Direction::Up),
        map(tag("down"), |_| Direction::Down),
    ))(input)
}

fn parse_units(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

#[aoc_generator(day2)]
fn read_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(parse_command)
        .filter_map(|l| l.ok())
        .map(|res| res.1)
        .collect()
}

#[aoc(day2, part1)]
fn part1(commands: &[Command]) -> u32 {
    let (horizontal, depth) = commands.iter().fold((0, 0), |(x, y), Command { direction, units }| {
        match direction {
            Direction::Forward => (x + units, y),
            // Convert y-position to depth (inverse)
            Direction::Up => (x, y - units),
            Direction::Down => (x, y + units),
        }
    });
    horizontal * depth
}

#[aoc(day2, part2)]
fn part2(commands: &[Command]) -> u32 {
    let (horizontal, depth, _aim) = commands.iter().fold(
        (0, 0, 0),
        |(x, y, z), Command { direction, units }| match direction {
            Direction::Down => (x, y, z + units),
            Direction::Up => (x, y, z - units),
            Direction::Forward => (x + units, y + z * units, z),
        },
    );
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_a() {
        assert_eq!(part1(&read_commands(EXAMPLE)), 150)
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(&read_commands(EXAMPLE)), 900)
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("forward 5"),
            Ok((
                "",
                Command {
                    direction: Direction::Forward,
                    units: 5
                }
            ))
        )
    }

    #[test]
    fn test_parse_direction() {
        assert_eq!(parse_direction("up"), Ok(("", Direction::Up)));
    }

    #[test]
    fn test_parse_units() {
        assert_eq!(parse_units("5"), Ok(("", 5)));
        assert_eq!(parse_units("100"), Ok(("", 100)))
    }
}
