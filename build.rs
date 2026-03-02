/*
REQUIRED
rustup install nightly
rustup override set nightly

TESTING
export GTK_DEBUG=interactive
$env:GTK_DEBUG="interactive"

COMPILE
glib-compile-resources res/gtk4/linux.gresources.xml --target=res/resources.gresources
glib-compile-resources res/gtk4/windows.gresources.xml --target=res/resources.gresources

WINDOWS TEST BUILD
cargo build --release
powershell -ExecutionPolicy Bypass -File tools\package.ps1

WINDOWS BUILD
gdk-pixbuf-query-loaders --update-cache

$env:PATH="C:\Windows\System32;C:\Windows"
Start-Process -FilePath .\target\release\ethernaught.exe
*/

fn main() {
    #[cfg(profile = "debug")]
    println!("cargo:rustc-env=PROFILE=DEBUG");

    #[cfg(profile = "nightly")]
    println!("cargo:rustc-env=PROFILE=NIGHTLY");

    #[cfg(profile = "release")]
    println!("cargo:rustc-env=PROFILE=RELEASE");

    println!("cargo:rerun-if-env-changed=PROFILE");
}
