mod day_one;
mod day_two;

use anyhow::Result;

fn main() -> Result<()> {
    day_one::run_day_one()?;
    day_two::run_day_two()?;
    Ok(())
}
