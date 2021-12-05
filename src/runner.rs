use std::fmt::Display;

use anyhow::Result;

pub trait Day<I, R> {
    fn parse_input(input: &str) -> Result<I>;
    fn part_one(input: &I) -> Result<R>;
    fn part_two(input: &I) -> Result<R>;
}

pub trait Runner<I, R, T>
where
    T: Day<I, R>,
{
    fn run(path: &str) -> Result<(R, R)>;
}

impl<I, R, T> Runner<I, R, T> for T
where
    T: Day<I, R>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)> {
        let input = std::fs::read_to_string(path)?;
        let input = <T as Day<I, R>>::parse_input(input.as_str())?;
        let r1 = <T as Day<I, R>>::part_one(&input)?;
        println!("{} part 1: {}", path, r1);
        let r2 = <T as Day<I, R>>::part_two(&input)?;
        println!("{} part 2: {}", path, r2);
        Ok((r1, r2))
    }
}
