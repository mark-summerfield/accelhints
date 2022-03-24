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
    let actual = accelhints::accelhints(&lines);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    let quality = accelhints::quality(&actual[..]);
    assert!(quality.is_ok());
    let quality = quality.unwrap();
    let q = 0.747;
    assert!(approx_equal64(quality, q, 0.001), "{} != {}", quality, q);
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
    let actual = accelhints::accelhints(&lines);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    let quality = accelhints::quality(&actual);
    assert!(quality.is_ok());
    let quality = quality.unwrap();
    let q = 0.71;
    assert!(approx_equal64(quality, q, 0.001), "{} != {}", quality, q);
}

#[test]
fn pathological() {
    let lines =
        ["abc", "bca", "cab", "aab", "bbc", "cca", "cba", "bcb", "acc"];
    let expected =
        ["&abc", "&bca", "&cab", "aab", "bbc", "cca", "cba", "bcb", "acc"];
    let actual = accelhints::accelhints(&lines);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    let quality = accelhints::quality(&actual);
    assert!(quality.is_ok(), "{:?}", quality);
    let quality = quality.unwrap();
    let q = 0.328;
    assert!(approx_equal64(quality, q, 0.001), "{} != {}", quality, q);
}
