use std::fs::{self};

use regex::Regex;

fn main() {
    println!("The answer for the first half follows:");
    part_one();
    println!("The answer for the second half follows:");
    part_two();
}

fn part_one() {
    let result = fs::read_to_string("puzzle.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .fold(0 as u32, |acc, line| {
            let digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>();

            acc + digits.first().unwrap().to_digit(10).unwrap() * 10
                + digits.last().unwrap().to_digit(10).unwrap()
        });

    println!("The sum is: {result}");
}

fn part_two() {
    let pattern = Regex::new("one|two|three|four|five|six|seven|eight|nine|\\d").unwrap();
    let result = fs::read_to_string("puzzle.txt")
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }

            Some(
                pattern
                    .find_iter(line)
                    .map(|m| match m.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => m.as_str().parse().unwrap(),
                    })
                    .collect::<Vec<i32>>(),
            )
        })
        .fold(0, |acc, digits| {
            acc + digits.first().unwrap() * 10 + digits.last().unwrap()
        });

    println!("The result after considering spelled out digits is: {result}");
}
