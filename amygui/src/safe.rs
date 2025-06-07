use raylib::prelude::*;

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
            crate::ffi::GuiColorPickerHSVWheel(
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

    #[inline]
    fn draw_texture_direct(&mut self, texture: impl AsRef<ffi::Texture>, rec: Rectangle) {
        unsafe {
            crate::ffi::DrawTextureDirect(*texture.as_ref(), rec.into())
        }
    }
}

impl<D: RaylibDrawGui> RaylibDrawAmyGUI for D {}
