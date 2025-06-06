fn main() {
    println!("cargo::rerun-if-changed=binding/amygui.h");
    println!("cargo::rerun-if-changed=binding/color_wheel.c");

    cc::Build::new()
        .files(vec!["binding/color_wheel.c"])
        .include("binding")
        .include("raylib/src")
        // .include("raylib/examples/shapes/raygui.h")
        .warnings(true)
        // .flag("-std=c99")
        .extra_warnings(true)
        .compile("amygui");
}
