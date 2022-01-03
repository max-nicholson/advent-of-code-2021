use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part_1(input: &str) -> usize {
    let count_of_illegal_chars = input.lines().filter_map(find_first_illegal_character).fold(
        HashMap::<char, usize>::new(),
        |mut counters, ch| {
            let count = counters.entry(ch).or_default();
            *count += 1;
            counters
        },
    );
    score_illegal_characters(count_of_illegal_chars)
}

fn build_character_pair_map() -> HashMap<char, char> {
    HashMap::from_iter([('}', '{'), (')', '('), ('>', '<'), (']', '[')])
}

fn find_first_illegal_character(line: &str) -> Option<char> {
    let pairs = build_character_pair_map();
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => stack.push(c),
            '}' | ')' | ']' | '>' => {
                let opening_symbol = *pairs.get(&c).unwrap();
                let paired_symbol = stack.pop();
                if paired_symbol != Some(opening_symbol) {
                    return Some(c);
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

fn score_illegal_characters(character_counts: HashMap<char, usize>) -> usize {
    let scores: HashMap<char, usize> =
        HashMap::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    character_counts
        .into_iter()
        .map(|(ch, count)| scores.get(&ch).unwrap() * count)
        .sum()
}

#[aoc(day10, part2)]
pub fn part_2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .map(complete_line)
        .filter(|completion_string| !completion_string.is_empty())
        .map(score_completion_string)
        .collect();
    scores.sort_unstable();
    scores[(scores.len() - 1) / 2]
}

fn complete_line(line: &str) -> Vec<char> {
    let pairs = build_character_pair_map();
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => stack.push(c),
            '}' | ')' | ']' | '>' => {
                let opening_symbol = *pairs.get(&c).unwrap();
                let paired_symbol = stack.pop();
                if paired_symbol != Some(opening_symbol) {
                    return Vec::new();
                }
            }
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        return Vec::new();
    }

    let reverse_pairs: HashMap<char, char> = pairs.into_iter().map(|(k, v)| (v, k)).collect();

    stack
        .into_iter()
        .rev()
        .map(|ch| *reverse_pairs.get(&ch).unwrap())
        .collect()
}

fn score_completion_string(characters: Vec<char>) -> usize {
    let character_score: HashMap<char, usize> =
        HashMap::from_iter([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    characters
        .iter()
        .fold(0, |acc, ch| acc * 5 + character_score.get(ch).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_find_first_illegal_character() {
        assert_eq!(
            find_first_illegal_character("[({(<(())[]>[[{[]{<()<>>"),
            None
        );
        assert_eq!(
            find_first_illegal_character("{([(<{}[<>[]}>{[]{[(<()>"),
            Some('}')
        );
        assert_eq!(
            find_first_illegal_character("[[<[([]))<([[{}[[()]]]"),
            Some(')')
        );
    }

    #[test]
    fn test_score_illegal_characters() {
        assert_eq!(
            score_illegal_characters(HashMap::from_iter([(')', 2), (']', 1)])),
            63
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 26397);
    }

    #[test]
    fn test_complete_line() {
        // Incomplete line
        assert_eq!(
            complete_line("[({(<(())[]>[[{[]{<()<>>"),
            ['}', '}', ']', ']', ')', '}', ')', ']']
        );
        // Corrupted line
        assert_eq!(complete_line("{([(<{}[<>[]}>{[]{[(<()>"), [],)
    }

    #[test]
    fn fn_score_completion_string() {
        assert_eq!(
            score_completion_string(vec!['}', '}', ']', ']', ')', '}', ')', ']']),
            288957
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 288957);
    }
}
