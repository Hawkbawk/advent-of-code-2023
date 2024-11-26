use std::fs::{self, File};

use regex::Regex;

fn main() {
    println!("The answer for the first half follows:");
    part_one();
}

fn part_one() {
    let mut result: Vec<u32> = vec![];

    let result = fs::read_to_string("puzzle.txt")
        .unwrap()
        .lines()
        .fold(0 as u32, |acc, line| {
            if line.trim().is_empty() {
                return acc;
            }

            let digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>();

            acc + digits.first().unwrap().to_digit(10).unwrap() * 10
                + digits.last().unwrap().to_digit(10).unwrap()
        });

    println!("The sum is: {result}");
}
