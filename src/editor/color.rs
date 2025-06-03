use raylib::prelude::*;
use crate::{brush::Brush, draw_texture_custom, frame::Frame};

const FRAC_1_255: f32 = 1.0/255.0;

pub struct ColorEditor {
    hsv_tex: RenderTexture2D,
    is_color_dirty: bool,
    hue: f32,
    sat: f32,
    lum: f32,
}

impl ColorEditor {
    fn generate_tex<D: RaylibDraw>(d: &mut D, sat: f32) {
        for hue in 0..360 {
            for value in 0..255 {
                let color = Color::color_from_hsv(hue as f32, sat, value as f32*FRAC_1_255);
                d.draw_pixel(hue, 255 - value, color);
            }
        }
    }

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, brush: &Brush) -> Self {
        let Vector3 { x: hue, y: sat, z: lum } = brush.color.color_to_hsv();

        let mut hsv_tex = rl.load_render_texture(thread, 360, 255).unwrap();
        {
            let mut d = rl.begin_texture_mode(thread, &mut hsv_tex);
            Self::generate_tex(&mut d, sat);
        }

        Self {
            hsv_tex,
            is_color_dirty: true,
            hue,
            sat,
            lum,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, frame: &mut Frame) {
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let Vector2 { mut x, mut y } = mouse_pos;
            x = x.clamp(0.0, 360.0);
            y = (1.0 - y*FRAC_1_255).clamp(0.0, 1.0);
            if x != self.hue || y != self.lum {
                self.is_color_dirty = true;
                self.hue = x;
                self.lum = y;
            }
        }

        let scroll = rl.get_mouse_wheel_move();
        if scroll != 0.0 {
            self.is_color_dirty = true;
            self.sat = (self.sat + scroll*FRAC_1_255).clamp(0.0, 1.0);
            {
                let mut d = rl.begin_texture_mode(thread, &mut self.hsv_tex);
                Self::generate_tex(&mut d, self.sat);
            }
        }

        if self.is_color_dirty {
            brush.color = Color::color_from_hsv(self.hue, self.sat, self.lum);

            let mut d = frame.begin_drawing(rl, thread);
            let rec = Rectangle { x: 0.0, y: 0.0, width: 360.0, height: 255.0 };
            draw_texture_custom(&mut d, &self.hsv_tex, &rec, Color::WHITE);
            let rec = Rectangle { x: 365.0, y: 20.0, width: 32.0, height: 32.0 };
            d.draw_rectangle_rec(rec, brush.color);
        }
    }
}