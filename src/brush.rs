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
                let radius = u32::try_from(radius as i32).unwrap();
                let buffer_size = radius * 2 + 2;
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
                let new_radius = u32::try_from(new_radius as i32).unwrap();
                let new_buffer_size = new_radius * 2 + 2;
                self.stamp = rl.load_render_texture(thread, new_buffer_size, new_buffer_size).unwrap();
            }
            self.radius = new_radius;
            self.center = new_radius * 0.5 + 1.0;
            let center_v = Vector2::new(self.center, self.center);
            {
                let mut d = rl.begin_texture_mode(thread, &mut self.stamp);
                d.clear_background(Color::BLANK);
                d.draw_circle_sector(center_v, self.radius, 0.0, 360.0, 360, Color::WHITE);
            }
        }
    }

    pub fn draw_line<D: RaylibDraw + RaylibBlendModeExt>(&self, d: &mut D, start: Vector2, end: Vector2) {
        let (color, blend) = if self.is_erasing {
            (Color::BLANK, BlendMode::BLEND_CUSTOM_SEPARATE)
        } else {
            (self.color, BlendMode::BLEND_ALPHA)
        };
        let mut d = d.begin_blend_mode(blend);
        d.draw_line_ex(start, end, 2.0*self.radius, color);
        d.draw_circle_v(end, self.radius, color);
    }

    pub fn draw<D: RaylibDraw + RaylibBlendModeExt>(&self, d: &mut D, position: Vector2) {
        let (color, blend) = if self.is_erasing {
            (Color::BLANK, BlendMode::BLEND_CUSTOM_SEPARATE)
        } else {
            (self.color, BlendMode::BLEND_ALPHA)
        };
        let mut d = d.begin_blend_mode(blend);
        d.draw_circle_v(position, self.radius, color);
    }
}