use std::fs;

pub mod day01;

pub fn parse_file_input(day: u16) -> String {
    fs::read_to_string(format!("./data/day{:02}.txt", day)).unwrap()
}
