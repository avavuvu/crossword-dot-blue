use std::{env, fs, path::Path};

const CONTENT_DIR: &str = "resources/content";
const MANIFEST: &str = "public/build/.vite/manifest.json";

fn main() {
    println!("cargo:rerun-if-changed={MANIFEST}");
    if env::var("PROFILE").as_deref() == Ok("release") && !Path::new(MANIFEST).exists() {
        panic!("{MANIFEST} not found: run `bun run build` before a release build");
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    let out = Path::new(&out_dir).join("content");
    fs::create_dir_all(&out).expect("create content output dir");

    println!("cargo:rerun-if-changed={CONTENT_DIR}");

    let entries = fs::read_dir(CONTENT_DIR).expect("read resources/content");
    for entry in entries {
        let path = entry.expect("read dir entry").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        println!("cargo:rerun-if-changed={}", path.display());

        let source = fs::read_to_string(&path).expect("read markdown file");
        let html = markdown::to_html(&source);

        let name = path.file_stem().and_then(|stem| stem.to_str()).expect("markdown file name");
        fs::write(out.join(format!("{name}.html")), html).expect("write rendered html");
    }
}
