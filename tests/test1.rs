// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[test]
fn edit() {
    let lines =
        ["&Undo", "Redo", "&Copy", "Cut", "Paste", "Find", "Find Again"];
    let expected = [
        "&Undo",
        "&Redo",
        "&Copy",
        "Cu&t",
        "&Paste",
        "&Find",
        "Find &Again",
    ];
    let actual = accelkeys::accelkeys(&lines);
    assert_eq!(actual, expected);
}

#[test]
fn style() {
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
    let expected = [
        "&Bold",
        "&Italic",
        "&Underline",
        "No Super- &or Sub-script",
        "Su&perscript",
        "&Subscript",
        "&Text Color",
        "&Font",
        "&No List",
        "Bullet &List",
        "Nu&mbered List",
        "&Align Left",
        "&Center",
        "&Justify",
        "Align &Right",
    ];
    let actual = accelkeys::accelkeys(&lines);
    assert_eq!(actual, expected);
}

#[test]
fn pathalogical() {
    let lines =
        ["abc", "bca", "cab", "aab", "bbc", "cca", "cba", "bcb", "acc"];
    let expected =
        ["abc", "bca", "cab", "aab", "bbc", "cca", "&cba", "&bcb", "&acc"];
    let actual = accelkeys::accelkeys(&lines);
    assert_eq!(actual, expected);
}
