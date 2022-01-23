use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::str;

use aoc_runner_derive::aoc;

pub fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, char>) {
    let mut lines = input.lines();

    let template = lines.next().expect("Polymer template").chars().collect();

    lines.next();

    let rules: HashMap<&str, char> = lines
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            (pair, element.chars().next().unwrap())
        })
        .collect();

    (template, rules)
}

fn step(polymer: &[char], rules: &HashMap<&str, char>) -> Vec<char> {
    let mut new_polymer = polymer.to_vec();
    let mut offset = 1;
    for i in 0..polymer.len() - 1 {
        let pair = [polymer[i] as u8, polymer[i + 1] as u8];
        let pair = str::from_utf8(&pair).expect("utf-8 characters");
        if let Some(&element) = rules.get(pair) {
            new_polymer.insert(i + offset, element);
            offset += 1;
        }
    }
    new_polymer
}

fn evaluate(polymer: Vec<char>) -> u32 {
    let mut counters: HashMap<char, u32> = HashMap::new();
    polymer
        .into_iter()
        .for_each(|c| *counters.entry(c).or_insert(0) += 1);

    match counters.values().minmax() {
        MinMaxResult::MinMax(&min, &max) => max - min,
        _ => unreachable!(),
    }
}

#[aoc(day14, part1)]
pub fn part_1(input: &str) -> u32 {
    let (template, rules) = parse_input(input);

    let polymer = (0..10).fold(template, |polymer, _| step(&polymer, &rules));

    evaluate(polymer)
}

#[aoc(day14, part2)]
pub fn part_2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 1588);
    }

    #[test]
    fn test_part_2() {
        todo!();
        assert_eq!(part_2(EXAMPLE), 2188189693529);
    }
}
