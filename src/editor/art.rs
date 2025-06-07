use crate::{brush::Brush, editor::Editor, frame::Frame};
use raylib::prelude::*;
use amygui::prelude::*;

mod flood_fill;
use flood_fill::flood_fill;

enum Tool {
    Pen {
        pen_pos_prev: Option<(Vector2, Option<Vector2>)>,
    },
    Fill,
}

pub struct ArtEditor {
    canvas: RenderTexture2D,
    is_canvas_dirty: bool,
    zoom_pow: i32,
    pan: Vector2,
    tool: Tool,
    is_erasing: bool,
    is_drag_panning: bool,
}

impl ArtEditor {
    pub const fn new(canvas: RenderTexture2D) -> Self {
        Self {
            canvas,
            is_canvas_dirty: true,
            zoom_pow: 0,
            pan: Vector2::zero(),
            tool: Tool::Pen {
                pen_pos_prev: None,
            },
            is_erasing: false,
            is_drag_panning: false,
        }
    }

    pub const fn set_pan(&mut self, pan: Vector2) {
        self.pan = pan;
    }
}

impl Editor for ArtEditor {
    #[inline]
    fn mark_dirty(&mut self) {
        self.is_canvas_dirty = true;
    }

    #[inline]
    fn is_dirty(&self) -> bool {
        self.is_canvas_dirty
    }

    #[inline]
    fn is_focused(&self) -> bool {
        self.is_drag_panning || matches!(self.tool, Tool::Pen { pen_pos_prev: Some(_) })
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, viewport: Rectangle, frame: &mut Frame, is_awake: bool) {
        let mouse_pos = rl.get_mouse_position();

        // Zoom + pan
        if is_awake {
            let scroll = rl.get_mouse_wheel_move();

            let mut new_brush_radius = brush.radius;

            if scroll != 0.0 {
                if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) {
                    // zoom
                    let zoom_pow_old = self.zoom_pow;
                    self.zoom_pow = (self.zoom_pow + scroll.round() as i32).clamp(-4, 4);
                    self.pan = self.pan + mouse_pos*(2.0f32.powi(-self.zoom_pow) - 2.0f32.powi(-zoom_pow_old));
                    self.is_canvas_dirty = true;
                } else if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                    // horizontal pan
                    self.pan.x += scroll * 20.0;
                    self.is_canvas_dirty = true;
                } else if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT) {
                    // pen size
                    new_brush_radius = (new_brush_radius + scroll * 0.5).max(0.5);
                } else {
                    // vertical pan
                    self.pan.y += scroll * 20.0;
                    self.is_canvas_dirty = true;
                }
            }

            // pen size
            if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                new_brush_radius = 0.5;
            } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                new_brush_radius = 1.0;
            } else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
                new_brush_radius = 2.0;
            } else if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
                new_brush_radius = 3.0;
            } else if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
                new_brush_radius = 4.0;
            }

            if brush.radius != new_brush_radius {
                brush.radius = new_brush_radius;
                self.is_canvas_dirty = true;
            }
        }

        let zoom = 2.0f32.powi(self.zoom_pow);
        let zoom_inv = zoom.recip();

        if is_awake {
            {
                let is_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_MIDDLE);
                if is_pressed || rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_MIDDLE) {
                    self.is_drag_panning = is_pressed;
                }
            }

            if self.is_drag_panning {
                let movement = rl.get_mouse_delta();
                if movement.length_sqr() > 0.0 {
                    self.pan = self.pan + movement * zoom_inv;
                    self.is_canvas_dirty = true;
                }
            }
        }

        let pen_pos = Vector2 {
            x: (mouse_pos.x*zoom_inv - self.pan.x).floor(),
            y: (mouse_pos.y*zoom_inv - self.pan.y).floor(),
        };

        if is_awake {
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
                    if let Some((pos_prev, pos_pprev)) = &mut *pen_pos_prev && pos_prev != &pen_pos {
                        let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                        let pos_pprev = std::mem::replace(pos_pprev, Some(*pos_prev));
                        let pos_prev = std::mem::replace(pos_prev, pen_pos);
                        brush.paint(&mut d, pos_pprev.into_iter().chain([pos_prev, pen_pos].into_iter()), self.is_erasing);
                        self.is_canvas_dirty = true;
                    }

                    if  rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) ||
                        rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT)
                    {
                        *pen_pos_prev = None;
                    }

                    if pen_pos_prev.is_none() {
                        let is_right_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);
                        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) || is_right_pressed {
                            *pen_pos_prev = Some((pen_pos, None));
                            self.is_erasing = is_right_pressed;
                            let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                            brush.paint(&mut d, [pen_pos], self.is_erasing);
                            self.is_canvas_dirty = true;
                        }
                    }
                }

                Tool::Fill => {
                    let is_erasing = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);
                    if is_erasing || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        let mut img = self.canvas.load_image().unwrap();
                        let (x, y) = (pen_pos.x as i32, img.height - pen_pos.y as i32);
                        flood_fill(&mut img, x, y, if is_erasing { Color::BLANK } else { brush.color });
                        let len = get_pixel_data_size(img.width, img.height, img.format()).try_into().unwrap();
                        let pixels = unsafe { std::slice::from_raw_parts(img.data.cast(), len) };
                        self.canvas.update_texture(pixels).unwrap();
                        self.is_canvas_dirty = true;
                    }
                }
            }
        }

        // Render
        if self.is_canvas_dirty {
            let pan = Vector2 {
                x: self.pan.x.round(),
                y: self.pan.y.round(),
            };

            let canvas_rec = Rectangle {
                x: pan.x * zoom,
                y: pan.y * zoom,
                width:  self.canvas.texture.width  as f32 * zoom,
                height: self.canvas.texture.height as f32 * zoom,
            };

            {
                let mut d = frame.begin_drawing(rl, thread);
                let mut d = d.begin_scissor_mode(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
                d.clear_background(Color::BLACK);
                d.draw_rectangle_rec(canvas_rec, Color::new(42, 42, 42, 255));

                d.draw_texture_direct(&self.canvas, canvas_rec);
            }
            self.is_canvas_dirty = false;
        }
    }
}
