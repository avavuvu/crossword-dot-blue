use std::{env, fs, path::Path};

const CONTENT_DIR: &str = "resources/content";
const THEMES: &str = "resources/css/themes.css";
const MANIFEST: &str = "public/build/.vite/manifest.json";

fn main() {
    println!("cargo:rerun-if-changed={MANIFEST}");
    println!("cargo:rustc-env=VITE_MANIFEST={MANIFEST}");
    if env::var("PROFILE").as_deref() == Ok("release") && !Path::new(MANIFEST).exists() {
        panic!("{MANIFEST} not found: run `bun run build` before a release build");
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    let out = Path::new(&out_dir);

    render_content(&out.join("content"));
    write_themes(&out.join("themes.rs"));
}

fn render_content(out: &Path) {
    fs::create_dir_all(out).expect("create content output dir");
    println!("cargo:rerun-if-changed={CONTENT_DIR}");

    let entries = fs::read_dir(CONTENT_DIR).expect("read resources/content");
    for entry in entries {
        let path = entry.expect("read dir entry").path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }

        println!("cargo:rerun-if-changed={}", path.display());

        let source = fs::read_to_string(&path).expect("read markdown file");
        let html = markdown::to_html(&source);

        let name = path.file_stem().and_then(|stem| stem.to_str()).expect("markdown file name");
        fs::write(out.join(format!("{name}.html")), html).expect("write rendered html");
    }
}

fn write_themes(out: &Path) {
    println!("cargo:rerun-if-changed={THEMES}");
    let css = fs::read_to_string(THEMES).expect("read themes.css");

    let mut names: Vec<&str> = Vec::new();
    for rest in css.split("[data-theme=\"").skip(1) {
        let Some(end) = rest.find('"') else { continue };
        let name = &rest[..end];
        if !name.is_empty() && !names.contains(&name) {
            names.push(name);
        }
    }

    let mut source = String::from("pub const THEMES: &[(&str, &str)] = &[\n    (\"\", \"Default\"),\n");
    for name in names {
        source.push_str(&format!("    ({name:?}, {:?}),\n", label(name)));
    }
    source.push_str("];\n");

    fs::write(out, source).expect("write themes.rs");
}

fn label(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut characters = word.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().chain(characters).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
