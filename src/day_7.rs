use anyhow::Result;

use crate::runner::{Parse, RunMut};

pub struct Day7 {}

impl Parse<Vec<i32>> for Day7 {
    fn parse_input(input: &str) -> Result<Vec<i32>> {
        Ok(input
            .trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect())
    }
}

impl RunMut<Vec<i32>, i32> for Day7 {
    fn part_one(input: &mut Vec<i32>) -> Result<i32> {
        input.sort_unstable();
        let mid = input[input.len() / 2];
        let fuel = input.iter().fold(0, |acc, n| {
            let diff = (mid - n).abs();
            acc + diff
        });
        Ok(fuel)
    }

    fn part_two(input: &mut Vec<i32>) -> Result<i32> {
        input.sort_unstable();
        let mid = input[input.len() / 2];
        let left = mid - 1;
        let right = mid + 1;
        let mid_fuel = expensive_fuel(input, mid);
        let left_fuel = expensive_fuel(input, left);
        let right_fuel = expensive_fuel(input, right);
        if mid_fuel < right_fuel && mid_fuel < left_fuel {
            return Ok(mid_fuel);
        }
        let mut control: i32;
        let decrease: bool;
        let mut current_optimal: i32;
        if left_fuel < right_fuel {
            control = left;
            decrease = true;
            current_optimal = left_fuel;
        } else {
            control = right;
            decrease = false;
            current_optimal = right_fuel;
        }

        loop {
            match decrease {
                true => control -= 1,
                false => control += 1,
            }
            let possible_optimal = expensive_fuel(input, control);
            if possible_optimal >= current_optimal {
                return Ok(current_optimal);
            }
            current_optimal = possible_optimal;
        }
    }
}

fn expensive_fuel(input: &[i32], pos: i32) -> i32 {
    input.iter().fold(0, |acc, n| {
        let diff = (pos - n).abs();
        acc + (1..=diff).into_iter().sum::<i32>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day7.test");

    #[test]
    fn test_part_one() -> Result<()> {
        let mut input = Day7::parse_input(INPUT)?;
        let result = Day7::part_one(&mut input)?;
        assert_eq!(result, 37);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = Day7::parse_input(INPUT)?;
        let result = Day7::part_two(&mut input)?;
        assert_eq!(result, 168);
        Ok(())
    }

    #[test]
    fn test_expensive_fuel() -> Result<()> {
        let input = Day7::parse_input(INPUT)?;
        let fuel = expensive_fuel(&input, 5);
        assert_eq!(fuel, 168);
        let fuel = expensive_fuel(&input, 2);
        assert_eq!(fuel, 206);
        let fuel = expensive_fuel(&input, 4);
        assert_eq!(fuel, 170);
        Ok(())
    }
}
