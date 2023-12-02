use aoc2023::day01;
use aoc2023::day02;

fn main() {
    let day = std::env::args().nth(1).unwrap_or_default();
    match day.as_str() {
        "1" => day01::run(),
        "2" => day02::run(),
        _ => {
            println!("No match for day argument '{day}' found. Running most recent day.");
            day02::run();
        }
    }
}
