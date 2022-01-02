use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_runner_derive::aoc;

type Signals<'a> = Vec<&'a str>;

type Output<'a> = Vec<&'a str>;

type Entry<'a> = (Signals<'a>, Output<'a>);

fn find_extra_char(a: &str, b: &str) -> char {
    // get which one is shorter
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };

    // fill the set with the characters from the shorter string
    let set: HashSet<char> = shorter.chars().collect();

    for c in longer.chars() {
        if !set.contains(&c) {
            return c;
        }
    }
    panic!("No extra character")
}

fn decode_segments_map<'a>(signals: &'a Signals) -> HashMap<&'a str, usize> {
    // 2: 1
    // 3: 7
    // 4: 4
    // 5: 2, 3, 5
    // 6: 0, 6, 9
    // 7: 8

    // 7 is the same as 1, but also with 'a' => 'a'
    // 2 is the only digit that doesn't use 'f' => 'f' and 2
    // From 1 and 'f' we can then determine 'c' => 'c'
    // 6 is the only 6 segment digit without 'c' => 6
    // 5 is the only 5 segment digit without 'c' => 5
    // 3 is the remaining 5 segment digit => 3
    // Difference between 5 and 6 => 'e'
    // Only 6 segment with missing 'e' => 9
    // Remaining number => 0
    let mut digits: HashMap<&str, usize> = HashMap::new();
    for &signal in signals {
        if let Some(digit) = decode_unique_digits(signal) {
            digits.insert(signal, digit);
        }
    }
    let one: &str = digits.iter().find(|(k, &v)| v == 1usize).unwrap().0;

    let character_count: HashMap<char, usize> =
        signals.iter().fold(HashMap::new(), |mut map, signal| {
            signal.chars().for_each(|c| {
                let counter = map.entry(c).or_insert(0);
                *counter += 1;
            });
            map
        });

    let &f = character_count
        .iter()
        .find_map(|(k, v)| match v {
            9 => Some(k),
            _ => None,
        })
        .unwrap();

    let two: &str = signals
        .iter()
        .find_map(|signal| {
            if !signal.contains(f) {
                Some(signal)
            } else {
                None
            }
        })
        .unwrap();

    let c: char = one
        .chars()
        .find_map(|ch| if ch != f { Some(ch) } else { None })
        .unwrap();

    let six: &str = signals
        .iter()
        .find(|signal| signal.len() == 6 && !signal.contains(c))
        .unwrap();

    let five: &str = signals
        .iter()
        .find(|signal| signal.len() == 5 && !signal.contains(c))
        .unwrap();

    let three: &str = signals
        .iter()
        .find(|&signal| signal.len() == 5 && signal != &two && signal != &five)
        .unwrap();

    let e = find_extra_char(six, five);

    let nine: &str = signals
        .iter()
        .find(|&signal| signal.len() == 6 && !signal.contains(e))
        .unwrap();

    let zero: &str = signals
        .iter()
        .find(|&signal| signal.len() == 6 && signal != &six && signal != &nine)
        .unwrap();

    digits.insert(two, 2);
    digits.insert(three, 3);
    digits.insert(five, 5);
    digits.insert(zero, 0);
    digits.insert(six, 6);
    digits.insert(nine, 9);

    digits
}

fn decode_unique_digits(signal: &str) -> Option<usize> {
    match signal.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        // TODO: other segments
        _ => None,
    }
}

// NB: Can't use aoc(generator) as it will complain about lifetimes
fn parse_input<'a>(input: &'a str) -> Vec<Entry<'a>> {
    input
        .lines()
        .map(|l| {
            let mut contents = l.split(" | ");
            let signals: Signals = contents.next().unwrap().split(' ').collect();
            let output: Output = contents.next().unwrap().split(' ').collect();
            (signals, output)
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part_1(input: &str) -> usize {
    let entries = parse_input(input);
    entries
        .iter()
        .flat_map(|(_signals, output)| output)
        .filter(|signal| match decode_unique_digits(signal) {
            Some(x) => matches!(x, 1 | 4 | 7 | 8),
            None => false,
        })
        .count()
}

#[aoc(day8, part2)]
pub fn part_2(input: &str) -> usize {
    let entries = parse_input(input);
    entries
        .iter()
        .map(|(signals, output)| {
            let map = decode_segments_map(signals);
            let character_set_map: HashMap<_, _> = map
                .iter()
                // Must use BTreeSet as HashSet doesn't implement Hash trait
                .map(|(&k, &v)| (BTreeSet::from_iter(k.chars()), v))
                .collect();
            output
                .iter()
                .map(|&digit| {
                    let chars = BTreeSet::from_iter(digit.chars());
                    character_set_map.get(&chars).unwrap()
                })
                .fold(0, |acc, n| acc * 10 + n)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 61229);
    }
}
