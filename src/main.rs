mod day_one;
mod day_three;
mod day_two;

use anyhow::Result;

fn main() -> Result<()> {
    day_one::run_day_one()?;
    day_two::run_day_two()?;
    day_three::run_day_three()?;
    Ok(())
}
