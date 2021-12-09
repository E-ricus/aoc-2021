use std::fmt::Display;

use anyhow::Result;

pub trait Run<I, R> {
    fn part_one(input: &I) -> Result<R>;
    fn part_two(input: &I) -> Result<R>;
}

pub trait Parse<I> {
    fn parse_input(input: &str) -> Result<I>;
}

pub trait ParseWithLifeTime<'a, I> {
    fn parse_input(input: &'a str) -> Result<I>;
}

pub trait RunMut<I, R> {
    fn part_one(input: &mut I) -> Result<R>;
    fn part_two(input: &mut I) -> Result<R>;
}

// There's gotta be better way to implement this, than to have a traits per implementation

pub trait Executor<I, R, T>
where
    T: Run<I, R>,
{
    fn run(path: &str) -> Result<(R, R)>;
}

pub trait MutExecutor<I, R, T>
where
    T: RunMut<I, R> + Parse<I>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)>;
}

impl<I, R, T> Executor<I, R, T> for T
where
    T: Run<I, R> + Parse<I>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)> {
        let input = std::fs::read_to_string(path)?;
        let input = <T as Parse<I>>::parse_input(input.as_str())?;
        let r1 = <T as Run<I, R>>::part_one(&input)?;
        println!("{} part 1: {}", path, r1);
        let r2 = <T as Run<I, R>>::part_two(&input)?;
        println!("{} part 2: {}", path, r2);
        Ok((r1, r2))
    }
}

impl<I, R, T> MutExecutor<I, R, T> for T
where
    T: RunMut<I, R> + Parse<I>,
    R: Display,
    I: Clone,
{
    fn run(path: &str) -> Result<(R, R)> {
        let input = std::fs::read_to_string(path)?;
        let mut input = <T as Parse<I>>::parse_input(input.as_str())?;
        let r1 = <T as RunMut<I, R>>::part_one(&mut input.clone())?;
        println!("{} part 1: {}", path, r1);
        let r2 = <T as RunMut<I, R>>::part_two(&mut input)?;
        println!("{} part 2: {}", path, r2);
        Ok((r1, r2))
    }
}
