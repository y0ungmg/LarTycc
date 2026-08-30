use std::env;

fn main() {
    let destination = cmake::Config::new("..")
        .define("LARTYCC_BUILD_TESTS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        destination.display()
    );
    println!("cargo:rustc-link-lib=static=lartycc_audio_c");
    println!("cargo:rustc-link-lib=static=lartycc_audio_device");
    println!("cargo:rustc-link-lib=static=lartycc_audio");
    println!("cargo:rustc-link-lib=static=miniaudio");

    let target = env::var("CARGO_CFG_TARGET_OS").expect("Cargo sets CARGO_CFG_TARGET_OS");
    match target.as_str() {
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=m");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=uuid");
            println!("cargo:rustc-link-lib=dylib=winmm");
        }
        _ => {}
    }

    println!("cargo:rerun-if-changed=../CMakeLists.txt");
    println!("cargo:rerun-if-changed=../audio-engine/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../audio-engine/include");
    println!("cargo:rerun-if-changed=../audio-engine/src");
}
