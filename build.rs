#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    println!("cargo:rustc-flags=-l fuse3");
    println!("cargo:rustc-flags=-l glib-2.0");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
