use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(2);
    println!("Running Day 2");
    println!(
        "Part 1 :: What is the sum of the IDs of those games? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: What is the sum of the power of these sets? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    // Parse input into games
    let games = input_to_games(input);

    // Define the "Threshold" game based on the puzzle definition
    let threshold_game = Game {
        game_num: 0,
        red: 12,
        green: 13,
        blue: 14,
    };

    // Filter all games into a set of possible games and then sum their `game_num`s
    games
        .into_iter()
        .filter(|game| is_game_possible(&threshold_game, game))
        .map(|game| game.game_num)
        .sum()
}

fn part2(input: &str) -> usize {
    // Parse input into games
    let games = input_to_games(input);

    // Calculate the power for the minimum set of cubes for each game, then sum them
    games
        .into_iter()
        .map(|game| game.red * game.green * game.blue)
        .sum()
}

#[derive(Debug)]
struct Game {
    game_num: usize,
    red: usize,
    green: usize,
    blue: usize,
}

fn is_game_possible(threshold_game: &Game, game_to_test: &Game) -> bool {
    game_to_test.red <= threshold_game.red
        && game_to_test.green <= threshold_game.green
        && game_to_test.blue <= threshold_game.blue
}

fn input_to_games(input: &str) -> Vec<Game> {
    input.split("\r\n").map(line_to_game).collect()
}

fn line_to_game(line: &str) -> Game {
    let game_split: Vec<&str> = line.split(": ").collect();
    if game_split.len() < 2 {
        unreachable!("Could not find ':' in line: {line}");
    }

    let mut game = Game {
        game_num: game_split[0]
            .split(' ')
            .last()
            .unwrap_or_else(|| panic!("Unexpected format of Game number for line {line}"))
            .parse()
            .unwrap_or_else(|_| panic!("Could not find game number in {}", game_split[0])),
        red: 0,
        green: 0,
        blue: 0,
    };
    for round in game_split[1].split("; ") {
        for color in round.split(", ") {
            let color_split: Vec<&str> = color.split(' ').collect();
            let num_cubes = color_split[0]
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Coudn't find number in {:?}", color));
            match color_split[1] {
                "red" => game.red = std::cmp::max(game.red, num_cubes),
                "green" => game.green = std::cmp::max(game.green, num_cubes),
                "blue" => game.blue = std::cmp::max(game.blue, num_cubes),
                _ => unreachable!("Unexpected color found in round {round}"),
            }
        }
    }
    // println!("Parsed line {line} into game: {:?}", game);
    game
}

#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\r\n
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\r\n
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\r\n
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\r\n
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        )
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\r\n
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\r\n
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\r\n
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\r\n
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        )
    }
}
