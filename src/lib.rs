// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use pathfinding::prelude::{kuhn_munkres_min, Matrix};

type Grid = Vec<Vec<i16>>;

pub fn accelkeys(lines: &[&str], alphabet: &str) -> Vec<String> {
    let alphabet: Vec<char> = alphabet.chars().collect();
    let mut weights = initial_weights(alphabet.len());
    update_weights(lines, &alphabet, &mut weights);
    let weights = Matrix::from_rows(weights).unwrap();
    let (_, indexes) = kuhn_munkres_min(&weights);
    lines_with_accelerators(lines, &alphabet, &indexes)
}

fn initial_weights(size: usize) -> Grid {
    let value = (size * 20) as i16;
    let mut weights = Vec::with_capacity(size);
    for row in 0..size {
        weights.push(Vec::with_capacity(size));
        for column in 0..size {
            // favor furthest rows and nearest columns
            weights[row].push(value - row as i16 + column as i16);
        }
    }
    weights
}

fn update_weights(lines: &[&str], alphabet: &[char], weights: &mut Grid) {
    let first = alphabet.len() as i16;
    let start_of_word = first * 5;
    let anywhere = first * 10;
    for (row, line) in lines.iter().enumerate() {
        let mut prev = '\x01';
        for (column, c) in line.chars().enumerate() {
            let c = c.to_ascii_uppercase();
            if let Some(i) = find(alphabet, c) {
                let mut weight = -2 * row as i16;
                if column == 0 {
                    weight += first;
                } else if prev.is_ascii_whitespace() {
                    weight += start_of_word - column as i16;
                } else if prev != '&' {
                    weight += anywhere - column as i16;
                }
                if weights[row][i] > weight {
                    weights[row][i] = weight;
                }
            }
            prev = c;
        }
    }
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
) -> Vec<String> {
    let mut accelerated = vec![];
    for (row, column) in indexes.iter().enumerate() {
        let c = alphabet[*column];
        if row < lines.len() {
            let line = lines[row];
            if line.contains('&') {
                accelerated.push(line.to_string());
                continue;
            }
            let uline = line.to_ascii_uppercase().replace('\t', " ");
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
    accelerated
}
