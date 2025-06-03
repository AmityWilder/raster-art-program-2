use std::collections::VecDeque;
use raylib::prelude::*;
use crate::{brush::Brush, draw_texture_custom, frame::Frame};

fn get_pixel(img: &mut Image, x: i32, y: i32) -> Option<Color> {
    if 0 <= x && x < img.width && 0 <= y && y < img.height {
        Some(img.get_color(x, y))
    } else {
        None
    }
}

/// ```not_rust
/// fn fill(x, y):
///     if not Inside(x, y) then return
///     let s = new empty queue or stack
///     Add (x, x, y, 1) to s
///     Add (x, x, y - 1, -1) to s
///     while s is not empty:
///         Remove an (x1, x2, y, dy) from s
///         let x = x1
///         if Inside(x, y):
///             while Inside(x - 1, y):
///                 Set(x - 1, y)
///                 x = x - 1
///             if x < x1:
///                 Add (x, x1 - 1, y - dy, -dy) to s
///         while x1 <= x2:
///             while Inside(x1, y):
///                 Set(x1, y)
///                 x1 = x1 + 1
///             if x1 > x:
///                 Add (x, x1 - 1, y + dy, dy) to s
///             if x1 - 1 > x2:
///                 Add (x2 + 1, x1 - 1, y - dy, -dy) to s
///             x1 = x1 + 1
///             while x1 < x2 and not Inside(x1, y):
///                 x1 = x1 + 1
///             x = x1
/// ```
fn flood_fill(img: &mut Image, x: i32, y: i32, new_color: Color) {
    // if not Inside(x, y) then return
    let Some(old_color) = get_pixel(img, x, y) else { return };
    if new_color == old_color { return; } // already filled
    // let s = new empty queue or stack
    let mut s = VecDeque::new();
    // Add (x, x, y, 1) to s
    s.push_back((x, x, y, 1));
    // Add (x, x, y - 1, -1) to s
    s.push_back((x, x, y - 1, -1));
    // while s is not empty:
    //     Remove an (x1, x2, y, dy) from s
    while let Some((mut x1, x2, y, dy)) = s.pop_front() {
        // let x = x1
        let mut x = x1;
        // if Inside(x, y):
        if get_pixel(img, x, y).is_some_and(|c| c == old_color) {
            // while Inside(x - 1, y):
            while get_pixel(img, x - 1, y).is_some_and(|c| c == old_color) {
                // Set(x - 1, y)
                img.draw_pixel(x - 1, y, new_color);
                // x = x - 1
                x -= 1;
            }
            // if x < x1:
            if x < x1 {
                // Add (x, x1 - 1, y - dy, -dy) to s
                s.push_back((x, x1 - 1, y - dy, -dy));
            }
        }
        // while x1 <= x2:
        while x1 <= x2 {
            // while Inside(x1, y):
            while get_pixel(img, x1, y).is_some_and(|c| c == old_color) {
                // Set(x1, y)
                img.draw_pixel(x1, y, new_color);
                // x1 = x1 + 1
                x1 += 1;
            }
            // if x1 > x:
            if x1 > x {
                // Add (x, x1 - 1, y + dy, dy) to s
                s.push_back((x, x1 - 1, y + dy, dy));
            }
            // if x1 - 1 > x2:
            if x1 - 1 > x2 {
                // Add (x2 + 1, x1 - 1, y - dy, -dy) to s
                s.push_back((x2 + 1, x1 - 1, y - dy, -dy));
            }
            // x1 = x1 + 1
            x1 += 1;
            // while x1 < x2 and not Inside(x1, y):
            while x1 < x2 && !get_pixel(img, x1, y).is_some_and(|c| c == old_color) {
                // x1 = x1 + 1
                x1 += 1;
            }
            // x = x1
            x = x1;
        }
    }
}

enum Tool {
    Pen {
        pen_pos_prev: Option<(Vector2, Option<Vector2>)>,
    },
    Fill,
}

pub struct ArtEditor {
    canvas: RenderTexture2D,
    is_canvas_dirty: bool,
    zoom_pow: f32,
    frac_pan: Vector2,
    tool: Tool,
}

