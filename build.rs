use std::{env, path::PathBuf};

fn main() {
    let proto_files = &["proto/base.proto"];
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("base_descriptor.bin");
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(descriptor_path)
        .compile(proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {e}"));

    for file in proto_files {
        println!("cargo:rerun-if-changed={file}");
    }
}
