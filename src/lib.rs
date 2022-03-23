// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::{bail, Result};
use pathfinding::prelude::{kuhn_munkres, Matrix};
use std::collections::HashMap;

type Weight = i8;
type Grid = Vec<Vec<Weight>>;
type StringVec = Vec<String>;

pub fn accelkeys(lines: &[&str]) -> Result<StringVec> {
    accelkeys_alphabet(lines, "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789")
}

pub fn accelkeys_alphabet(
    lines: &[&str],
    alphabet: &str,
) -> Result<StringVec> {
    let alphabet: Vec<char> = alphabet.chars().collect();
    if lines.len() >= alphabet.len() {
        bail!("can't have more items than alphabet characters");
    }
    let weights = get_weights(lines, &alphabet);
    let weights = Matrix::from_rows(weights).unwrap();
    let (_, indexes) = kuhn_munkres(&weights);
    lines_with_accelerators(lines, &alphabet, &indexes)
}

fn get_weights(lines: &[&str], alphabet: &[char]) -> Grid {
    let size = alphabet.len();
    let mut weights = Vec::with_capacity(size);
    for _ in 0..size {
        weights.push(vec![0; size]);
    }
    for (row, line) in lines.iter().enumerate() {
        let mut prev = '\x01';
        for (column, c) in line.chars().enumerate() {
            let c = c.to_ascii_uppercase();
            if let Some(i) = find(alphabet, c) {
                let weight = if column == 0 {
                    4 // first
                } else if prev == '&' {
                    99 // preset
                } else if prev.is_ascii_whitespace() {
                    2 // word start
                } else {
                    1 // anywhere
                };
                if weights[row][i] < weight {
                    weights[row][i] = weight;
                }
            }
            prev = c;
        }
    }
    weights
}

fn find<T>(sequence: &[T], what: T) -> Option<usize>
where
    T: std::cmp::PartialEq,
{
    for (i, x) in sequence.iter().enumerate() {
        if *x == what {
            return Some(i);
        }
    }
    None
}

fn lines_with_accelerators(
    lines: &[&str],
    alphabet: &[char],
    indexes: &[usize],
) -> Result<StringVec> {
    let mut accelerated = vec![];
    for (row, column) in indexes.iter().enumerate() {
        let c = alphabet[*column];
        if row < lines.len() {
            let line = lines[row];
            if line.contains('&') {
                accelerated.push(line.to_string());
                continue;
            }
            let uline = line.to_ascii_uppercase();
            let index = if uline.starts_with(c) {
                Some(0) // first is best
            } else if let Some(i) = uline.find(&format!(" {c}")) {
                Some(i + 1) // skip the space to start of word
            } else {
                uline.find(c) // anywhere or nowhere
            };
            let mut line = line.to_string();
            if let Some(index) = index {
                line.insert(index, '&');
            }
            accelerated.push(line);
        }
    }
    Ok(accelerated)
}

/// Returns a quality rating in the range 0.0 to 1.0 where 0.0 means no
/// accelerators and 1.0 means all lines begin with an accelerator.
pub fn quality(lines: &[String]) -> Result<f64> {
    let mut count_for_char = HashMap::new();
    let mut target = 0.0;
    let mut actual = 0.0;
    let size = lines.len();
    for (row, line) in lines.iter().enumerate() {
        let ideal = (25.0 * size as f64) + row as f64;
        target += ideal;
        let line: Vec<char> = line.chars().collect();
        let mut factor = -1.0;
        let mut index = line.len();
        let mut prev = '\x01';
        for (i, c) in line.iter().enumerate() {
            if *c == '&' {
                index = i + 1;
                if i == 0 {
                    factor = 25.0; // preset
                    break;
                } else if prev.is_ascii_whitespace() {
                    factor = 5.0; // word start
                    break;
                } else {
                    factor = 1.0; // anywhere; no break - keep looking
                }
            }
            prev = *c;
        }
        if factor > 0.0 {
            if index >= line.len() {
                bail!("& at end of line");
            }
            actual +=
                (factor * size as f64) + row as f64 - ((index - 1) as f64);
            let c = line[index];
            if let Some(count) = count_for_char.get_mut(&c) {
                *count += 1;
            } else {
                count_for_char.insert(c, 1);
            }
        }
    }
    if isclose64(target, 0.0) {
        bail!("missing lines");
    }
    for (c, count) in count_for_char.iter() {
        if count > &1 {
            bail!("&{c} occurs {count} times");
        }
    }
    Ok(actual / target)
}

fn isclose64(a: f64, b: f64) -> bool {
    (a..=(a + f64::EPSILON)).contains(&b)
}
