use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterpStyle {
    Space,
    Line,
    Curve,
}

pub struct Brush {
    pub radius: f32,
    pub color: Color,
    pub interp: InterpStyle,
}

impl Brush {
    pub fn new(radius: f32, color: Color) -> Self {
        Self {
            radius,
            color,
            interp: InterpStyle::Curve,
        }
    }

    pub fn paint<D: RaylibDraw + RaylibBlendModeExt>(&self, d: &mut D, points: impl IntoIterator<Item = Vector2>, is_erasing: bool) {
        let (color, blend) = if is_erasing {
            (Color::BLANK, BlendMode::BLEND_CUSTOM_SEPARATE)
        } else {
            (self.color, BlendMode::BLEND_ALPHA)
        };
        let thick = 2.0*self.radius;
        let mut d = d.begin_blend_mode(blend);

        let mut pprev = None;
        let mut prev = None;
        for curr in points.into_iter() {
            if self.interp != InterpStyle::Space && let Some(prev) = prev {
                if self.interp == InterpStyle::Curve && let Some(pprev) = pprev {
                    let control = prev*2.0 - pprev*0.5 - curr*0.5;
                    d.draw_spline_bezier_quadratic(&[pprev, control, curr], thick, color);
                } else {
                    d.draw_line_ex(prev, curr, thick, color);
                }
            } else {
                if self.radius < 1.0 {
                    d.draw_pixel_v(curr, color);
                } else {
                    d.draw_circle_sector(curr, self.radius, 0.0, 360.0, 60, color);
                }
            }
            pprev = prev;
            prev = Some(curr);
        }
    }
}