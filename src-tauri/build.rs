use cool::Cool;
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "macos")]
    tauri_build::build();
}

fn _generate_schema() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let schemas_dir = manifest_dir.join("schemas");
    let cool_schema = schemas_dir.join("cool.json");
    let schema = schemars::schema_for!(Cool);
    std::fs::write(cool_schema, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}
