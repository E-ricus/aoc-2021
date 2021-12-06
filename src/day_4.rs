use anyhow::Result;

use crate::runner::{Parse, RunMut};

pub struct Day4 {}

impl Parse<Bingo> for Day4 {
    fn parse_input(input: &str) -> Result<Bingo> {
        let mut numbers: Vec<u16> = Vec::new();
        let mut boards: Vec<Board> = Vec::new();
        let mut is_first = true;
        let mut board = Board::new();
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

        for line in lines {
            if is_first {
                numbers = line.split(',').filter_map(|n| n.parse().ok()).collect();
                is_first = false;
                continue;
            }

            if line.is_empty() {
                if !board.numbers.is_empty() {
                    boards.push(board);
                    board = Board::new();
                }
                continue;
            }
            board.numbers_from_str(line.as_str());
        }
        boards.push(board);
        Ok(Bingo { boards, numbers })
    }
}

impl RunMut<Bingo, u32> for Day4 {
    fn part_one(input: &mut Bingo) -> Result<u32> {
        let (winner, win_num) =
            bingo_winner(&input.numbers, &mut input.boards).ok_or(anyhow::anyhow!("no winner"))?;
        Ok(get_result(winner, win_num))
    }

    fn part_two(input: &mut Bingo) -> Result<u32> {
        let (winner, win_num) = bingo_last_winner(&input.numbers, &mut input.boards)
            .ok_or(anyhow::anyhow!("no winner"))?;
        Ok(get_result(winner, win_num))
    }
}

#[derive(Debug, Clone)]
pub struct Bingo {
    boards: Vec<Board>,
    numbers: Vec<u16>,
}

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<Number>>,
    won: bool,
}

impl Board {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
            won: false,
        }
    }
    fn numbers_from_str(&mut self, row: &str) {
        let numbers = row
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .map(Number::new)
            .collect();
        self.numbers.push(numbers);
    }
    fn set_number(&mut self, num: u16) {
        for row in self.numbers.iter_mut() {
            for number in row {
                if number.num == num {
                    number.state = State::Marked;
                    return;
                }
            }
        }
    }
    fn is_winner(&self) -> bool {
        for row in self.numbers.iter() {
            let mut count = 0;
            for number in row {
                if matches!(number.state, State::Marked) {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }
        for i in 0..5 {
            let mut count = 0;
            for row in self.numbers.iter() {
                if matches!(row[i].state, State::Marked) {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Number {
    num: u16,
    state: State,
}

impl Number {
    fn new(num: u16) -> Self {
        Self {
            num,
            state: State::Unmarked,
        }
    }

    fn is_marked(&self) -> bool {
        matches!(self.state, State::Marked)
    }
}

#[derive(Debug, Clone)]
enum State {
    Marked,
    Unmarked,
}

fn bingo_winner(numbers: &[u16], boards: &mut Vec<Board>) -> Option<(Board, u16)> {
    for num in numbers {
        for b in boards.iter_mut() {
            b.set_number(*num);
            if b.is_winner() {
                return Some((b.clone(), *num));
            }
        }
    }
    None
}

fn bingo_last_winner(numbers: &[u16], boards: &mut Vec<Board>) -> Option<(Board, u16)> {
    let board_num = boards.len();
    let mut wins = 0;
    // let mut winners: Vec<usize> = Vec::new();
    for num in numbers {
        for b in boards.iter_mut() {
            b.set_number(*num);
            if b.is_winner() && !b.won {
                wins += 1;
                b.won = true;
            }
            if board_num == wins {
                return Some((b.clone(), *num));
            }
        }
    }
    None
}

fn get_result(winner: Board, win_num: u16) -> u32 {
    let unmarked: u16 = winner
        .numbers
        .into_iter()
        .flatten()
        .filter(|n| !n.is_marked())
        .map(|n| n.num)
        .sum();
    (win_num * unmarked).into()
}

#[cfg(test)]
mod tests_day4 {
    use super::*;
    use crate::runner::MutExecutor;

    #[test]
    fn test_is_winner_row() {
        let numbers = vec![
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            "2  0 12  3  7",
        ];
        let n = vec![14, 21, 17, 24, 4];
        let mut board = Board::new();
        numbers.into_iter().for_each(|s| board.numbers_from_str(s));
        n.into_iter().for_each(|n| board.set_number(n));
        assert!(board.is_winner());
    }

    #[test]
    fn test_is_winner_column() {
        let numbers = vec![
            "14 10 7 22  2",
            "21 16 15  9 19",
            "17  8 23 26 20",
            "24 11 13  6  5",
            "4  0 12  3  7",
        ];
        let n = vec![14, 21, 17, 24, 4];
        let mut board = Board::new();
        numbers.into_iter().for_each(|s| board.numbers_from_str(s));
        n.into_iter().for_each(|n| board.set_number(n));
        assert!(board.is_winner());
    }

    #[test]
    fn test_is_not_winner() {
        let numbers = vec![
            "14 10 7 22  2",
            "21 16 15  9 19",
            "17  8 23 26 20",
            "24 11 13  6  5",
            "4  0 12  3  7",
        ];
        let n = vec![14, 21, 17, 24, 25];
        let mut board = Board::new();
        numbers.into_iter().for_each(|s| board.numbers_from_str(s));
        n.into_iter().for_each(|n| board.set_number(n));
        assert!(!board.is_winner());
    }

    const INPUT: &str = include_str!("../inputs/day4.test");

    #[test]
    fn test_bingo_winner() -> Result<()> {
        let mut input = Day4::parse_input(INPUT)?;
        let winner = bingo_winner(&input.numbers, &mut input.boards);
        assert!(winner.is_some());
        let (winner, num) = winner.unwrap();
        assert_eq!(winner.numbers[0][0].num, 14);
        assert_eq!(num, 24);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let mut input = Day4::parse_input(INPUT)?;
        let result = Day4::part_one(&mut input)?;
        assert_eq!(result, 4512);
        Ok(())
    }

    #[test]
    fn test_bingo_last_winner() -> Result<()> {
        let mut input = Day4::parse_input(INPUT)?;
        let winner = bingo_last_winner(&input.numbers, &mut input.boards);
        assert!(winner.is_some());
        let (winner, num) = winner.unwrap();
        assert_eq!(winner.numbers[0][0].num, 3);
        assert_eq!(num, 13);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = Day4::parse_input(INPUT)?;
        let result = Day4::part_two(&mut input)?;
        assert_eq!(result, 1924);
        Ok(())
    }

    #[test]
    fn test_run_day_four() -> Result<()> {
        let (r1, r2) = Day4::run("inputs/day4.test")?;
        assert_eq!(r1, 4512);
        assert_eq!(r2, 1924);
        Ok(())
    }
}
