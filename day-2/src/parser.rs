use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::tuple,
    IResult,
};

use crate::{Command, Direction};

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
