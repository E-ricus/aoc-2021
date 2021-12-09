use std::collections::HashMap;

use anyhow::Result;

use crate::runner::{ParseWithLifeTime, Run};

// FIXME: ALL THE SECOND PART IMPL IS HORRIBLE
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
        let r = input.iter().fold(0, |mut acc, m| {
            let codex = find_codex_on_input(&m.input).unwrap();
            let mut value = String::new();
            m.output.iter().for_each(|o| {
                let (_, v) = codex
                    .iter()
                    .find(|(k, _)| contains_all_equal(k, o))
                    .unwrap();
                value.push(*v);
            });
            acc += value.parse::<usize>().unwrap();
            acc
        });
        Ok(r)
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

// FIXME: This is hardcoded brute force... gotta be a better way
fn find_codex_on_input<'a>(input: &'a [&str]) -> Option<HashMap<&'a str, char>> {
    let known_index = HashMap::from([(2, '1'), (4, '4'), (3, '7'), (7, '8')]);
    let mut codex = HashMap::with_capacity(10);
    let mut found = HashMap::with_capacity(10);
    let mut group_six = Vec::new();
    let mut group_five = Vec::new();
    // let mut group_six = HashMap::new();
    // let mut group_five = HashMap::new();
    for entry in input {
        let len = entry.len();
        match len {
            5 => {
                // group_five.insert(len, *entry);
                group_five.push(*entry);
            }
            6 => {
                // group_six.insert(len, *entry);
                group_six.push(*entry);
            }
            _ => {
                codex.insert(*entry, known_index[&len]);
                found.insert(known_index[&len], *entry);
            }
        }
    }
    // Nine is the only one on grop six that contains all 4
    let index = group_six
        .iter()
        .position(|e| contains_all(e, found[&'4']))?;
    let number = group_six.swap_remove(index);
    found.insert('9', number);
    codex.insert(number, '9');

    // two is the only one on grop five that contains nine but for one
    let index = group_five
        .iter()
        .position(|e| contains_but_one(e, found[&'9']))?;
    let number = group_five.swap_remove(index);
    found.insert('2', number);
    codex.insert(number, '2');

    // three is the only one on group five that contains two but for one
    let index = group_five
        .iter()
        .position(|e| contains_but_one(e, found[&'2']))?;
    let number = group_five.swap_remove(index);
    found.insert('3', number);
    codex.insert(number, '3');
    // five is the remaining one on the group
    let number = group_five.pop()?;
    found.insert('5', number);
    codex.insert(number, '5');

    // six contains all 5
    let index = group_six
        .iter()
        .position(|e| contains_all(e, found[&'5']))?;
    let number = group_six.swap_remove(index);
    found.insert('6', number);
    codex.insert(number, '6');
    // zero is the remaining one on the group
    let number = group_six.pop()?;
    codex.insert(number, '0');

    Some(codex)
}

// Why it doesn't work with iterators?
fn contains_but_one(a: &str, b: &str) -> bool {
    let mut count = 0;
    for c in a.chars() {
        if !b.contains(c) {
            count += 1;
        }
    }
    count == 1
    // a.chars().filter(|c| !b.contains(c)).count() == 1
}

fn contains_all(a: &str, b: &str) -> bool {
    let mut count = 0;
    for c in a.chars() {
        if b.contains(c) {
            count += 1;
        }
    }
    count == b.len()
}

fn contains_all_equal(a: &str, b: &str) -> bool {
    let mut count = 0;
    for c in a.chars() {
        if b.contains(c) {
            count += 1;
        }
    }
    count == b.len() && a.len() == b.len()
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
    fn test_part_one() -> Result<()> {
        let input = Day8::parse_input(INPUT)?;
        let count = Day8::part_one(&input)?;
        assert_eq!(count, 26);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = Day8::parse_input(INPUT)?;
        let count = Day8::part_two(&input)?;
        assert_eq!(count, 61229);
        Ok(())
    }

    #[test]
    fn test_codex() {
        let input = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];

        let codex = find_codex_on_input(&input);
        assert!(codex.is_some());
        let codex = codex.unwrap();
        assert_eq!(codex[&"cagedb"], '0');
        assert_eq!(codex[&"ab"], '1');
        assert_eq!(codex[&"acedgfb"], '8');
        assert_eq!(codex[&"cdfbe"], '5');
    }
}
