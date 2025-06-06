#![allow(non_snake_case)]

use raylib::prelude::*;

pub mod ffi {
    use std::ffi::*;
    use raylib::ffi::*;

    unsafe extern "C" {
        pub fn GuiColorPickerHSVWheel(
            bounds: Rectangle,
            triangleInnerSep: c_float,
            previewRadius: c_float,
            wheelThick: c_float,
            wheelSegments: c_int,
            hsv: Vector3,
        ) -> Vector3;
    }
}

pub trait RaylibDrawAmyGUI {
    /// Draw HSV color picker wheel, returns updated color in HSV
    #[inline]
    fn gui_color_picker_hsv_wheel(&mut self, bounds: Rectangle, triangle_inner_sep: f32, preview_radius: f32, wheel_thick: f32, wheel_segments: i32, hsv: Vector3) -> Vector3 {
        assert!(wheel_segments > 0, "must have at least one wheel segment");
        unsafe { ffi::GuiColorPickerHSVWheel(bounds.into(), triangle_inner_sep, preview_radius, wheel_thick, wheel_segments, hsv.into()).into() }
    }
}

impl<D: RaylibDrawGui> RaylibDrawAmyGUI for D {}

pub mod prelude {
    pub use crate::RaylibDrawAmyGUI;
}
