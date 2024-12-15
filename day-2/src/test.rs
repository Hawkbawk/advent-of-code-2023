use super::*;

#[test]
fn test_simple_example() {
    let input = "Game 1: 22 green; 11 blue; 19 red
Game 2: 1 red; 3 green, 2 blue; 9 green";
    let result = part_one(&input.to_string(), NUM_RED, NUM_GREEN, NUM_BLUE);
    assert_eq!(result, 2);
}

#[test]
fn test_given_example() {
    env_logger::init();
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(
        part_one(&input.to_string(), NUM_RED, NUM_GREEN, NUM_BLUE),
        8
    );
}
