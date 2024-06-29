fn main() {
    let dst = cmake::Config::new("src/cpp")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=day12cpp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/lib.cpp");
}