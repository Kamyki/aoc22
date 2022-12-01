use std::fs;
use std::io;

pub mod day01;

pub fn read_lines(filename: &str) -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    let path = cwd.join("inputs").join(filename);
    fs::read_to_string(path)
}
