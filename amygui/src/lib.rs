#![allow(non_snake_case)]

use raylib::prelude::*;

pub mod ffi {
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
    }
}

pub trait RaylibDrawAmyGUI {
    /// Draw HSV color picker wheel, returns updated color in HSV
    /// NOTES:
    /// - triangle radius is circumscribed
    /// - Color data should be passed normalized
    #[inline]
    fn gui_color_picker_hsv_wheel(
        &mut self,
        center: Vector2,
        triangle_radius: f32,
        preview_radius: f32,
        wheel_inner_radius: f32,
        wheel_outer_radius: f32,
        wheel_segments: i32,
        hsv: Vector3,
    ) -> Vector3 {
        unsafe {
            ffi::GuiColorPickerHSVWheel(
                center.into(),
                triangle_radius,
                preview_radius,
                wheel_inner_radius,
                wheel_outer_radius,
                wheel_segments,
                hsv.into(),
            ).into()
        }
    }
}

impl<D: RaylibDrawGui> RaylibDrawAmyGUI for D {}

pub mod prelude {
    pub use crate::RaylibDrawAmyGUI;
}
