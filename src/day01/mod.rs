use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(1);
    println!("Running Day 1");
    println!(
        "Part 1 :: What is the sum of all of the calibration values? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: What is the sum of all of the calibration values? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    let matchers: Vec<(&str, usize)> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    input
        .split("\r\n")
        .map(|l| calibration_value_for_line(l, &matchers))
        .sum()
}

fn part2(input: &str) -> usize {
    let matchers: Vec<(&str, usize)> = vec![
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    input
        .split("\r\n")
        .map(|l| calibration_value_for_line(l, &matchers))
        .sum()
}

fn calibration_value_for_line(line: &str, matchers: &[(&str, usize)]) -> usize {
    // Find the first instance of each matcher in the line, then reduce to the one with the lowest index
    let first = matchers
        .iter()
        .filter_map(|(string_to_match, val)| line.find(string_to_match).map(|i| (i, val)))
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .expect("Failed to find first digit in {l}")
        .1;

    // Find the last instance of each matcher in the line, then reduce to the one with the highest index
    let last = matchers
        .iter()
        .filter_map(|(string_to_match, val)| line.rfind(string_to_match).map(|i| (i, val)))
        .reduce(|a, b| if a.0 > b.0 { a } else { b })
        .expect("Failed to find the last digit in {l}")
        .1;

    // Combine the first and last to make one two-digit number
    10 * first + last
}

#[cfg(test)]
mod tests {
    use crate::day01::{part1, part2};

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(
            part1(
                "1abc2\r\n
        pqr3stu8vwx\r\n
        a1b2c3d4e5f\r\n
        treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(
            part2(
                "two1nine\r\n
        eightwothree\r\n
        abcone2threexyz\r\n
        xtwone3four\r\n
        4nineeightseven2\r\n
        zoneight234\r\n
        7pqrstsixteen"
            ),
            281
        );
    }

    #[test]
    fn test_part2_overlapping_strings() {
        assert_eq!(
            part2(
                "eighthree\r\n
                sevenine\r\n
                oneight"
            ),
            83 + 79 + 18
        );
        assert_eq!(
            part2(
                "eighthree\r\n
                sevenine4\r\n
                4oneight"
            ),
            83 + 74 + 48
        );
    }
}
