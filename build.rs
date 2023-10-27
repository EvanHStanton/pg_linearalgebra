fn main() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/openblas/lib");
    println!("cargo:rustc-link-lib=dylib=openblas");
    println!("cargo:rustc-link-lib=dylib=lapack");
}

