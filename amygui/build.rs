fn main() {
    println!("cargo::rerun-if-changed=binding/amygui.h");
    println!("cargo::rerun-if-changed=binding/amygui.c");

    cc::Build::new()
        .files(vec!["binding/amygui.c"])
        .include("binding")
        .include("raylib/src")
        .warnings(true)
        .extra_warnings(true)
        .compile("amygui");
}
