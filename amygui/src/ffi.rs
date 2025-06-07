#![allow(non_snake_case)]

use std::ffi::*;
use raylib::ffi::*;

unsafe extern "C" {
    pub fn GuiColorPickerHSVWheel(
        center: Vector2,
        triangleRadius: c_float,
        previewRadius: c_float,
        wheelInnerRadius: c_float,
        wheelOuterRadius: c_float,
        wheelSegments: c_int,
        hsv: Vector3,
    ) -> Vector3;

    // Draw texture 1:1 within rec
    pub fn DrawTextureDirect(texture: Texture, rec: Rectangle);
}
