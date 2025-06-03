use raylib::prelude::*;

pub struct Brush {
    radius: f32,
    pub is_erasing: bool,
    pub color: Color,
    center: f32,
    stamp: RenderTexture2D,
}

impl Brush {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, radius: f32, color: Color) -> Self {
        Self {
            radius,
            is_erasing: false,
            color,
            center: radius * 0.5 + 1.0,
            stamp: {
                let buffer_size = u32::try_from((radius * 2.0) as i32).unwrap() + 2;
                rl.load_render_texture(thread, buffer_size, buffer_size).unwrap()
            },
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, new_radius: f32) {
        if new_radius != self.radius {
            if new_radius > self.radius {
                let new_buffer_size = u32::try_from((new_radius * 2.0) as i32).unwrap() + 2;
                self.stamp = rl.load_render_texture(thread, new_buffer_size, new_buffer_size).unwrap();
            }
            self.radius = new_radius;
            self.center = new_radius + 1.0;
            let center_v = Vector2::new(self.center, self.center);
            {
                let mut d = rl.begin_texture_mode(thread, &mut self.stamp);
                d.clear_background(Color::BLANK);
                d.draw_circle_sector(center_v, self.radius, 0.0, 360.0, 360, Color::WHITE);
            }
        }
    }

    pub fn paint<D: RaylibDraw + RaylibBlendModeExt>(&self, d: &mut D, prev: Option<(Vector2, Option<Vector2>)>, curr: Vector2) {
        let (color, blend) = if self.is_erasing {
            (Color::BLANK, BlendMode::BLEND_CUSTOM_SEPARATE)
        } else {
            (self.color, BlendMode::BLEND_ALPHA)
        };
        let thick = 2.0*self.radius;
        let mut d = d.begin_blend_mode(blend);
        if let Some((prev, pprev)) = prev {
            if let Some(pprev) = pprev {
                let control = prev*2.0 - pprev*0.5 - curr*0.5;
                d.draw_spline_bezier_quadratic(&[pprev, control, curr], thick, color);
            } else {
                d.draw_line_ex(prev, curr, thick, color);
            }
        }
        d.draw_texture_ex(&self.stamp, curr - self.center, 0.0, 1.0, color);
    }
}