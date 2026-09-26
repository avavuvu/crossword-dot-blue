use std::{collections::HashMap, fs, sync::OnceLock};

use serde::Deserialize;

pub const ROUTE: &str = "/build";
pub const DIR: &str = "public/build";
const MANIFEST: &str = "public/build/.vite/manifest.json";

#[derive(Deserialize)]
struct Chunk {
    file: String,
    name: Option<String>,
    #[serde(default)]
    css: Vec<String>,
    #[serde(default, rename = "isEntry")]
    is_entry: bool,
}

pub struct Entry {
    pub file: String,
    pub styles: Vec<String>,
}

type Manifest = HashMap<String, Entry>;

static MANIFEST_CACHE: OnceLock<Manifest> = OnceLock::new();

fn read() -> Option<Manifest> {
    let text = fs::read_to_string(MANIFEST).ok()?;
    let chunks: HashMap<String, Chunk> = serde_json::from_str(&text).ok()?;
    Some(
        chunks
            .into_values()
            .filter(|chunk| chunk.is_entry)
            .filter_map(|chunk| {
                let entry = Entry { file: chunk.file, styles: chunk.css };
                chunk.name.map(|name| (name, entry))
            })
            .collect(),
    )
}

fn lookup<T>(name: &str, pick: impl Fn(&Entry) -> T) -> Option<T> {
    let found = if cfg!(debug_assertions) {
        read().and_then(|manifest| manifest.get(name).map(pick))
    } else {
        MANIFEST_CACHE.get().and_then(|manifest| manifest.get(name).map(pick))
    };
    if found.is_none() {
        eprintln!("[assets] no entry named {name:?} in {MANIFEST}");
    }
    found
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

pub fn url(name: &str) -> Option<String> {
    lookup(name, |entry| format!("{ROUTE}/{}", entry.file))
}

pub fn styles(name: &str) -> Vec<String> {
    lookup(name, |entry| entry.styles.iter().map(|file| format!("{ROUTE}/{file}")).collect()).unwrap_or_default()
}
