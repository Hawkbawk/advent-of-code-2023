use std::fs;

use regex::Regex;

const NUM_RED: u64 = 12;
const NUM_GREEN: u64 = 13;
const NUM_BLUE: u64 = 14;

fn main() {
    env_logger::init();
    let input = fs::read_to_string("puzzle.txt").expect("Couldn't find the puzzle input");
    let result = part_one(&input, NUM_RED, NUM_GREEN, NUM_BLUE);
    println!("The sum of the id's of all valid games is {result}");

    let result = part_two(&input);

    println!("The answer to part two is: {result}")
}

struct Game {
    id: u64,
    sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    num_red: u64,
    num_blue: u64,
    num_green: u64,
}

fn create_games(input: &String) -> Vec<Game> {
    let red_count_regex = Regex::new(r"(\d+) red").unwrap();
    let green_count_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_count_regex = Regex::new(r"(\d+) blue").unwrap();
    let game_num_regex = Regex::new(r"Game (\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let game_num = game_num_regex
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let sets = line.split(";").collect::<Vec<&str>>();
            log::debug!("Parsing game {game_num}");
            let sets = sets
                .iter()
                .map(|set| {
                    let num_red = match red_count_regex.captures(set) {
                        Some(captures) => captures[1].parse::<u64>().unwrap(),
                        None => 0,
                    };
                    let num_blue = match blue_count_regex.captures(set) {
                        Some(captures) => captures[1].parse::<u64>().unwrap(),
                        None => 0,
                    };
                    let num_green = match green_count_regex.captures(set) {
                        Some(captures) => captures[1].parse::<u64>().unwrap(),
                        None => 0,
                    };
                    CubeSet {
                        num_red,
                        num_blue,
                        num_green,
                    }
                })
                .collect::<Vec<CubeSet>>();
            log::debug!("Sets are as follows: {:?}", sets);

            Game { id: game_num, sets }
        })
        .collect::<Vec<Game>>()
}

fn part_two(input: &String) -> u64 {
    let games = create_games(input);

    let cube_sets = games.iter().map(|game| {
        let num_red = game.sets.iter().map(|s| s.num_red).max().unwrap();
        let num_green = game.sets.iter().map(|s| s.num_green).max().unwrap();
        let num_blue = game.sets.iter().map(|s| s.num_blue).max().unwrap();

        log::debug!(
            "For game {}, here are the max number of each cube needed:\nr: {}, g: {}, b: {}",
            game.id,
            num_red,
            num_green,
            num_blue
        );

        CubeSet {
            num_red,
            num_blue,
            num_green,
        }
    });

    cube_sets.fold(0, |acc, item| {
        acc + item.num_red * item.num_green * item.num_blue
    })
}

fn part_one(input: &String, num_red: u64, num_green: u64, num_blue: u64) -> u64 {
    let games = create_games(input);

    games.iter().fold(0, |acc, game| {
        let invalid = game
            .sets
            .iter()
            .any(|e| e.num_red > num_red || e.num_green > num_green || e.num_blue > num_blue);

        if invalid {
            log::debug!("Game {} is invalid", game.id);
            acc
        } else {
            log::debug!("Game {} is valid", game.id);
            acc + game.id
        }
    })
}

#[cfg(test)]
mod test;
