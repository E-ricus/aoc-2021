use anyhow::Result;

pub fn run_day_four(path: &str) -> Result<(u32, u32)> {
    let mut numbers: Vec<u16> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut is_first = true;
    let mut board = Board::new();
    let lines: Vec<String> = std::fs::read_to_string(path)?
        .lines()
        .map(|s| s.to_string())
        .collect();
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
    let result_one =
        part_one(numbers.clone(), boards.clone()).ok_or(anyhow::anyhow!("no winner"))?;
    println!("Day four part one: {}", result_one);
    let result_two = part_two(numbers, boards).ok_or(anyhow::anyhow!("no winner"))?;
    println!("Day four part two: {}", result_two);
    Ok((result_one, result_two))
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

fn bingo_winner(numbers: Vec<u16>, mut boards: Vec<Board>) -> Option<(Board, u16)> {
    for num in numbers {
        for b in boards.iter_mut() {
            b.set_number(num);
            if b.is_winner() {
                return Some((b.clone(), num));
            }
        }
    }
    None
}

fn bingo_last_winner(numbers: Vec<u16>, mut boards: Vec<Board>) -> Option<(Board, u16)> {
    let board_num = boards.len();
    let mut wins = 0;
    // let mut winners: Vec<usize> = Vec::new();
    for num in numbers {
        for b in boards.iter_mut() {
            b.set_number(num);
            if b.is_winner() && !b.won {
                wins += 1;
                b.won = true;
            }
            if board_num == wins {
                return Some((b.clone(), num));
            }
        }
    }
    None
}

fn get_result(winner: Board, win_num: u16) -> Option<u32> {
    let unmarked = winner.numbers.into_iter().fold(0, |acc, b| {
        let count: u16 = b
            .into_iter()
            .filter(|n| !n.is_marked())
            .map(|n| n.num)
            .sum();
        acc + count
    });
    Some((win_num * unmarked as u16).into())
}

fn part_one(numbers: Vec<u16>, boards: Vec<Board>) -> Option<u32> {
    let (winner, win_num) = bingo_winner(numbers, boards)?;
    get_result(winner, win_num)
}

fn part_two(numbers: Vec<u16>, boards: Vec<Board>) -> Option<u32> {
    let (winner, win_num) = bingo_last_winner(numbers, boards)?;
    get_result(winner, win_num)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_bingo_winner() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut boards: Vec<Board> = Vec::new();
        let board_string = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);

        let winner = bingo_winner(numbers, boards);
        assert!(winner.is_some());
        let (winner, num) = winner.unwrap();
        assert_eq!(winner.numbers[0][0].num, 14);
        assert_eq!(num, 24);
    }

    #[test]
    fn test_part_one() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut boards: Vec<Board> = Vec::new();
        let board_string = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let result = part_one(numbers, boards);
        assert!(result.is_some());
        assert_eq!(result, Some(4512));
    }

    #[test]
    fn test_bingo_last_winner() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut boards: Vec<Board> = Vec::new();
        let board_string = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);

        let winner = bingo_last_winner(numbers, boards);
        assert!(winner.is_some());
        let (winner, num) = winner.unwrap();
        assert_eq!(winner.numbers[0][0].num, 3);
        assert_eq!(num, 13);
    }

    #[test]
    fn test_part_two() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut boards: Vec<Board> = Vec::new();
        let board_string = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let board_string = vec![
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];

        let mut board = Board::new();
        board_string
            .into_iter()
            .for_each(|s| board.numbers_from_str(s));
        boards.push(board);
        let result = part_two(numbers, boards);
        assert!(result.is_some());
        assert_eq!(result, Some(1924));
    }

    #[test]
    fn test_run_day_four() -> Result<()> {
        let (result_one, result_two) = run_day_four("inputs/day4.test")?;
        assert_eq!(result_one, 4512);
        assert_eq!(result_two, 1924);
        Ok(())
    }
}
