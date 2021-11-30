use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() {
    println!("Hello, world!");
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
