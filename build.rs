fn main() {
    println!("cargo:rustc-link-search=fmod/macos");
    println!("cargo:rustc-link-lib=dylib=fmod");
}