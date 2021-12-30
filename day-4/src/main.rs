use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

type Draw = Vec<usize>;

type Board = Vec<Vec<usize>>;

type Boards = Vec<Board>;

fn main() {
    println!("part_1: {}", part_1(INPUT));
    println!("part_2: {}", part_2(INPUT));
}

fn parse_draw(input: &str) -> Draw {
    input
        .split(",")
        .map(|n| str::parse::<usize>(n).unwrap())
        .collect()
}

fn parse_board<'a>(input: impl Iterator<Item = &'a str>) -> Board {
    let mut board: Board = Vec::new();
    for line in input {
        // line = "10  0 12 27 5"
        board.push(
            line.split_whitespace()
                .map(|x| str::parse::<usize>(x).unwrap())
                .collect(),
        )
    }
    board
}

fn parse_input(input: &str) -> (Draw, Boards) {
    let lines: Vec<&str> = input.lines().collect();
    let draw = parse_draw(lines[0]);

    let mut boards = Vec::new();

    for row in (2..lines.len()).step_by(6) {
        boards.push(parse_board(lines[row..row + 5].iter().cloned()))
    }

    (draw, boards)
}

fn score(draw: HashSet<usize>, board: &Board, last_number: usize) -> usize {
    let mut sum = 0;
    for row in board {
        for number in row {
            if !draw.contains(number) {
                sum += number
            }
        }
    }

    sum * last_number
}

fn has_won(board: &Board, drawn: &HashSet<usize>) -> bool {
    // check rows
    for row in board {
        if row.iter().all(|c| drawn.contains(c)) {
            return true;
        }
    }
    // check columns
    for column_idx in 0..5 {
        if (0..5)
            .map(|row| board[row][column_idx])
            .all(|n| drawn.contains(&n))
        {
            return true;
        }
    }
    false
}

fn find_first_winning_board<'a>(
    draw: &Draw,
    boards: &'a Boards,
) -> (&'a Board, HashSet<usize>, usize) {
    // Skip first 4 draws as we can't possibly have a win until draw 5
    let mut drawn = HashSet::<usize>::from_iter(draw.into_iter().take(4).cloned());

    for number in draw.iter().skip(4) {
        drawn.insert(*number);
        for board in boards {
            if has_won(board, &drawn) {
                return (board, drawn, number.clone());
            }
        }
    }
    panic!("No winning board");
}

fn find_last_winning_board<'a>(
    draw: &Draw,
    boards: &'a Boards,
) -> (&'a Board, HashSet<usize>, usize) {
    // Skip first 4 draws as we can't possibly have a win until draw 5
    let mut drawn = HashSet::<usize>::from_iter(draw.into_iter().take(4).cloned());
    let mut won_boards: HashSet<usize> = HashSet::new();

    for number in draw.iter().skip(4) {
        drawn.insert(*number);
        for (board_number, board) in boards.iter().enumerate() {
            if won_boards.contains(&board_number) {
                continue;
            }

            if has_won(board, &drawn) {
                if won_boards.len() == boards.len() - 1 {
                    // This is the last board
                    return (board, drawn, number.clone());
                }
                won_boards.insert(board_number);
            }
        }
    }
    panic!("No last winning board found");
}

pub fn part_1(input: &str) -> usize {
    let (draw, boards) = parse_input(input);

    let (winning_board, drawn, last_number_drawn) = find_first_winning_board(&draw, &boards);

    score(drawn, winning_board, last_number_drawn)
}

pub fn part_2(input: &str) -> usize {
    let (draw, boards) = parse_input(input);

    let (winning_board, drawn, last_number_drawn) = find_last_winning_board(&draw, &boards);

    score(drawn, winning_board, last_number_drawn)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_draw() {
        assert_eq!(parse_draw("7,4,9,5,11"), vec![7, 4, 9, 5, 11])
    }

    #[test]
    fn test_parse_board() {
        assert_eq!(
            parse_board(
                "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"
                    .lines()
            ),
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19]
            ]
        )
    }

    #[test]
    fn test_score() {
        let draw: Vec<usize> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        assert_eq!(
            score(
                HashSet::<usize>::from_iter(draw),
                &vec![
                    vec![14, 21, 17, 24, 4],
                    vec![10, 16, 15, 9, 19],
                    vec![18, 8, 23, 26, 20],
                    vec![22, 11, 13, 6, 5],
                    vec![2, 0, 12, 3, 7]
                ],
                24,
            ),
            4512
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 1924);
    }
}
