use std::cmp::{max, min};

use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(3);
    println!("Running Day 3");
    println!(
        "Part 1 :: What is the sum of all of the part numbers in the engine schematic? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: What is the sum of all of the gear ratios in your engine schematic? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    let schematic = input_to_schematic(input);

    // Iterate over each row and sum each number that has an adjacent symbol
    let mut sum_of_part_numbers = 0;
    for (i, row) in schematic.iter().enumerate() {
        let mut row_sum = 0;
        let mut number_start_ind = None;
        // Iterate over each character, keeping track of numbers using number_start_ind
        for (j, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                if number_start_ind.is_none() {
                    number_start_ind = Some(j);
                }
            } else if let Some(start_index) = number_start_ind {
                if has_adjacent_symbol(&schematic, i, start_index, j - 1) {
                    row_sum += parse_number_from_point_range(&schematic, i, start_index, j - 1);
                }
                number_start_ind = None;
            }
        }
        // Handle numbers that end at the end of the row
        if let Some(start_index) = number_start_ind {
            if has_adjacent_symbol(&schematic, i, start_index, row.len() - 1) {
                row_sum += parse_number_from_point_range(&schematic, i, start_index, row.len() - 1);
            }
        }
        sum_of_part_numbers += row_sum;
    }

    sum_of_part_numbers
}

fn part2(input: &str) -> usize {
    let schematic = input_to_schematic(input);

    // Find each "*" and get all adjacent numbers, adding the gear ratio the total if
    // there are exactly 2 adjacent numbers.
    let mut sum_of_gear_ratios = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '*' {
                let numbers = get_numbers_from_points(
                    &schematic,
                    &get_adjacent_char_indicies(&schematic, |c| c.is_ascii_digit(), i, j, j),
                );
                if numbers.len() == 2 {
                    sum_of_gear_ratios += numbers[0] * numbers[1];
                }
            }
        }
    }

    sum_of_gear_ratios
}

type Schematic = Vec<Vec<char>>;

fn input_to_schematic(input: &str) -> Schematic {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn has_adjacent_symbol(
    schematic: &Schematic,
    row_index: usize,
    col_index_start: usize,
    col_index_end: usize,
) -> bool {
    !get_adjacent_char_indicies(
        schematic,
        |c| !c.is_ascii_digit() && c != '.',
        row_index,
        col_index_start,
        col_index_end,
    )
    .is_empty()
}

fn get_adjacent_char_indicies<F>(
    schematic: &Schematic,
    char_match: F,
    row_index: usize,
    col_index_start: usize,
    col_index_end: usize,
) -> Vec<(usize, usize)>
where
    F: Fn(char) -> bool,
{
    let mut char_indicies: Vec<(usize, usize)> = vec![];
    // Col range from 1 left of the start and 1 right of the end
    let col_range = max(0, col_index_start as isize - 1) as usize
        ..=min(schematic[0].len() - 1, col_index_end + 1);

    // Row above
    if row_index > 0 {
        for j in col_range.clone() {
            if char_match(schematic[row_index - 1][j]) {
                char_indicies.push((row_index - 1, j));
            }
        }
    }

    // Same row
    if col_index_start > 0 && char_match(schematic[row_index][col_index_start - 1]) {
        char_indicies.push((row_index, col_index_start - 1));
    }
    if col_index_end < schematic[0].len() - 1 && char_match(schematic[row_index][col_index_end + 1])
    {
        char_indicies.push((row_index, col_index_end + 1));
    }

    // Row below
    if row_index < schematic[0].len() - 1 {
        for j in col_range {
            if char_match(schematic[row_index + 1][j]) {
                char_indicies.push((row_index + 1, j));
            }
        }
    }

    char_indicies
}

fn parse_number_from_point_range(
    schematic: &Schematic,
    row_index: usize,
    col_index_start: usize,
    col_index_end: usize,
) -> usize {
    schematic[row_index][col_index_start..=col_index_end]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .expect("Failed to parse number")
}

fn parse_number_from_point(schematic: &Schematic, row_index: usize, col_index: usize) -> usize {
    let mut number_string = String::from(schematic[row_index][col_index]);

    if col_index > 0 {
        for j in (0..col_index).rev() {
            if schematic[row_index][j].is_ascii_digit() {
                number_string.insert_str(0, &schematic[row_index][j].to_string());
            } else {
                break;
            }
        }
    }

    if col_index < schematic[row_index].len() - 1 {
        for j in col_index + 1..schematic[row_index].len() {
            if schematic[row_index][j].is_ascii_digit() {
                number_string.push(schematic[row_index][j]);
            } else {
                break;
            }
        }
    }

    number_string
        .parse()
        .unwrap_or_else(|_| panic!("Unable to parse number from {number_string}"))
}

fn get_numbers_from_points(schematic: &Schematic, points: &[(usize, usize)]) -> Vec<usize> {
    let mut numbers: Vec<usize> = vec![];

    if !points.is_empty() {
        let mut cur_point = points[0];
        numbers.push(parse_number_from_point(schematic, cur_point.0, cur_point.1));

        for (i, p) in points.iter().enumerate() {
            if i == 0 {
                continue;
            } else if cur_point.0 != p.0 || p.1 as isize - cur_point.1 as isize != 1 {
                numbers.push(parse_number_from_point(schematic, p.0, p.1));
            }
            cur_point = *p;
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use crate::day03::{input_to_schematic, parse_number_from_point_range, part1, part2};
    const SAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse_number_from_point_range() {
        let schematic = input_to_schematic(SAMPLE_INPUT);
        assert_eq!(467, parse_number_from_point_range(&schematic, 0, 0, 2));
        assert_eq!(114, parse_number_from_point_range(&schematic, 0, 5, 7));
        assert_eq!(617, parse_number_from_point_range(&schematic, 4, 0, 2));
        assert_eq!(664, parse_number_from_point_range(&schematic, 9, 1, 3));
        assert_eq!(598, parse_number_from_point_range(&schematic, 9, 5, 7));
    }

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(4361, part1(SAMPLE_INPUT))
    }

    #[test]
    fn test_part1_custom_input() {
        assert_eq!(
            0,
            part1(
                "467..114..
..........
..35..633.
..........
617.......
.......58.
..592.....
......755.
..........
.664.598.."
            )
        )
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(467835, part2(SAMPLE_INPUT));
    }
}
