// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use std::{env, fs::File, io::prelude::*, process};

fn main() {
    let mut show_quality = false;
    let mut filenames = vec![];
    for arg in env::args().skip(1) {
        match &arg[..] {
            "-h" | "--help" => {
                println!("accelkeys [-q|--quality] in.txt");
                process::exit(1);
            }
            "-q" | "--quality" => show_quality = true,
            _ => filenames.push(arg),
        }
    }
    for filename in filenames.iter() {
        process_file(&filename, show_quality);
    }
}

fn process_file(filename: &str, show_quality: bool) {
    match File::open(filename) {
        Ok(mut file) => {
            let mut text = String::new();
            match file.read_to_string(&mut text) {
                Ok(_) => process_text(&text, show_quality),
                Err(err) => println!("failed to read {err}"),
            }
        }
        Err(err) => println!("failed to open {err}"),
    }
}

fn process_text(text: &str, show_quality: bool) {
    let chunks: Vec<&str> = if text.contains("\n\n") {
        text.split("\n\n").collect()
    } else {
        vec![text]
    };
    for chunk in chunks.iter() {
        let mut lines = vec![];
        for line in chunk.split('\n') {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                lines.push(line);
            }
        }
        match accelkeys::accelkeys(&lines) {
            Ok(accelerated) => {
                for line in accelerated.iter() {
                    println!("{line}");
                }
                if show_quality {
                    match accelkeys::quality(&accelerated) {
                        Ok(quality) => {
                            let quality = quality * 100.0;
                            println!("# Quality = {quality:.0}%")
                        }
                        Err(err) => {
                            println!("failed to compute quality: {err}")
                        }
                    }
                }
            }
            Err(err) => println!("error: {err}"),
        }
        println!();
    }
}
