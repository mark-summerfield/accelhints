// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub static INPUT: &str = include_str!("../tests/input.txt");
pub static EXPECTED: &str = include_str!("../tests/expected.txt");

#[test]
fn files() {
    let inputs: Vec<&str> = INPUT.split("\n\n").collect();
    let expecteds: Vec<&str> = EXPECTED.split("\n\n").collect();
    assert_eq!(inputs.len(), expecteds.len());
    for i in 0..inputs.len() {
        let input: Vec<&str> = inputs[i].split('\n').collect();
        let expected: Vec<&str> = expecteds[i].split('\n').collect();
        let actual = accelkeys::accelkeys(&input);
        assert_eq!(actual, expected, "list #{}", i + 1);
    }
}
