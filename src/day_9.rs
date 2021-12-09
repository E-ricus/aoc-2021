use std::collections::{HashMap, HashSet};

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

    fn part_two(input: &Vec<Vec<u32>>) -> Result<u32> {
        let mut basins: Vec<u32> = input
            .iter()
            .enumerate()
            .map(move |(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(move |(j, n)| is_lowest_adjacent(*n, i, *j, input))
                    .map(move |(j, _)| basin_size(i, j, input))
            })
            .flatten()
            .collect();
        basins.sort_unstable();
        let r: u32 = basins.into_iter().rev().take(3).product();

        Ok(r)
    }
}

fn basin_size(i: usize, j: usize, map: &[Vec<u32>]) -> u32 {
    let mut visited = HashSet::from([(i, j)]);
    let adjacents = create_adjacents(i, j, map);
    adjacents
        .into_iter()
        .filter(|(_, a)| *a < 9)
        .fold(1, |acc, (k, _)| {
            acc + count_neightbors(k.0, k.1, map, &mut visited)
        })
}

fn count_neightbors(
    i: usize,
    j: usize,
    map: &[Vec<u32>],
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    if !visited.insert((i, j)) {
        return 0;
    }
    let adjacents = create_adjacents(i, j, map);
    adjacents
        .into_iter()
        .filter(|(_, a)| *a < 9)
        .fold(1, |acc, (k, _)| {
            acc + count_neightbors(k.0, k.1, map, visited)
        })
}

fn is_lowest_adjacent(value: &u32, i: usize, j: usize, map: &[Vec<u32>]) -> bool {
    let adjacents = create_adjacents(i, j, map);
    adjacents.values().filter(|a| a <= &value).count() == 0
}

// TODO: Simplify coordinates
fn create_adjacents(i: usize, j: usize, map: &[Vec<u32>]) -> HashMap<(usize, usize), u32> {
    let i_len = map.len() - 1;
    let j_len = map[i].len() - 1;
    match (i, j) {
        (0, 0) => HashMap::from([((i + 1, j), map[i + 1][j]), ((i, j + 1), map[i][j + 1])]),
        (0, j) if j == j_len => {
            HashMap::from([((i + 1, j), map[i + 1][j]), ((i, j - 1), map[i][j - 1])])
        }
        (i, 0) if i == i_len => {
            HashMap::from([((i - 1, j), map[i - 1][j]), ((i, j + 1), map[i][j + 1])])
        }
        (i, j) if i == i_len && j == j_len => {
            HashMap::from([((i - 1, j), map[i - 1][j]), ((i, j - 1), map[i][j - 1])])
        }
        (0, j) => HashMap::from([
            ((i + 1, j), map[i + 1][j]),
            ((i, j - 1), map[i][j - 1]),
            ((i, j + 1), map[i][j + 1]),
        ]),
        (i, 0) => HashMap::from([
            ((i - 1, j), map[i - 1][j]),
            ((i + 1, j), map[i + 1][j]),
            ((i, j + 1), map[i][j + 1]),
        ]),
        (i, j) if i == i_len => HashMap::from([
            ((i - 1, j), map[i - 1][j]),
            ((i, j - 1), map[i][j - 1]),
            ((i, j + 1), map[i][j + 1]),
        ]),
        (i, j) if j == j_len => HashMap::from([
            ((i - 1, j), map[i - 1][j]),
            ((i + 1, j), map[i + 1][j]),
            ((i, j - 1), map[i][j - 1]),
        ]),
        _ => HashMap::from([
            ((i - 1, j), map[i - 1][j]),
            ((i + 1, j), map[i + 1][j]),
            ((i, j - 1), map[i][j - 1]),
            ((i, j + 1), map[i][j + 1]),
        ]),
    }
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
    fn test_part_two() -> Result<()> {
        let input = Day9::parse_input(INPUT)?;
        let count = Day9::part_two(&input)?;
        assert_eq!(count, 1134);
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

    #[test]
    fn test_vasin_size() -> Result<()> {
        let map = Day9::parse_input(INPUT)?;
        let size = basin_size(0, 1, &map);
        assert_eq!(size, 3);
        let size = basin_size(0, 9, &map);
        assert_eq!(size, 9);
        let size = basin_size(2, 2, &map);
        assert_eq!(size, 14);
        let size = basin_size(4, 6, &map);
        assert_eq!(size, 9);

        Ok(())
    }
}
