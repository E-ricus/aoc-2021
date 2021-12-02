mod day_one;
mod day_two;

use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() {
    day_one::run_day_one();
    day_two::run_day_two();
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
