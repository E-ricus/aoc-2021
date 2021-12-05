mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod runner;

use anyhow::Result;
use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use runner::Executor;
use runner::MutExecutor;

fn main() -> Result<()> {
    Day1::run("inputs/day1.input")?;
    Day2::run("inputs/day2.input")?;
    Day3::run("inputs/day3.input")?;
    Day4::run("inputs/day4.input")?;
    Ok(())
}
