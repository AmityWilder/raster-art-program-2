use raylib::prelude::*;
use crate::{brush::Brush, draw_texture_custom, frame::Frame};

pub struct ColorEditor {
    hsv_tex: RenderTexture2D,
}

impl ColorEditor {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut hsv_tex = rl.load_render_texture(thread, 360, 255).unwrap();
        {
            let mut d = rl.begin_texture_mode(thread, &mut hsv_tex);
            for hue in 0..360 {
                let color = Color::color_from_hsv(hue as f32, 1.0, 1.0);
                d.draw_rectangle_gradient_v(hue, 0, 1, 127, Color::WHITE, color);
                d.draw_rectangle_gradient_v(hue, 127, 1, 127, color, Color::BLACK);
            }
        }

        Self {
            hsv_tex,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, frame: &mut Frame) {
        let mut d = frame.begin_drawing(rl, thread);
        let rec = Rectangle { x: 0.0, y: 0.0, width: 360.0, height: 255.0 };
        draw_texture_custom(&mut d, &self.hsv_tex, &rec);
    }
}