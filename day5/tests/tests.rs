extern crate day5;

use day5::*;

#[test]
fn test_sample_input1() {
    let input = vec![0 , 3, 0, 1, -3];
    assert_eq!(5, count_steps(&input));
}

#[test]
fn test_sample_input2() {
    let input = vec![0 , 3, 0, 1, -3];
    assert_eq!(10, count_steps_part2(&input));
}