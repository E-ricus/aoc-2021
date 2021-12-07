use anyhow::Result;

use crate::runner::{Parse, Run};

pub struct Day1 {}

impl Parse<Vec<i32>> for Day1 {
    fn parse_input(input: &str) -> Result<Vec<i32>> {
        Ok(input.lines().filter_map(|s| s.parse().ok()).collect())
    }
}
impl Run<Vec<i32>, i32> for Day1 {
    fn part_one(input: &Vec<i32>) -> Result<i32> {
        Ok(count_increased(input))
    }

    fn part_two(input: &Vec<i32>) -> Result<i32> {
        Ok(count_increased(&clean_measures(input)))
    }
}

fn count_increased(measures: &[i32]) -> i32 {
    let mut count = -1;
    measures.iter().fold(0, |acc, m| {
        if m > &acc {
            count += 1;
        }
        *m
    });
    count
}

fn clean_measures(measures: &[i32]) -> Vec<i32> {
    let mut new_measurents = Vec::new();
    for (i, m) in measures.iter().enumerate() {
        if i + 2 < measures.len() {
            new_measurents.push(m + measures[i + 1] + measures[i + 2])
        }
    }
    new_measurents
}

#[cfg(test)]
mod tests_day1 {
    use super::*;
    use crate::runner::Executor;
    use anyhow::Result;

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day1::run("inputs/day1.test")?;
        assert_eq!(r1, 7);
        assert_eq!(r2, 5);
        Ok(())
    }

    #[test]
    fn test_count_increased() {
        let measures = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = count_increased(&measures);
        assert_eq!(7, count);
    }

    #[test]
    fn test_clean_measunes() {
        let measures = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = clean_measures(&measures);
        assert_eq!(8, count.len());
        let mut it = count.into_iter();
        assert_eq!(607, it.next().unwrap());
        assert_eq!(618, it.next().unwrap());
        assert_eq!(792, it.last().unwrap());
    }
}
