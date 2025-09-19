use std::fs;
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
    bindings.write_to_file(&out_path).expect("Couldn't write bindings!");

    // Add #![allow(warnings)] after the header comment
    let content = fs::read_to_string(&out_path).expect("Failed to read generated file");
    let lines: Vec<&str> = content.lines().collect();

    let mut new_content = String::new();
    if let Some(first_line) = lines.first() {
        new_content.push_str(first_line);
        new_content.push_str("\n\n#![allow(warnings)]\n");
        for line in lines.iter().skip(1) {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    fs::write(out_path, new_content).expect("Failed to write modified file");

    println!("Generated bindings to src/ffi/fmod_sys.rs");
    println!("Run with: cargo run --bin generate_bindings");
}
