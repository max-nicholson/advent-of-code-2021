use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_diagonal(&self) -> bool {
        // Could use trigonometry but this is easier
        !(self.is_horizontal() | self.is_vertical())
    }

    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if self.is_diagonal() {
            let x_range = create_range(self.start.0, self.end.0);
            let y_range = create_range(self.start.1, self.end.1);

            for point in x_range.zip(y_range) {
                points.push(point)
            }

            return points;
        }

        let (start, end) = match self.is_vertical() {
            true => (self.start.1, self.end.1),
            false => (self.start.0, self.end.0),
        };

        let mut v = [start, end];
        v.sort();

        if self.is_vertical() {
            let x = self.start.0;
            for y in v[0]..=v[1] {
                points.push((x, y))
            }
        } else {
            let y = self.start.1;
            for x in v[0]..=v[1] {
                points.push((x, y))
            }
        }
        points
    }
}

type Lines = Vec<Line>;

fn create_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    // Can't simply use start.0..=end.0, as we may have a decreasing range
    // This turns into an empty Range (instead of a decrementing one)
    let (part1, part2) = if start <= end {
        (start..=end, 1..=0)
    } else {
        (1..=0, end..=start)
    };
    part1.chain(part2.rev())
}

fn main() {
    println!("part_1: {}", part_1(INPUT));
    println!("part_2: {}", part_2(INPUT));
}

fn to_usize(x: &str) -> usize {
    usize::from_str_radix(x, 10).unwrap()
}

fn parse_lines(input: &str) -> Lines {
    input
        .lines()
        .map(|line| {
            let points: Vec<&str> = line.split(" -> ").collect();
            let start: Vec<usize> = points[0].split(",").map(to_usize).collect();
            let end: Vec<usize> = points[1].split(",").map(to_usize).collect();
            Line {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
            }
        })
        .collect()
}

pub fn part_1(input: &str) -> usize {
    let lines = parse_lines(input);
    let mut counters: HashMap<Point, usize> = HashMap::new();

    for line in lines.iter().filter(|l| l.is_horizontal() | l.is_vertical()) {
        for point in line.points() {
            let count = counters.entry(point).or_insert(0);
            *count += 1;
        }
    }
    counters.values().filter(|&x| x >= &2).count()
}

pub fn part_2(input: &str) -> usize {
    let lines = parse_lines(input);
    let mut counters: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        for point in line.points() {
            let count = counters.entry(point).or_insert(0);
            *count += 1;
        }
    }
    counters.values().filter(|&x| x >= &2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_is_vertical() {
        assert!(Line {
            start: (1, 1),
            end: (1, 3)
        }
        .is_vertical());
        assert!(!Line {
            start: (9, 7),
            end: (7, 7)
        }
        .is_vertical())
    }

    #[test]
    fn test_is_horizontal() {
        assert!(!Line {
            start: (1, 1),
            end: (1, 3)
        }
        .is_horizontal());
        assert!(Line {
            start: (9, 7),
            end: (7, 7)
        }
        .is_horizontal())
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines("1,1 -> 1,3\n9,7 -> 7,7"),
            vec![
                Line {
                    start: (1, 1),
                    end: (1, 3)
                },
                Line {
                    start: (9, 7),
                    end: (7, 7)
                }
            ]
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 12);
    }
}
