// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub static INPUT: &str = include_str!("../tests/input.txt");
pub static EXPECTED: &str = include_str!("../tests/expected-rs.txt");

#[test]
fn files() {
    let mut inputs: Vec<&str> = INPUT.split("\n\n").collect();
    trim_empty(&mut inputs);
    let mut expecteds: Vec<&str> = EXPECTED.split("\n\n").collect();
    trim_empty(&mut expecteds);
    assert_eq!(inputs.len(), expecteds.len());
    for i in 0..inputs.len() {
        let mut input: Vec<&str> = inputs[i].split('\n').collect();
        trim_empty(&mut input);
        let mut expected: Vec<&str> = expecteds[i].split('\n').collect();
        trim_empty(&mut expected);
        let actual = accelhints::accelhints(&input);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!(actual, expected, "list #{}", i + 1);
    }
}

fn trim_empty(list: &mut Vec<&str>) {
    while list[list.len() - 1].is_empty() {
        list.pop();
    }
}
