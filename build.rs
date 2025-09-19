use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-search=native=fmod/macos");
            println!("cargo:rustc-link-lib=dylib=fmod");
            // Set rpath for macOS to find the library relative to the executable
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../../fmod/macos");
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../../fmod/macos");
        }
        "linux" => {
            println!("cargo:rustc-link-search=native=fmod/linux");
            println!("cargo:rustc-link-lib=dylib=fmod");
            // Set rpath for Linux
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../fmod/linux");
        }
        "windows" => {
            println!("cargo:rustc-link-search=native=fmod/windows");
            println!("cargo:rustc-link-lib=dylib=fmod");

            // Copy DLL to output directory for Windows
            // OUT_DIR is something like: target/debug/build/driftwave-xxx/out
            // We need to get to: target/debug/
            let out_path = Path::new(&out_dir);
            let target_dir = out_path
                .parent() // target/debug/build/driftwave-xxx
                .and_then(|p| p.parent()) // target/debug/build
                .and_then(|p| p.parent()) // target/debug
                .unwrap();

            let dll_src = Path::new("fmod/windows/fmod.dll");
            let dll_dst = target_dir.join("fmod.dll");

            if dll_src.exists() {
                fs::copy(&dll_src, &dll_dst).unwrap();
                println!("cargo:rerun-if-changed=fmod/windows/fmod.dll");
            }
        }
        _ => panic!("Unsupported OS"),
    }
}
