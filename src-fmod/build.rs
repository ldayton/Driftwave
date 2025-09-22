use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    match target_os.as_str() {
        "macos" => {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let fmod_dir = Path::new(&manifest_dir).join("fmod").join("macos");

            println!("cargo:rustc-link-search=native={}", fmod_dir.display());
            println!("cargo:rustc-link-lib=dylib=fmod");

            // Copy dylib to output directory for macOS
            let out_path = Path::new(&out_dir);
            let target_dir = out_path
                .parent() // target/debug/build/driftwave-xxx
                .and_then(|p| p.parent()) // target/debug/build
                .and_then(|p| p.parent()) // target/debug
                .unwrap();

            let dylib_src = fmod_dir.join("libfmod.dylib");
            let dylib_dst = target_dir.join("libfmod.dylib");

            if dylib_src.exists() {
                fs::copy(&dylib_src, &dylib_dst).unwrap();
                println!("cargo:rerun-if-changed={}", dylib_src.display());
            }

            // Set rpath for macOS to find the library relative to the executable
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        }
        "linux" => {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let fmod_dir = Path::new(&manifest_dir).join("fmod").join("linux");

            println!("cargo:rustc-link-search=native={}", fmod_dir.display());
            println!("cargo:rustc-link-lib=dylib=fmod");
            // Set rpath for Linux
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../fmod/linux");
        }
        "windows" => {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let fmod_dir = Path::new(&manifest_dir).join("fmod").join("windows");

            println!("cargo:rustc-link-search=native={}", fmod_dir.display());
            println!("cargo:rustc-link-lib=dylib=fmod_vc");

            // Copy DLL to output directory for Windows
            // OUT_DIR is something like: target/debug/build/driftwave-xxx/out
            // We need to get to: target/debug/
            let out_path = Path::new(&out_dir);
            let target_dir = out_path
                .parent() // target/debug/build/driftwave-xxx
                .and_then(|p| p.parent()) // target/debug/build
                .and_then(|p| p.parent()) // target/debug
                .unwrap();

            let dll_src = fmod_dir.join("fmod.dll");
            let dll_dst = target_dir.join("fmod.dll");

            if dll_src.exists() {
                fs::copy(&dll_src, dll_dst).unwrap();
                println!("cargo:rerun-if-changed={}", dll_src.display());
            }
        }
        _ => panic!("Unsupported OS"),
    }
}
