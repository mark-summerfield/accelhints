// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub fn main() {
    let lines =
        ["&Undo", "Redo", "&Copy", "Cut", "Paste", "Find", "Find Again"];
    let accels = accelkeys::accelkeys(&lines);
    dbg!(accels);
    let lines = [
        "&Bold",
        "Italic",
        "Underline",
        "No Super- or Sub-script",
        "Superscript",
        "Subscript",
        "Text Color",
        "Font",
        "No List",
        "Bullet List",
        "Numbered List",
        "Align Left",
        "Center",
        "Justify",
        "Align Right",
    ];
    let accels = accelkeys::accelkeys(&lines);
    dbg!(accels);
    let lines =
        ["abc", "bca", "cab", "aab", "bbc", "cca", "cba", "bcb", "acc"];
    let accels = accelkeys::accelkeys(&lines);
    dbg!(accels);
}
