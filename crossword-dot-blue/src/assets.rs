use std::{collections::HashMap, fs, sync::OnceLock};

use serde::Deserialize;

pub const ROUTE: &str = "/build";
pub const DIR: &str = "public/build";
const MANIFEST: &str = "public/build/.vite/manifest.json";

#[derive(Deserialize)]
struct Chunk {
    file: String,
    name: Option<String>,
    #[serde(default, rename = "isEntry")]
    is_entry: bool,
}

type Manifest = HashMap<String, String>;

static MANIFEST_CACHE: OnceLock<Manifest> = OnceLock::new();

fn read() -> Option<Manifest> {
    let text = fs::read_to_string(MANIFEST).ok()?;
    let chunks: HashMap<String, Chunk> = serde_json::from_str(&text).ok()?;
    Some(
        chunks
            .into_values()
            .filter(|chunk| chunk.is_entry)
            .filter_map(|chunk| chunk.name.map(|name| (name, chunk.file)))
            .collect(),
    )
}

pub fn init() {
    if cfg!(debug_assertions) {
        if read().is_none() {
            eprintln!("[assets] {MANIFEST} not found, run `bun run build` (or `bun run dev`)");
        }
        return;
    }

    let manifest = read().unwrap_or_else(|| panic!("{MANIFEST} not found, run `bun run build` before starting"));
    MANIFEST_CACHE.set(manifest).ok();
}

pub fn url(name: &str) -> String {
    let file = if cfg!(debug_assertions) {
        read().and_then(|manifest| manifest.get(name).cloned())
    } else {
        MANIFEST_CACHE.get().and_then(|manifest| manifest.get(name).cloned())
    };

    match file {
        Some(file) => format!("{ROUTE}/{file}"),
        None => {
            eprintln!("[assets] no entry named {name:?} in {MANIFEST}");
            format!("{ROUTE}/{name}")
        }
    }
}
