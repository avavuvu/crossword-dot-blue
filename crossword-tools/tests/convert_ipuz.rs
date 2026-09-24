use std::fs;
use std::path::PathBuf;

use crossword_tools::ipuz;
use crossword_tools::xd::write::write_xd;

fn fixture(dir: &str, name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(dir)
        .join(name)
}

fn read(dir: &str, name: &str) -> String {
    let path = fixture(dir, name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn assert_converts(name: &str) {
    let input = read("input", &format!("{name}.ipuz"));
    let expected = read("output", &format!("{name}.xd"));

    let puzzle = ipuz::parse(&input).expect("ipuz parse failed");
    let actual = write_xd(&puzzle);

    assert_eq!(actual.trim_end(), expected.trim_end());
}

#[test]
fn converts_a_ipuz_to_xd() {
    assert_converts("a");
}

#[test]
fn converts_b_ipuz_to_xd() {
    assert_converts("b");
}
