fn main() {
    let proto_files = &[
        "proto/hello.proto",
        "proto/add_numbers.proto",
        "proto/base.proto",
        "proto/language.proto",
    ];
    tonic_build::configure()
        .build_server(true)
        .compile(proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
}
