use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(6);
    println!("Running Day 6");
    println!(
        "Part 1 :: What do you get if you multiply these numbers together? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: How many ways can you beat the record in this one much longer race? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    // Get the number of ways to win for each race and take the product of all results
    input_to_races(input)
        .iter()
        .map(|r| r.num_ways_to_win())
        .product()
}

fn part2(input: &str) -> usize {
    // Parse the input as a single race and return the number of the ways to win
    let start = std::time::Instant::now();
    // let res = input_as_kerning_to_race(input).num_ways_to_win();
    let num_ways_to_win = input_as_kerning_to_race(input).num_ways_to_win_efficient();
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
    num_ways_to_win
}

#[derive(Debug)]
struct Race {
    time: usize,
    current_record: usize,
}

impl Race {
    // This is sufficiently fast for the puzzle input, ~1180ms
    fn num_ways_to_win(&self) -> usize {
        (1..self.time)
            .filter(|t| t * (self.time - t) > self.current_record)
            .collect::<Vec<usize>>()
            .len()
    }

    // Instead of validating all times in the middle of the winning range,
    // find the shortest and longest winning time. All items between are
    // also winners, so longest - shortest + 1 is the number of ways to win
    // (+1 because the range is inclusive). This runs in ~100ms, ~12x faster.
    fn num_ways_to_win_efficient(&self) -> usize {
        let mut min = 0;
        for t in 1..self.time {
            if t * (self.time - t) > self.current_record {
                min = t;
                break;
            }
        }
        if min == 0 {
            return 0; // No ways to win
        }
        let mut max = self.time;
        for t in (min..=self.time).rev() {
            if t * (self.time - t) > self.current_record {
                max = t;
                break;
            }
        }
        max - min + 1
    }
}

fn input_to_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().map(parse_line_to_usizes).unwrap();
    let distances = lines.next().map(parse_line_to_usizes).unwrap();
    times
        .zip(distances)
        .map(|(time, current_record)| Race {
            time,
            current_record,
        })
        .collect()
}

fn input_as_kerning_to_race(input: &str) -> Race {
    let mut lines = input.lines();
    Race {
        time: lines
            .next()
            .map(parse_line)
            .unwrap()
            .fold(String::from(""), |acc, t| acc + t)
            .parse::<usize>()
            .unwrap_or_else(|e| panic!("Unable to parse number: {e}")),
        current_record: lines
            .next()
            .map(parse_line)
            .unwrap()
            .fold(String::from(""), |acc, t| acc + t)
            .parse::<usize>()
            .unwrap_or_else(|e| panic!("Unable to parse number: {e}")),
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = &str> {
    line.split(':')
        .last()
        .unwrap_or_else(|| panic!("Could not find ':' in line '{line}'"))
        .split_whitespace()
}

fn parse_line_to_usizes(line: &str) -> impl Iterator<Item = usize> + '_ {
    parse_line(line).map(|num| num.parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day06::{part1, part2};

    const SAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(288, part1(SAMPLE_INPUT));
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(71503, part2(SAMPLE_INPUT));
    }
}
