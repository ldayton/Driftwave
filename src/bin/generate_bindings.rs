use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("fmod/headers/fmod.h")
        .header("fmod/headers/fmod_common.h")
        .header("fmod/headers/fmod_errors.h")
        // Only generate bindings for FMOD functions/types
        .allowlist_function("FMOD_.*")
        .allowlist_type("FMOD_.*")
        .allowlist_var("FMOD_.*")
        // Use core::ffi instead of std::os::raw
        .ctypes_prefix("::core::ffi")
        // Generate rust code
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/ffi/fmod_sys.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");

    println!("Generated bindings to src/ffi/fmod_sys.rs");
    println!("Run with: cargo run --bin generate_bindings");
}