impl ArtEditor {
    pub fn new(canvas: RenderTexture2D) -> Self {
        Self {
            canvas,
            is_canvas_dirty: true,
            zoom_pow: 0.0,
            frac_pan: Vector2::zero(),
            tool: Tool::Pen {
                pen_pos_prev: None,
            },
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_canvas_dirty = true;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, frame: &mut Frame) {
        let mouse_pos = rl.get_mouse_position();

        // Zoom + pan
        {
            let scroll = rl.get_mouse_wheel_move();

            let mut new_brush_radius = brush.radius();

            if scroll != 0.0 {
                if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) {
                    // zoom
                    self.zoom_pow = (self.zoom_pow + scroll).clamp(-4.0, 4.0);
                    self.is_canvas_dirty = true;
                } else if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                    // horizontal pan
                    self.frac_pan.x += scroll * 20.0;
                    self.is_canvas_dirty = true;
                } else if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT) {
                    // pen size
                    new_brush_radius = (new_brush_radius + scroll * 0.5).max(0.5);
                } else {
                    // vertical pan
                    self.frac_pan.y += scroll * 20.0;
                    self.is_canvas_dirty = true;
                }
            }

            // pen size
            const KEY_ZERO_U32: u32 = KeyboardKey::KEY_ZERO as u32;
            const KEY_ONE_U32: u32 = KeyboardKey::KEY_ONE as u32;
            const KEY_NINE_U32: u32 = KeyboardKey::KEY_NINE as u32;
            if let Some(key @ KEY_ONE_U32..=KEY_NINE_U32) = rl.get_key_pressed_number() {
                new_brush_radius = (key - KEY_ZERO_U32) as f32 * 0.5;
            }

            if brush.radius() != new_brush_radius {
                brush.set_radius(rl, thread, new_brush_radius);
                self.is_canvas_dirty = true;
            }
        }

        let zoom = 2.0f32.powi(self.zoom_pow as i32);
        let zoom_inv = zoom.recip();

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            let movement = rl.get_mouse_delta();
            if movement.length_sqr() > 0.0 {
                self.frac_pan = self.frac_pan + movement * zoom_inv;
                self.is_canvas_dirty = true;
            }
        }

        let pen_pos = Vector2 {
            x: (mouse_pos.x*zoom_inv - self.frac_pan.x).round(),
            y: (mouse_pos.y*zoom_inv - self.frac_pan.y).round(),
        };

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            self.tool = Tool::Fill;
        } else if rl.is_key_pressed(KeyboardKey::KEY_B) {
            if !matches!(self.tool, Tool::Pen { .. }) {
                self.tool = Tool::Pen { pen_pos_prev: None };
            }
        }

        // Paint
        match &mut self.tool {
            Tool::Pen { pen_pos_prev } => {
                if  rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) ||
                    rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT)
                {
                    *pen_pos_prev = None;
                }

                if let Some((pos_prev, pos_pprev)) = &mut *pen_pos_prev && pos_prev != &pen_pos {
                    let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                    let pos_pprev = std::mem::replace(pos_pprev, Some(*pos_prev));
                    let pos_prev = std::mem::replace(pos_prev, pen_pos);
                    brush.paint(&mut d, Some((pos_prev, pos_pprev)), pen_pos);
                    self.is_canvas_dirty = true;
                }

                if pen_pos_prev.is_none() {
                    let is_right_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) || is_right_pressed {
                        *pen_pos_prev = Some((pen_pos, None));
                        brush.is_erasing = is_right_pressed;
                        let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                        brush.paint(&mut d, None, pen_pos);
                        self.is_canvas_dirty = true;
                    }
                }
            }

            Tool::Fill => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let mut img = self.canvas.load_image().unwrap();
                    flood_fill(&mut img, pen_pos.x as i32, pen_pos.y as i32, brush.color);
                    let len = get_pixel_data_size(img.width, img.height, img.format()).try_into().unwrap();
                    let pixels = unsafe { std::slice::from_raw_parts(img.data.cast(), len) };
                    self.canvas.update_texture(pixels).unwrap();
                    self.is_canvas_dirty = true;
                }
            }
        }

        // Render
        if self.is_canvas_dirty {
            let pan = Vector2 {
                x: self.frac_pan.x.round(),
                y: self.frac_pan.y.round(),
            };

            let canvas_rec = Rectangle {
                x: pan.x * zoom,
                y: pan.y * zoom,
                width:  self.canvas.texture.width  as f32 * zoom,
                height: self.canvas.texture.height as f32 * zoom,
            };

            {
                let mut d = frame.begin_drawing(rl, thread);
                d.clear_background(Color::BLACK);
                d.draw_rectangle_rec(canvas_rec, Color::new(42, 42, 42, 255));

                draw_texture_custom(&mut d, &self.canvas, &canvas_rec, Color::WHITE);
            }
            self.is_canvas_dirty = false;
        }
    }
}
