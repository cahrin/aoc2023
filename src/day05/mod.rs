use std::cmp::{max, min};

use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(5);
    println!("Running Day 5");
    println!(
        "Part 1 :: What is the lowest location number that corresponds to any of the initial seed numbers? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: What is the lowest location number that corresponds to any of the initial seed numbers? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    let (seeds, categories) = parse_input_to_seeds_and_maps(input);
    let mut destinations: Vec<usize> = vec![];
    // Process one seed at a time for all categories, building a destinations set that has
    // all the final locations of the initial seeds.
    for seed in seeds {
        let mut dest = seed;
        for category in categories.iter() {
            dest = category.get_destination(dest);
        }
        destinations.push(dest);
    }
    *destinations.iter().min().unwrap()
}

fn part2(input: &str) -> usize {
    // Same idea as part1 but use ranges of seeds instead of single seeds.
    // Start by going category-by-category and mapping the ranges.
    // If a range isn't fully contained in a mapping, first map the contained part,
    // then create new seed ranges for the un-contained segments and process those
    // for the same category.
    // At the end, the 'lowest' location from each range is the only relevant part.
    // The solution is then the lowest range start value of all the seed location ranges.
    let (seeds, categories) = parse_input_to_seeds_and_maps(input);

    // Construct a vector of ranges in the form (start, end)
    let mut seed_ranges = Vec::new();
    for seed_range in seeds.chunks_exact(2) {
        seed_ranges.push((seed_range[0], seed_range[0] + seed_range[1] - 1))
    }

    for category in categories {
        let mut destination_ranges = vec![];
        while let Some(seed_range) = seed_ranges.pop() {
            let mut had_mapping_match = false;
            for map in category.maps.iter() {
                // Determine if there is any overlap in the seed_range and the mapping range
                if map.source_range_start <= seed_range.1
                    && map.source_range_start + map.range_length > seed_range.0
                {
                    // If the end of the seed_range extends beyond the end of the mapping range, then
                    // create a new seed_range starting from the end of the mapping range until
                    // the end of the seed range.
                    if seed_range.1 > map.source_range_start + map.range_length - 1 {
                        seed_ranges.push((map.source_range_start + map.range_length, seed_range.1));
                    }

                    // If the start of the seed_range is earlier than the start of the mapping range,
                    // then create a new seed_range starting from the start of the seed range until
                    // the start of the mapping range.
                    if seed_range.0 < map.source_range_start {
                        seed_ranges.push((seed_range.0, map.source_range_start - 1));
                    }

                    // Handle mapping for portion of the seed_range that matches the current map
                    destination_ranges.push((
                        max(seed_range.0, map.source_range_start) - map.source_range_start
                            + map.destination_range_start,
                        min(seed_range.1, map.source_range_start + map.range_length - 1)
                            - map.source_range_start
                            + map.destination_range_start,
                    ));

                    had_mapping_match = true;
                    break;
                }
            }
            // If there was no mapping match for the current Category, then push the unchanged seed_range
            // forward to the next Category.
            if !had_mapping_match {
                destination_ranges.push(seed_range);
            }
        }

        seed_ranges = destination_ranges;
    }
    seed_ranges.iter().map(|s| s.0).min().unwrap()
}

#[derive(Debug)]
struct Category {
    maps: Vec<Map>,
}

impl Category {
    fn get_destination(&self, source: usize) -> usize {
        for map in self.maps.iter() {
            if let Some(destination) = map.get_destination(source) {
                return destination;
            }
        }
        source
    }
}
#[derive(Copy, Clone, Debug)]
struct Map {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl Map {
    fn get_destination(&self, source: usize) -> Option<usize> {
        if source >= self.source_range_start && source < self.source_range_start + self.range_length
        {
            Some(self.destination_range_start + source - self.source_range_start)
        } else {
            None
        }
    }
}

fn parse_input_to_seeds_and_maps(input: &str) -> (Vec<usize>, Vec<Category>) {
    let mut lines = input.lines();
    let seeds: Vec<usize> = lines
        .next()
        .expect("Failed to parse first line of input")
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| {
            seed.parse()
                .unwrap_or_else(|e| panic!("Failed to parse seed num from '{seed}': {e}"))
        })
        .collect();

    let mut categories: Vec<Category> = vec![];
    let mut cur_category_maps: Vec<Map> = vec![];
    for line in lines {
        if line.trim().is_empty() {
            // indicates the end of a category - push the cur_category_maps as a new category
            if !cur_category_maps.is_empty() {
                categories.push(Category {
                    maps: cur_category_maps.clone(),
                });
            }
            continue;
        } else if line.contains(':') {
            // indicates a new category - clear cur_category_maps
            cur_category_maps.clear();
        } else {
            // it's a map line, parse and add to cur_category_maps
            let parts: Vec<usize> = line
                .split_whitespace()
                .map(|part| {
                    part.parse().unwrap_or_else(|e| {
                        panic!("Could not parse number from mapping part: {part}: {e}")
                    })
                })
                .collect();
            cur_category_maps.push(Map {
                destination_range_start: parts[0],
                source_range_start: parts[1],
                range_length: parts[2],
            })
        }
    }
    // Add the final category
    if !cur_category_maps.is_empty() {
        categories.push(Category {
            maps: cur_category_maps,
        });
    }

    (seeds, categories)
}

#[cfg(test)]
mod tests {
    use crate::day05::{part1, part2, Map};

    #[test]
    fn test_map_get_destination() {
        assert_eq!(
            81,
            Map {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            }
            .get_destination(79)
            .unwrap()
        );

        // seed-to-soil section from real input
        assert_eq!(
            None,
            Map {
                destination_range_start: 3305253869,
                source_range_start: 1699909104,
                range_length: 39566623,
            }
            .get_destination(1848591090)
        );

        assert_eq!(
            None,
            Map {
                destination_range_start: 3344820492,
                source_range_start: 1130725752,
                range_length: 384459310,
            }
            .get_destination(1848591090)
        );

        assert_eq!(
            None,
            Map {
                destination_range_start: 3244681427,
                source_range_start: 1739475727,
                range_length: 60572442,
            }
            .get_destination(1848591090)
        );

        assert_eq!(
            Some(1000060452),
            Map {
                destination_range_start: 951517531,
                source_range_start: 1800048169,
                range_length: 868898709,
            }
            .get_destination(1848591090)
        );
    }

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(35, part1(SAMPLE_INPUT));
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(46, part2(SAMPLE_INPUT));
    }
}
