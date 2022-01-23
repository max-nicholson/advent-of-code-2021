use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::str;

use aoc_runner_derive::aoc;

pub fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = input.lines();

    let template = lines.next().expect("Polymer template").chars().collect();

    lines.next();

    let rules: HashMap<(char, char), char> = lines
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            (
                (pair.next().unwrap(), pair.next().unwrap()),
                element.chars().next().unwrap(),
            )
        })
        .collect();

    (template, rules)
}

fn step(
    current_pairs: HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut pairs = HashMap::new();

    current_pairs.into_iter().for_each(|(pair, count)| {
        let (a, b) = pair;
        match rules.get(&pair) {
            Some(&element) => {
                *pairs.entry((a, element)).or_insert(0) += count;
                *pairs.entry((element, b)).or_insert(0) += count;
            }
            None => unimplemented!(),
        }
    });

    pairs
}

fn evaluate(pairs: HashMap<(char, char), usize>, last_element: char) -> u128 {
    let mut counters: HashMap<char, u128> = HashMap::new();

    // If we only use the first item in the pair for each pair, we avoid double counting
    // However, we will need to increment the last element in the (original) template,
    // because we will undercount by 1 (as it won't be the "first" element in any pair)

    pairs
        .iter()
        .for_each(|((a, _b), &count)| *counters.entry(*a).or_insert(0) += count as u128);

    *counters.entry(last_element).or_insert(0) += 1;

    match counters.values().minmax() {
        MinMaxResult::MinMax(&min, &max) => max - min,
        _ => unreachable!(),
    }
}

fn load_initial_pairs(template: Vec<char>) -> HashMap<(char, char), usize> {
    let mut current_pairs: HashMap<(char, char), usize> = HashMap::new();

    template.into_iter().tuple_windows().for_each(|(a, b)| {
        *current_pairs.entry((a, b)).or_insert(0) += 1;
    });

    current_pairs
}

#[aoc(day14, part1)]
pub fn part_1(input: &str) -> u128 {
    let (template, rules) = parse_input(input);

    let last_element = template[template.len() - 1];

    let final_pairs = (0..10).fold(load_initial_pairs(template), |acc, _step| step(acc, &rules));

    evaluate(final_pairs, last_element)
}

#[aoc(day14, part2)]
pub fn part_2(input: &str) -> u128 {
    let (template, rules) = parse_input(input);

    let last_element = template[template.len() - 1];

    let final_pairs = (0..40).fold(load_initial_pairs(template), |acc, _step| step(acc, &rules));

    evaluate(final_pairs, last_element)
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
        assert_eq!(part_2(EXAMPLE), 2188189693529);
    }
}
