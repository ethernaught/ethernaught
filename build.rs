use std::env;

fn main() {
    let profile = env::var("BUILD_PROFILE").unwrap_or_else(|_| String::from("debug"));

    println!("cargo:rustc-cfg=profile=\"{}\"", profile);

    println!("cargo:rerun-if-env-changed=CARGO_PROFILE");

    println!("cargo:rerun-if-changed=src/*");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
