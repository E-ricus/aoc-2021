use anyhow::Result;

use crate::runner::{ParseWithLifeTime, Run};

pub struct Day8 {}

impl<'a> ParseWithLifeTime<'a, Vec<Metric<'a>>> for Day8 {
    fn parse_input(input: &'a str) -> Result<Vec<Metric<'a>>> {
        let r = input
            .lines()
            .filter_map(|l| l.split_once(" | "))
            .map(|(input, output)| Metric {
                input: input.split_whitespace().collect(),
                output: output.split_whitespace().collect(),
            })
            .collect();
        Ok(r)
    }
}

impl Run<Vec<Metric<'_>>, usize> for Day8 {
    fn part_one(input: &Vec<Metric<'_>>) -> Result<usize> {
        let basic_digits: Vec<usize> = vec![2, 3, 4, 7];
        let count = input.iter().fold(0, |mut acc, m| {
            let sum = m
                .output
                .iter()
                .filter(|o| basic_digits.contains(&o.len()))
                .count();
            acc += sum;
            acc
        });
        Ok(count)
    }

    fn part_two(input: &Vec<Metric<'_>>) -> Result<usize> {
        Ok(0)
    }
}

// TODO: Find if there is a way to do this on the executor
impl Day8 {
    pub fn run(path: &str) -> Result<(usize, usize)> {
        let input = std::fs::read_to_string(path)?;
        let input = Self::parse_input(input.as_str())?;
        let r1 = Self::part_one(&input)?;
        println!("{} part 1: {}", path, r1);
        let r2 = Self::part_two(&input)?;
        println!("{} part 2: {}", path, r2);
        Ok((r1, r2))
    }
}

#[derive(Debug)]
pub struct Metric<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day8.test");

    #[test]
    fn test_first_day() -> Result<()> {
        let input = Day8::parse_input(INPUT)?;
        let count = Day8::part_one(&input)?;
        assert_eq!(count, 26);
        Ok(())
    }
}
