extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "src/proto/auth.proto",
            "src/proto/general.proto",
            "src/proto/track.proto",
            "src/proto/upload.proto",
            "src/proto/profile.proto",
            "src/proto/playlists.proto",
        ],
        &["src/"],
    )
    .unwrap();
}
