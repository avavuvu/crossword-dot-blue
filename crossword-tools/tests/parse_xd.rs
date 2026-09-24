use std::fs;
use std::path::PathBuf;

use crossword_tools::puzzle::{Cell, Style};
use crossword_tools::xd::write::write_xd;
use crossword_tools::{ipuz, xd};

fn read(dir: &str, name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(dir)
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn assert_xd_matches_ipuz(name: &str) {
    let from_ipuz = ipuz::parse(&read("input", &format!("{name}.ipuz"))).expect("ipuz parse failed");
    let from_xd = xd::parse(&read("output", &format!("{name}.xd"))).expect("xd parse failed");

    assert_eq!(from_xd, from_ipuz);
}

#[test]
fn a_xd_matches_a_ipuz() {
    assert_xd_matches_ipuz("a");
}

#[test]
fn b_xd_matches_b_ipuz() {
    assert_xd_matches_ipuz("b");
}

#[test]
fn round_trip_is_stable() {
    for name in ["a.xd", "b.xd", "COOL S.xd"] {
        let first = write_xd(&xd::parse(&read("output", name)).unwrap());
        let second = write_xd(&xd::parse(&first).unwrap());
        assert_eq!(first, second, "{name} changed on the second round trip");
    }
}

#[test]
fn parses_cool_s() {
    let puzzle = xd::parse(&read("output", "COOL S.xd")).expect("xd parse failed");

    assert_eq!((puzzle.width, puzzle.height), (9, 15));
    assert_eq!(puzzle.meta.title.as_deref(), Some("COOL S"));
    assert_eq!(puzzle.meta.author.as_deref(), Some("Brandon Cathcart"));
    assert!(puzzle.meta.notes.as_deref().unwrap().contains("Cool_S"));
    assert!(!puzzle.meta.extra.contains_key("editor"));

    assert_eq!(puzzle.cells[0], Cell::Block { styles: vec![Style::Empty] });
    assert_eq!(puzzle.cells[9 + 1], Cell::Block { styles: vec![] });
    assert!(matches!(&puzzle.cells[3], Cell::Letter { solution, styles, .. } if solution == "L" && styles.is_empty()));

    assert_eq!(puzzle.clue_order.len(), 36);
    assert_eq!(puzzle.clues["A1"].indexes, vec![3, 4, 5]);
    assert_eq!(puzzle.clues["D1"].answer, "LONDONER");
    assert_eq!(puzzle.clues["A29"].body, "Very versatile bits of plastic");
}

const TINY: &str = "\
## Metadata

title: Tiny
rebus: 1=XY
splitcharacter: |

## Grid

1AT
ONE
PEN

## Clues

A1. one ~ XY|AT
A4. two ~ ONE
A5. three ~ PEN

D1. four ~ XY|OP
D2. five ~ ANE
D3. six ~ TEN
";

#[test]
fn reads_rebus_and_splits() {
    let puzzle = xd::parse(TINY).expect("xd parse failed");

    assert_eq!(puzzle.rebuses["1"], "XY");
    assert!(matches!(&puzzle.cells[0], Cell::Letter { solution, .. } if solution == "XY"));

    let a1 = &puzzle.clues["A1"];
    assert_eq!(a1.answer, "XYAT");
    assert_eq!(a1.splits, vec![1]);
    assert_eq!(puzzle.clues["D1"].splits, vec![1]);
    assert!(puzzle.clues["A4"].splits.is_empty());

    let written = write_xd(&puzzle);
    assert!(written.contains("splitcharacter: |"));
    assert!(written.contains("A1. one ~ XY|AT"));
    assert!(written.contains("rebus: 1=XY"));
}

#[test]
fn rejects_split_inside_rebus() {
    let text = TINY.replace("~ XY|AT", "~ X|YAT");
    assert!(matches!(xd::parse(&text), Err(xd::Error::SplitInRebus { id, .. }) if id == "A1"));
}

#[test]
fn rejects_answer_mismatch() {
    let text = read("output", "b.xd").replace("~ GASPS", "~ GASPX");
    assert!(matches!(xd::parse(&text), Err(xd::Error::AnswerMismatch { id, .. }) if id == "A1"));
}

#[test]
fn rejects_unknown_style() {
    let text = read("output", "COOL S.xd").replace("E: empty", "E: glow");
    assert!(matches!(xd::parse(&text), Err(xd::Error::UnknownStyle { name, .. }) if name == "glow"));
}

#[test]
fn rejects_unknown_section() {
    let text = read("output", "b.xd").replace("## Clues", "## Hints");
    assert!(matches!(xd::parse(&text), Err(xd::Error::UnknownSection { name, .. }) if name == "hints"));
}

#[test]
fn rejects_missing_clue() {
    let text = read("output", "b.xd").replace("A6. Baby in blue, traditionally ~ BOY\n", "");
    assert!(matches!(xd::parse(&text), Err(xd::Error::MissingClue(id)) if id == "A6"));
}

#[test]
fn skips_comments() {
    let text = read("output", "b.xd").replace("## Grid", "<!-- a comment -->\n## Grid");
    xd::parse(&text).expect("comment should be ignored");
}
