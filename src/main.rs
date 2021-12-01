mod first_day;

use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() {
    println!("Hello, world!");
    first_day::run_first_day();
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
