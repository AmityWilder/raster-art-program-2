use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=binding/color_wheel.c");
    cc::Build::new()
        .files(vec!["binding/color_wheel.c"])
        .include("binding")
        .include("../target/debug/build/raylib-sys-fe42d3c57842a6a3/out/include") // HACK: extremely questionable
        .warnings(true)
        // .flag("-std=c99")
        .extra_warnings(true)
        .compile("amygui");
}
