use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String> where P: AsRef<Path> {
    let file = File::open(filename).expect("file does not exist");
    io::BufReader::new(file).lines().map(|line| line.expect("cannot read line"))
}
