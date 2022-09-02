use std::{env, path::PathBuf};

fn main() {
  let proto_file = "./proto/movie_store.proto";
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); // Add this

  tonic_build::configure()
    .build_server(true)
    .file_descriptor_set_path(out_dir.join("greeter_descriptor.bin")) // Add this
    .out_dir("./src")
    .compile(&[proto_file], &["."])
    .unwrap_or_else(|error| panic!("protobuf compile error: {error}"));

  println!("cargo:rerun-if-changed={proto_file}");
}
