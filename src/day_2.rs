use anyhow::Result;

use crate::runner::{Parse, Run};

pub struct Day2 {}

impl Parse<Vec<Movements>> for Day2 {
    fn parse_input(input: &str) -> Result<Vec<Movements>> {
        let r = input
            .lines()
            .filter_map(|s| match s.split_once(' ') {
                Some(("forward", x)) => Some(Movements::Forward(x.parse().ok()?)),
                Some(("down", x)) => Some(Movements::Down(x.parse().ok()?)),
                Some(("up", x)) => Some(Movements::Up(x.parse().ok()?)),
                _ => None,
            })
            .collect();
        Ok(r)
    }
}

impl Run<Vec<Movements>, u32> for Day2 {
    fn part_one(input: &Vec<Movements>) -> Result<u32> {
        let mut x_pos = 0;
        let mut y_pos = 0;
        input.iter().for_each(|movement| match movement {
            Movements::Forward(x) => x_pos += x,
            Movements::Down(x) => y_pos += x,
            Movements::Up(x) => y_pos -= x,
        });
        Ok(x_pos * y_pos)
    }

    fn part_two(input: &Vec<Movements>) -> Result<u32> {
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut aim = 0;
        input.iter().for_each(|movement| match movement {
            Movements::Forward(x) => {
                x_pos += x;
                y_pos += aim * x;
            }
            Movements::Down(x) => aim += x,
            Movements::Up(x) => aim -= x,
        });
        Ok(x_pos * y_pos)
    }
}

#[derive(Debug, Clone)]
pub enum Movements {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[cfg(test)]
mod tests_day2 {
    use super::*;
    use crate::runner::Executor;
    use anyhow::Result;

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day2::run("inputs/day2.test")?;
        assert_eq!(r1, 150);
        assert_eq!(r2, 900);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let movements = vec![
            Movements::Forward(5),
            Movements::Down(5),
            Movements::Forward(8),
            Movements::Up(3),
            Movements::Down(8),
            Movements::Forward(2),
        ];
        let pos = Day2::part_one(&movements)?;
        assert_eq!(pos, 150);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let movements = vec![
            Movements::Forward(5),
            Movements::Down(5),
            Movements::Forward(8),
            Movements::Up(3),
            Movements::Down(8),
            Movements::Forward(2),
        ];
        let pos = Day2::part_two(&movements)?;
        assert_eq!(pos, 900);
        Ok(())
    }
}
