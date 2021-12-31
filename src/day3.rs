use aoc_runner_derive::aoc;
use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref ONE: char = char::from_digit(1, 2).unwrap();
}

#[derive(Debug)]
pub struct WeightedTrie {
    pub height: usize,
    pub tree: Vec<usize>,
}

impl WeightedTrie {
    pub fn new(height: u32) -> Self {
        if height <= 1 {
            panic!("Must be positive height")
        }

        Self {
            height: height as usize,
            tree: vec![0; 2usize.pow(height + 1) - 1],
        }
    }

    pub fn insert(&mut self, input: &str) {
        if input.len() > self.height {
            panic!(
                "Trie was initialized to {} height, requires {} height",
                self.height,
                input.len()
            )
        }

        self.tree[0] += 1;
        let mut index = 0;
        for digit in input.chars() {
            index = match digit {
                '0' => Self::left_from(index),
                '1' => Self::right_from(index),
                _ => unimplemented!("Only binary characters expected"),
            };
            self.tree[index] += 1;
        }
    }

    pub fn left_from(i: usize) -> usize {
        2 * i + 1
    }

    pub fn right_from(i: usize) -> usize {
        2 * i + 2
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let numbers: Vec<&str> = input.lines().collect();

    let columns = numbers.first().unwrap_or(&"").len();

    let mut totals: HashMap<usize, usize> =
        (0..columns).map(|i| (i, 0usize)).collect::<HashMap<_, _>>();

    for number in &numbers {
        for (position, digit) in number.chars().enumerate() {
            if digit == *ONE {
                let counter = totals.entry(position).or_insert(0);
                *counter += 1;
            }
        }
    }

    let lines = numbers.len();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for column in 0..columns {
        let total = totals.get(&column).unwrap();
        if total > &(lines / 2) {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();
    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let numbers: Vec<&str> = input.lines().collect();
    let columns = numbers.first().unwrap_or(&"").len() as u32;

    let mut trie = WeightedTrie::new(columns);
    for number in &numbers {
        trie.insert(number);
    }

    let mut oxygen_generator_rating = String::new();
    let mut parent = 0usize;
    for _ in 0..trie.height {
        let left = WeightedTrie::left_from(parent);
        let right = WeightedTrie::right_from(parent);
        let delta: i32 = trie.tree[left] as i32 - trie.tree[right] as i32;
        parent = match delta {
            // Most common bit
            d if d > 0 => {
                oxygen_generator_rating.push('0');
                left
            }
            // Tie or right is greater
            _ => {
                oxygen_generator_rating.push('1');
                right
            }
        };
    }
    let oxygen_generator_rating = usize::from_str_radix(&oxygen_generator_rating, 2).unwrap();

    let mut co2_scrubber_rating = String::new();
    let mut parent = 0usize;
    for _ in 0..trie.height {
        let left = WeightedTrie::left_from(parent);
        let right = WeightedTrie::right_from(parent);
        let left_value = trie.tree[left];
        let right_value = trie.tree[right];

        // As CO2 takes the least common value, we need to add explicit corner cases for only one
        // value left in the trie
        if left_value == 0 && right_value == 1 {
            co2_scrubber_rating.push('1');
            parent = right
        } else if left_value == 1 && right_value == 0 {
            co2_scrubber_rating.push('0');
            parent = left
        } else {
            let delta: i32 = left_value as i32 - right_value as i32;
            parent = match delta {
                // Least common bit
                d if d > 0 => {
                    co2_scrubber_rating.push('1');
                    right
                }
                // Tie or right is greater
                _ => {
                    co2_scrubber_rating.push('0');
                    left
                }
            };
        }
    }
    let co2_scrubber_rating = usize::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_a() {
        assert_eq!(part1(EXAMPLE), 198);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(EXAMPLE), 230);
    }

    #[test]
    fn test_trie() {
        let trie = WeightedTrie::new(2);
        assert_eq!(trie.height, 2);
        assert_eq!(trie.tree, vec![0; 7]);
    }

    #[test]
    #[should_panic(expected = "Must be positive height")]
    fn test_zero_height_trie() {
        WeightedTrie::new(0);
    }

    #[test]
    fn test_trie_insert() {
        let mut trie = WeightedTrie::new(2);
        trie.insert("10");
        assert_eq!(trie.tree, vec![1, 0, 1, 0, 0, 1, 0]);
        trie.insert("11");
        assert_eq!(trie.tree, vec![2, 0, 2, 0, 0, 1, 1]);
    }
}
