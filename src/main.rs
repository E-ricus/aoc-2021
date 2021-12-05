mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod runner;

use anyhow::Result;
use day_one::Day1;
use runner::Runner;

fn main() -> Result<()> {
    Day1::run("inputs/day1.input")?;
    day_two::run_day_two()?;
    day_three::run_day_three()?;
    day_four::run_day_four("inputs/day4.input")?;
    Ok(())
}
