use aoc2023::day01;
use aoc2023::day02;
use aoc2023::day03;
use aoc2023::day04;
use aoc2023::day05;

fn main() {
    let day = std::env::args().nth(1).unwrap_or_default();
    match day.as_str() {
        "1" => day01::run(),
        "2" => day02::run(),
        "3" => day03::run(),
        "4" => day04::run(),
        "5" => day05::run(),
        _ => {
            println!("No match for day argument '{day}' found. Running most recent day.");
            day05::run();
        }
    }
}
