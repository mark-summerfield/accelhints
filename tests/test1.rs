// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

fn approx_equal64(a: f64, b: f64, limit: f64) -> bool {
    (a - b).abs() < limit
}

#[test]
fn edit() {
    let lines =
        ["Undo", "Redo", "Copy", "Cu&t", "Paste", "Find", "Find Again"];
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
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    let quality = accelkeys::quality(&actual[..]);
    assert!(quality.is_ok());
    let quality = quality.unwrap();
    assert!(approx_equal64(quality, 0.741, 0.001), "{} != 0.741", quality);
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
        "&Superscript",
        "Subscri&pt",
        "&Text Color",
        "&Font",
        "&No List",
        "Bullet &List",
        "Numbere&d List",
        "&Align Left",
        "&Center",
        "&Justify",
        "Align &Right",
    ];
    let actual = accelkeys::accelkeys(&lines);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    let quality = accelkeys::quality(&actual);
    assert!(quality.is_ok());
    let quality = quality.unwrap();
    assert!(approx_equal64(quality, 0.708, 0.001), "{} != 0.708", quality);
}

#[test]
fn pathological() {
    let lines =
        ["abc", "bca", "cab", "aab", "bbc", "cca", "cba", "bcb", "acc"];
    let expected =
        ["&abc", "&bca", "&cab", "aab", "bbc", "cca", "cba", "bcb", "acc"];
    let actual = accelkeys::accelkeys(&lines);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    let quality = accelkeys::quality(&actual);
    assert!(quality.is_ok(), "{:?}", quality);
    let quality = quality.unwrap();
    assert!(approx_equal64(quality, 0.327, 0.001), "{} != 0.327", quality);
}
