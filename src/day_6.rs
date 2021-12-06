use crate::runner::{Parse, RunMut};

use anyhow::Result;

pub struct Day6 {}

impl Parse<Vec<u16>, usize> for Day6 {
    fn parse_input(input: &str) -> Result<Vec<u16>> {
        Ok(input
            .trim()
            .split(',')
            .into_iter()
            .filter_map(|n| n.parse().ok())
            .collect())
    }
}

impl RunMut<Vec<u16>, usize> for Day6 {
    fn part_one(input: &mut Vec<u16>) -> Result<usize> {
        change_fish(input, 80);
        Ok(input.len())
    }

    fn part_two(input: &mut Vec<u16>) -> Result<usize> {
        change_fish(input, 256);
        Ok(input.len())
    }
    // add code here
}

fn change_fish(input: &mut Vec<u16>, days: u16) {
    for _ in 0..days {
        let mut new_fishes = Vec::new();
        for fish in input.iter_mut() {
            match fish {
                0 => {
                    *fish = 6;
                    new_fishes.push(8);
                }
                _ => *fish -= 1,
            }
        }
        if !new_fishes.is_empty() {
            input.append(&mut new_fishes);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day6.test");

    #[test]
    fn test_part_one() -> Result<()> {
        let mut input = Day6::parse_input(INPUT)?;
        let result = Day6::part_one(&mut input)?;
        assert_eq!(result, 5934);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = Day6::parse_input(INPUT)?;
        let result = Day6::part_two(&mut input)?;
        assert_eq!(result, 26984457539);
        Ok(())
    }

    #[test]
    fn test_change_fish() -> Result<()> {
        let mut input = Day6::parse_input(INPUT)?;
        change_fish(&mut input, 3);
        assert_eq!(input.len(), 7);
        let mut input = Day6::parse_input(INPUT)?;
        change_fish(&mut input, 18);
        assert_eq!(input.len(), 26);
        Ok(())
    }
}
