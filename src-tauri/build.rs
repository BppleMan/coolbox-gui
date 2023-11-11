use cool::Cool;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    generate_schema();
    build_ask_pass();
    tauri_build::build();
}

fn build_ask_pass() {
    Command::new("cargo")
        .args([
            "build",
            format!("--{}", env::var("PROFILE").unwrap()).as_str(),
            "-p",
            "coolbox-askpass",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn generate_schema() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let schemas_dir = manifest_dir.join("schemas");
    let cool_schema = schemas_dir.join("cool.json");
    let schema = schemars::schema_for!(Cool);
    std::fs::write(cool_schema, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}
