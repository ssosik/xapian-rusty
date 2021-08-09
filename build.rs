use cxx_build::CFG;
use std::env;
use std::path::Path;

fn main() {
    if cfg!(trybuild) {
        return;
    }

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let xapian_include_dir = Path::new(&manifest_dir).join("include");
    CFG.exported_header_dirs.push(&xapian_include_dir);

    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("xapian-bind.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-declarations")
        .static_flag(true)
        .include("xapian-core-1.4.17/include")
        .compile("xapian-rusty");

    println!("cargo:rustc-flags=-lz");
    println!("cargo:rustc-link-search=all=xapian-core-1.4.17/.libs");
    println!("cargo:rustc-link-lib=static=xapian");
    println!("cargo:rustc-link-lib=m");
}
