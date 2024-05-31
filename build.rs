fn main() {
    let proto_files = &["proto/base.proto"];
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("proto/base_descriptor.bin")
        .compile(proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
}
