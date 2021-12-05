mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod runner;

use anyhow::Result;
use day_one::Day1;
use day_three::Day3;
use day_two::Day2;
use runner::Runner;

fn main() -> Result<()> {
    Day1::run("inputs/day1.input")?;
    Day2::run("inputs/day2.input")?;
    Day3::run("inputs/day3.input")?;
    day_four::run_day_four("inputs/day4.input")?;
    Ok(())
}
