use num::integer::lcm;
use std::{cmp::Ordering, collections::HashMap};

use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(8);
    println!("Running Day 8");
    println!(
        "Part 1 :: How many steps are required to reach ZZZ? --> {:?}",
        part1(&input)
    );
    println!("Part 2 :: How many steps does it take before you're only on nodes that end with Z? --> {:?}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (instructions, node_maps, _) = parse_input(input);
    // Simpler case, get the number of steps for the known 'AAA' start node with known 'ZZZ' end node
    get_num_steps("AAA", &instructions, &node_maps, |n| n == "ZZZ")
}

fn part2(input: &str) -> usize {
    // `parse_input` returns starting nodes that begin with 'A'
    let (instructions, node_maps, nodes) = parse_input(input);
    // Find the number of steps for each starting node
    let nodes: Vec<usize> = nodes
        .into_iter()
        .map(|n| get_num_steps(&n, &instructions, &node_maps, |n| n.ends_with('Z')))
        .collect();
    // Find the lowest common multiple of all starting node steps, which is the first time
    // they'll all be at their destination nodes after the same number of steps
    lowest_common_multiple(nodes)
}

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>, Vec<String>) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap_or_else(|| panic!("Unable to parse first line from '{input}'"));
    let mut starting_nodes = vec![];
    let element_maps = lines
        .skip(1)
        .map(|line| {
            if line[0..3].ends_with('A') {
                starting_nodes.push(line[0..3].to_string());
            }
            (
                line[0..3].to_string(),
                (line[7..10].to_string(), line[12..15].to_string()),
            )
        })
        .collect();
    (instructions.to_string(), element_maps, starting_nodes)
}

fn get_instruction(instructions: &str, index: usize) -> char {
    instructions
        .chars()
        .nth(index % instructions.len())
        .unwrap()
}

fn get_next_node(
    cur_node: &str,
    instruction: char,
    node_maps: &HashMap<String, (String, String)>,
) -> String {
    let next_node_map = node_maps
        .get(cur_node)
        .unwrap_or_else(|| unreachable!("Unable to find node {cur_node}"));
    match instruction {
        'L' => next_node_map.0.clone(),
        _ => next_node_map.1.clone(),
    }
}

fn get_num_steps(
    node: &str,
    instructions: &str,
    node_maps: &HashMap<String, (String, String)>,
    is_destination: fn(&str) -> bool,
) -> usize {
    let mut step_count = 0;
    let mut cur_node = String::from(node);
    while !is_destination(&cur_node) {
        cur_node = get_next_node(
            &cur_node,
            get_instruction(instructions, step_count),
            node_maps,
        );
        step_count += 1;
        // println!("step: {step_count}, cur_element: {cur_node}");
    }
    step_count
}

fn lowest_common_multiple(nums: Vec<usize>) -> usize {
    match nums.len().cmp(&2) {
        Ordering::Less => panic!("nums length is < 2"),
        Ordering::Greater => lcm(nums[0], lowest_common_multiple(nums[1..].to_vec())),
        _ => lcm(nums[0], nums[1]),
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::{lowest_common_multiple, part1, part2};

    const SAMPLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ";

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(6, part1(SAMPLE_INPUT));
    }

    const SAMPLE_INPUT_P2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(6, part2(SAMPLE_INPUT_P2));
    }

    #[test]
    fn test_lowest_common_multiple() {
        assert_eq!(6, lowest_common_multiple(vec![2, 3]));
        assert_eq!(1000, lowest_common_multiple(vec![2, 100, 1000]));
        assert_eq!(90, lowest_common_multiple(vec![2, 3, 5, 9]));
    }
}
