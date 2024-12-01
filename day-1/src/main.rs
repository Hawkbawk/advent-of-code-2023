use std::fs::{self};

use regex::{Captures, Regex};

fn main() {
    env_logger::init();

    let input = &fs::read_to_string("puzzle.txt").expect("Error reading file");
    let first = part_one(input);
    println!("First half: {first}");
    let second = part_two(input);
    println!("Second half: {second}")
}

fn part_one(input: &String) -> u32 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .fold(0, |acc, line| {
            let digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            log::debug!(
                "first: {}, second: {}",
                digits.first().unwrap(),
                digits.last().unwrap()
            );

            acc + digits.first().unwrap() * 10 + digits.last().unwrap()
        })
}

fn part_two(input: &String) -> u32 {
    let pattern = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let parsed = pattern.replace_all(input, |result: &Captures| match &result[1] {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => unreachable!(),
    });

    log::debug!("{}", parsed);
    log::debug!(
        "{}",
        parsed
            .lines()
            .map(|line| { line.chars().filter(|c| c.is_digit(10)).collect::<String>() })
            .collect::<Vec<String>>()
            .join("\n")
    );

    part_one(&parsed.to_string())
}

#[cfg(test)]
mod test;
