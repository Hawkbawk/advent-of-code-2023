use super::*;

#[test]
fn test_part_one_simple() {
    let input = "1\n2\n3";

    let result = part_one(&input.to_string());

    assert_eq!(result, 66);
}

#[test]
fn test_part_one_empty_lines() {
    let input = "1\n\n2\n3\n             ";

    let result = part_one(&input.to_string());

    assert_eq!(result, 66);
}

#[test]
fn test_part_one_multiple_digits() {
    let input = "12\n56\n14\n19";

    let result = part_one(&input.to_string());

    assert_eq!(result, 101)
}

#[test]
fn test_part_one_multiple_digits_with_characters() {
    let input = "one12two\nnine5eight6\n1eleven4eight\ntwo1one9";

    assert_eq!(part_one(&input.to_string()), 101)
}

#[test]
fn test_part_two_simple() {
    let input = "one\ntwo\nthree";

    assert_eq!(part_two(&input.to_string()), 66)
}

#[test]
fn test_part_two_mixed_simple() {
    let input = "one\n2\n3";

    assert_eq!(part_two(&input.to_string()), 66);
}

#[test]
fn test_part_two_empty_lines() {
    let input = "one\n2\nthree\n      ";

    assert_eq!(part_two(&input.to_string()), 66);
}

#[test]
fn test_part_two_multiple_digits() {
    let input = "one2\nthree4five\n67eight";

    assert_eq!(part_two(&input.to_string()), 115);
}

#[test]
fn test_part_two_complex() {
    let input = "one23452two\nthreefourfoureight194\nsix17234eightnineonefoureight";

    assert_eq!(part_two(&input.to_string()), 114);
}

#[test]
fn test_part_two_some_garbage() {
    let input = "asdflhkjhoiuyqwegqwer1\nasdf;lkjpoiu65\neight14asdfopiupoiuwqer9";

    assert_eq!(part_two(&input.to_string()), 165);
}

#[test]
fn test_part_two_real_data() {
    let input = "pffldcmnlpsevensixqxhdncrclbc51five\n5bqnlphone6\n195one";

    assert_eq!(part_two(&input.to_string()), 142);
}

#[test]
fn test_part_two_example_given() {
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    assert_eq!(part_two(&input.to_string()), 281);
}
