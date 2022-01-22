use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

const START: &str = "start";
const END: &str = "end";

#[derive(Debug)]
pub struct Graph<'a> {
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

type VisitorCallback = fn(node: &str, visited: &HashMap<&str, u32>) -> bool;

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge<'b>(&'b mut self, from: &'a str, to: &'a str) {
        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    fn dfs(
        &self,
        current_edge: &'a str,
        visited: &mut HashMap<&'a str, u32>,
        path: &mut Vec<&'a str>,
        can_visit: VisitorCallback,
    ) -> usize {
        // Can't use visited.entry(current_edge).or_insert() as we borrow visited, then
        // can't reborrow during the `map` closure
        let count = visited.get(current_edge);
        let new_count = match count {
            Some(&c) => c + 1,
            None => 1,
        };
        visited.insert(current_edge, new_count);

        path.push(current_edge);

        let paths = if current_edge == END {
            1
        } else {
            self.edges
                .get(current_edge)
                .unwrap()
                .iter()
                .map(|node| {
                    if can_visit(node, visited) {
                        self.dfs(node, visited, path, can_visit)
                    } else {
                        0
                    }
                })
                .sum()
        };

        path.pop();
        visited.insert(current_edge, new_count - 1);
        paths
    }

    fn find_unique_paths(&self, visitor_callback: VisitorCallback) -> usize {
        self.dfs(
            START,
            &mut HashMap::new(),
            &mut Vec::new(),
            visitor_callback,
        )
    }
}

pub fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let mut tokens = line.split('-');
        let from = tokens.next().unwrap();
        let to = tokens.next().unwrap();
        graph.add_edge(from, to);
        graph.add_edge(to, from);
    }
    graph
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

#[aoc(day12, part1)]
pub fn part_1(input: &str) -> usize {
    let graph = parse_input(input);
    graph.find_unique_paths(|node, visited| {
        if !is_small_cave(node) {
            return true;
        }

        visited.get(node) != Some(&1)
    })
}

#[aoc(day12, part2)]
pub fn part_2(input: &str) -> usize {
    let graph = parse_input(input);
    graph.find_unique_paths(|node, visited| {
        if node == START {
            return visited.get(START) != Some(&1);
        }

        if is_small_cave(node) {
            return match visited.get(node) {
                Some(0) | None => true,
                Some(1) => !visited.iter().any(|(k, v)| is_small_cave(k) && v == &2),
                Some(_) => false,
            };
        }

        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const OTHER_EXAMPLE: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 10);
        assert_eq!(part_1(OTHER_EXAMPLE), 19);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 36);
        assert_eq!(part_2(OTHER_EXAMPLE), 103);
    }
}
