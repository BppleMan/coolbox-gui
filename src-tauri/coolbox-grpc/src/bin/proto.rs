use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let proto_dir = manifest_dir.join("proto");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(manifest_dir.join("src"))
        .compile(&[proto_dir.join("coolbox.proto")], &[proto_dir])
        .unwrap();
}
