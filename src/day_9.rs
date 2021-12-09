use anyhow::Result;

use crate::runner::{Parse, Run};

pub struct Day9 {}

impl Parse<Vec<Vec<u32>>> for Day9 {
    fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
        let r = input
            .lines()
            .map(|s| {
                s.chars()
                    .into_iter()
                    .map(|c| c.to_digit(10))
                    .flatten()
                    .collect()
            })
            .collect();
        Ok(r)
    }
}

impl Run<Vec<Vec<u32>>, u32> for Day9 {
    fn part_one(input: &Vec<Vec<u32>>) -> Result<u32> {
        let sum = input
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(j, n)| is_lowest_adjacent(*n, i, *j, input))
                    .map(|(_, n)| n + 1)
                    .sum::<u32>()
            })
            .sum();
        Ok(sum)
    }

    fn part_two(_input: &Vec<Vec<u32>>) -> Result<u32> {
        Ok(0)
    }
}

fn is_lowest_adjacent(value: &u32, i: usize, j: usize, map: &[Vec<u32>]) -> bool {
    let i_len = map.len() - 1;
    let j_len = map[i].len() - 1;

    // TODO: Simplify coordinates
    let adjacents = match (i, j) {
        (0, 0) => {
            vec![map[i + 1][j], map[i][j + 1]]
        }
        (0, j) if j == j_len => {
            vec![map[i + 1][j], map[i][j - 1]]
        }
        (i, 0) if i == i_len => {
            vec![map[i - 1][j], map[i][j + 1]]
        }
        (i, j) if i == i_len && j == j_len => {
            vec![map[i - 1][j], map[i][j - 1]]
        }
        (0, j) => {
            vec![map[i + 1][j], map[i][j - 1], map[i][j + 1]]
        }
        (i, 0) => {
            vec![map[i - 1][j], map[i + 1][j], map[i][j + 1]]
        }
        (i, j) if i == i_len => {
            vec![map[i - 1][j], map[i][j - 1], map[i][j + 1]]
        }
        (i, j) if j == j_len => {
            vec![map[i - 1][j], map[i + 1][j], map[i][j - 1]]
        }
        _ => vec![map[i - 1][j], map[i + 1][j], map[i][j - 1], map[i][j + 1]],
    };
    adjacents.into_iter().filter(|a| a <= value).count() == 0
}

#[cfg(test)]
mod tests_day9 {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day9.test");

    #[test]
    fn test_part_one() -> Result<()> {
        let input = Day9::parse_input(INPUT)?;
        let count = Day9::part_one(&input)?;
        assert_eq!(count, 15);
        Ok(())
    }

    #[test]
    fn test_is_lowest_adjacent() -> Result<()> {
        let map = Day9::parse_input(INPUT)?;
        assert!(!is_lowest_adjacent(&2, 0, 0, &map));
        assert!(is_lowest_adjacent(&0, 0, 9, &map));
        assert!(!is_lowest_adjacent(&9, 4, 0, &map));
        assert!(!is_lowest_adjacent(&8, 4, 9, &map));
        assert!(is_lowest_adjacent(&1, 0, 1, &map));
        assert!(is_lowest_adjacent(&5, 4, 6, &map));
        assert!(!is_lowest_adjacent(&8, 3, 0, &map));
        assert!(!is_lowest_adjacent(&2, 2, 9, &map));
        assert!(is_lowest_adjacent(&5, 2, 2, &map));
        Ok(())
    }
}
