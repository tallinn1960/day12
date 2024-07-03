fn main() {
    let dst = cmake::Config::new("src/cpp")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=day12cpp");
    println!("cargo:rustc-link-lib=dylib=tbb");
    let dst = cmake::Config::new("swift")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .generator("Ninja")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=day12swift");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=src/cpp/lib.cpp");
    println!("cargo:rerun-if-changed=swift/CMakeLists.txt");
    println!("cargo:rerun-if-changed=swift/Sources/Day12/Day12.swift");
}