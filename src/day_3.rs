use anyhow::Result;

use crate::runner::{Parse, Run};

pub struct Day3 {}

impl Parse<Vec<Vec<char>>> for Day3 {
    fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
        let r = input.lines().map(|s| s.chars().collect()).collect();
        Ok(r)
    }
}

impl Run<Vec<Vec<char>>, isize> for Day3 {
    fn part_one(input: &Vec<Vec<char>>) -> Result<isize> {
        let (gama, epsilon) = binaries_for_power(input)?;
        let gama = isize::from_str_radix(gama.as_str(), 2)?;
        let epsilon = isize::from_str_radix(epsilon.as_str(), 2)?;
        Ok(gama * epsilon)
    }

    fn part_two(input: &Vec<Vec<char>>) -> Result<isize> {
        let des = |ones: u32, zeros: u32| -> char {
            let ch: char;
            if ones >= zeros {
                ch = '0';
            } else {
                ch = '1';
            }
            ch
        };
        let oxigen = binaries_for_life(input.to_vec(), des)?;
        let des = |ones: u32, zeros: u32| -> char {
            let ch: char;
            if ones >= zeros {
                ch = '1';
            } else {
                ch = '0';
            }
            ch
        };
        let co_scrubber = binaries_for_life(input.to_vec(), des)?;
        let oxigen = isize::from_str_radix(oxigen.as_str(), 2)?;
        let co_scrubber = isize::from_str_radix(co_scrubber.as_str(), 2)?;
        Ok(oxigen * co_scrubber)
    }
}

fn count(input: &[Vec<char>], i: usize) -> (u32, u32) {
    let mut ones: u32 = 0;
    let mut zeros: u32 = 0;
    for line in input {
        match line[i] {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => panic!("invalid input"),
        }
    }
    (ones, zeros)
}

fn binaries_for_power(input: &[Vec<char>]) -> Result<(String, String)> {
    let mut gama = String::new();
    let mut epsilon = String::new();
    let len = input[0].len();
    for i in 0..len {
        let (ones, zeros) = count(input, i);
        if ones > zeros {
            gama.push('1');
            epsilon.push('0');
        } else {
            gama.push('0');
            epsilon.push('1');
        }
    }
    Ok((gama, epsilon))
}

fn binaries_for_life(mut input: Vec<Vec<char>>, desition: fn(u32, u32) -> char) -> Result<String> {
    let mut i = 0;
    while input.len() > 1 {
        let (ones, zeros) = count(&input, i);
        let ch = desition(ones, zeros);
        input = input.into_iter().filter(|v| v[i] == ch).collect();
        i += 1;
    }
    let result: String = input[0].clone().into_iter().collect();
    Ok(result)
}

#[cfg(test)]
mod tests_day3 {
    use super::*;
    use crate::runner::Executor;

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day3::run("inputs/day3.test")?;
        assert_eq!(r1, 198);
        assert_eq!(r2, 230);
        Ok(())
    }

    const INPUT: &str = include_str!("../inputs/day3.test");

    #[test]
    fn test_power_part_one() -> Result<()> {
        let input = Day3::parse_input(INPUT)?;
        let power = Day3::part_one(&input)?;
        assert_eq!(power, 198);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = Day3::parse_input(INPUT)?;
        let life_supply = Day3::part_two(&input)?;
        assert_eq!(life_supply, 230);
        Ok(())
    }

    #[test]
    fn test_binaries_for_power() -> Result<()> {
        let input = Day3::parse_input(INPUT)?;
        let (gama, epsilon) = binaries_for_power(&input)?;
        assert_eq!(gama, "10110");
        assert_eq!(epsilon, "01001");
        Ok(())
    }

    #[test]
    fn test_binaries_for_life_oxygen() -> Result<()> {
        let input = Day3::parse_input(INPUT)?;
        let des = |ones: u32, zeros: u32| -> char {
            let ch: char;
            if ones >= zeros {
                ch = '1';
            } else {
                ch = '0';
            }
            ch
        };
        let ox = binaries_for_life(input, des)?;
        assert_eq!(ox, "10111");
        Ok(())
    }

    #[test]
    fn test_binaries_for_life_co() -> Result<()> {
        let input = Day3::parse_input(INPUT)?;
        let des = |ones: u32, zeros: u32| -> char {
            let ch: char;
            if ones >= zeros {
                ch = '0';
            } else {
                ch = '1';
            }
            ch
        };
        let co = binaries_for_life(input, des)?;
        assert_eq!(co, "01010");
        Ok(())
    }
}
