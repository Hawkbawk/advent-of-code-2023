use std::fs;

use regex::Regex;

const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

fn main() {
    env_logger::init();
    let input = fs::read_to_string("puzzle.txt").expect("Couldn't find the puzzle input");
    let result = part_one(&input, NUM_RED, NUM_GREEN, NUM_BLUE);
    println!("The sum of all valid games is {}", result)
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    num_red: u32,
    num_blue: u32,
    num_green: u32,
}

fn part_one(input: &String, num_red: u32, num_green: u32, num_blue: u32) -> u32 {
    let red_count_regex = Regex::new(r"(\d+) red").unwrap();
    let green_count_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_count_regex = Regex::new(r"(\d+) blue").unwrap();

    let game_num_regex = Regex::new(r"Game (\d+)").unwrap();
    let games = input.lines().map(|line| {
        let game_num = game_num_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let sets = line.split(";").collect::<Vec<&str>>();
        log::debug!("Parsing game {game_num}");
        let sets = sets
            .iter()
            .map(|set| {
                let num_red = match red_count_regex.captures(set) {
                    Some(captures) => captures[1].parse::<u32>().unwrap(),
                    None => 0,
                };
                let num_blue = match blue_count_regex.captures(set) {
                    Some(captures) => captures[1].parse::<u32>().unwrap(),
                    None => 0,
                };
                let num_green = match green_count_regex.captures(set) {
                    Some(captures) => captures[1].parse::<u32>().unwrap(),
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
    });

    games.fold(0, |acc, game| {
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
