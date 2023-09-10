use std::process::Command;

fn main() {
    prost_build::compile_protos(
        &[
            "native/sdk/src/proto/auth.proto",
            "native/sdk/src/proto/general.proto",
            "native/sdk/src/proto/track.proto",
            "native/sdk/src/proto/upload.proto",
            "native/sdk/src/proto/profile.proto",
            "native/sdk/src/proto/playlists.proto",
        ],
        &["native/sdk/src/proto/"],
    )
    .unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config {
        language: cbindgen::Language::C,
        ..Default::default()
    };
    config.braces = cbindgen::Braces::SameLine;
    config.cpp_compat = true;
    config.no_includes = true;

    config.style = cbindgen::Style::Both;
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("native/sdk/binding.h");
    Command::new("dart")
        .arg("run")
        .arg("ffigen")
        .spawn()
        .expect("Dart ffi gen not success");
    Command::new("protoc")
        .arg("--proto_path=native/sdk/src")
        .arg("--dart_out=native/ffi/lib/gen")
        .arg("native/sdk/src/proto/general.proto")
        .arg("native/sdk/src/proto/auth.proto")
        .arg("native/sdk/src/proto/track.proto")
        .arg("native/sdk/src/proto/upload.proto")
        .arg("native/sdk/src/proto/profile.proto")
        .arg("native/sdk/src/proto/playlists.proto")
        .spawn()
        .expect("Dart ffi gen not success");
}
