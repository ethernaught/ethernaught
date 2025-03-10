fn main() {
    #[cfg(profile = "debug")]
    println!("cargo:rustc-env=PROFILE=DEBUG");

    #[cfg(profile = "nightly")]
    println!("cargo:rustc-env=PROFILE=NIGHTLY");

    #[cfg(profile = "release")]
    println!("cargo:rustc-env=PROFILE=RELEASE");

    println!("cargo:rerun-if-env-changed=PROFILE");
}
