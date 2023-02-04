use std::{env, path::PathBuf};

fn main() {
    println!("call build.rs");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    env::set_var("_M_IX86", "1");

    cc::Build::new()
        .file("rewolf-wow64ext\\src\\wow64ext.cpp")
        .include("rewolf-wow64ext\\src\\")
        .cpp(true)
        .out_dir(out_path.clone())
        .compile("wow64ext");
    println!("cargo:rustc-link-lib=wow64ext");

    bindgen::Builder::default()
        .header("src\\wrapper.hpp")
        .allowlist_function("X64Call")
        .allowlist_function("GetModuleHandle64")
        .allowlist_function("getNTDLL64")
        .allowlist_function("GetProcAddress64")
        .allowlist_function("VirtualQueryEx64")
        .allowlist_function("VirtualAllocEx64")
        .allowlist_function("VirtualFreeEx64")
        .allowlist_function("VirtualProtectEx64")
        .allowlist_function("ReadProcessMemory64")
        .allowlist_function("WriteProcessMemory64")
        .allowlist_function("GetThreadContext64")
        .allowlist_function("SetThreadContext64")
        .allowlist_function("SetLastErrorFromX64Call")
        .layout_tests(false)
        .derive_debug(true)
        .derive_default(true)
        .generate()
        .expect("unable to generate bindings")
        .write_to_file("src\\bindings.rs")
        .expect("couldn't write bindings!");
}
