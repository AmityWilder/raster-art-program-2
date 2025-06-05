#![allow(non_snake_case)]

use raylib::prelude::*;

pub mod ffi {
    use raylib::ffi::*;

    unsafe extern "C" {
        pub fn GuiColorPickerHSVWheel(bounds: Rectangle, colorHSV: Vector3) -> Vector3;
    }
}

pub trait RaylibDrawAmyGUI {
    /// Draw HSV color picker wheel, returns updated color
    #[inline]
    fn gui_color_picker_hsv_wheel(&mut self, bounds: Rectangle, color_hsv: Vector3) -> Vector3 {
        unsafe { ffi::GuiColorPickerHSVWheel(bounds.into(), color_hsv.into()).into() }
    }
}

impl<D: RaylibDrawGui> RaylibDrawAmyGUI for D {}

pub mod prelude {
    pub use crate::RaylibDrawAmyGUI;
}
