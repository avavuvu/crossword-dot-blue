pub mod grid;
pub mod ipuz;
pub mod puz;
pub mod puzzle;
pub mod xd;
pub mod utils;

use puzzle::Puzzle;

pub const EXTENSIONS: &[&str] = &["ipuz", "puz", "xd"];

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    Puz(#[from] puz::Error),
    #[error(transparent)]
    Ipuz(#[from] ipuz::Error),
    #[error(transparent)]
    Xd(#[from] xd::Error),
    #[error("the file is not valid UTF-8")]
    Utf8,
    #[error("unsupported file type {0:?}, use .ipuz, .puz or .xd")]
    UnsupportedExtension(String),
}

pub fn parse(extension: &str, bytes: &[u8]) -> Result<Puzzle, ParseError> {
    let extension = extension.trim_start_matches('.').to_ascii_lowercase();
    let text = || std::str::from_utf8(bytes).map_err(|_| ParseError::Utf8);

    match extension.as_str() {
        "puz" => Ok(puz::parse(bytes)?),
        "ipuz" => Ok(ipuz::parse(text()?)?),
        "xd" => Ok(xd::parse(text()?)?),
        _ => Err(ParseError::UnsupportedExtension(extension)),
    }
}
