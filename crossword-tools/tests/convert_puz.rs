use std::fs;
use std::path::PathBuf;

use crossword_tools::puz;
use crossword_tools::xd::write::write_xd;

fn fixture(dir: &str, name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(dir)
        .join(name)
}

fn assert_converts(name: &str) {
    let input_path = fixture("input", &format!("{name}.puz"));
    let expected_path = fixture("output", &format!("{name}.xd"));

    let input = fs::read(&input_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", input_path.display()));
    let expected = fs::read_to_string(&expected_path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", expected_path.display()));

    let puzzle = puz::parse(&input).expect("puz parse failed");
    let actual = write_xd(&puzzle);

    assert_eq!(actual.trim_end(), expected.trim_end());
}

#[test]
fn converts_b_puz_to_xd() {
    assert_converts("b");
}
