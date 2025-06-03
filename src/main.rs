use raylib::prelude::*;
use MouseButton::*;
use BlendMode::*;
use KeyboardKey::*;

use std::fmt::Write;

pub struct Brush {
    pub radius: f32,
    pub color: Color,
}

fn draw_texture_custom(_d: &mut impl RaylibDraw, texture: impl AsRef<ffi::Texture>, rec: &Rectangle) {
    use ffi::*;

    unsafe {
        let left = rec.x;
        let right = left + rec.width;
        let top = rec.y;
        let bottom = top + rec.height;

        rlSetTexture(texture.as_ref().id);
        rlBegin(RL_QUADS as i32);
        {
            rlColor4ub(255, 255, 255, 255); // Tint
            rlNormal3f(0.0, 0.0, 1.0);

            // Top left
            rlTexCoord2f(0.0, 1.0);
            rlVertex2f(left, top);

            // Bottom left
            rlTexCoord2f(0.0, 0.0);
            rlVertex2f(left, bottom);

            // Bottom right
            rlTexCoord2f(1.0, 0.0);
            rlVertex2f(right, bottom);

            // Top right
            rlTexCoord2f(1.0, 1.0);
            rlVertex2f(right, top);
        }
        rlEnd();
        rlSetTexture(0);
    }
}

struct ArtEditor {
    canvas: RenderTexture2D,
    pen_pos_prev: Option<Vector2>,
    is_erasing: bool,

    zoom_pow: f32,
    frac_pan: Vector2,

    is_redraw_needed: bool,

    last_redraw_str: String,
}

impl ArtEditor {
    fn new(canvas: RenderTexture2D) -> Self {
        Self {
            canvas,
            pen_pos_prev: None,
            is_erasing: false,
            zoom_pow: 0.0,
            frac_pan: Vector2::zero(),
            is_redraw_needed: true,
            last_redraw_str: "last redraw: 0.0000000".to_string(),
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush) {
        rl.poll_input_events();

        if rl.is_window_resized() {
            self.is_redraw_needed = true;
        }

        let mouse_pos = rl.get_mouse_position();

        // Zoom + pan
        {
            let scroll = rl.get_mouse_wheel_move();

            if scroll != 0.0 {
                if rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_RIGHT_CONTROL) {
                    // zoom
                    self.zoom_pow = (self.zoom_pow + scroll).clamp(-4.0, 4.0);
                    self.is_redraw_needed = true;
                } else if rl.is_key_down(KEY_LEFT_SHIFT) || rl.is_key_down(KEY_RIGHT_SHIFT) {
                    // horizontal pan
                    self.frac_pan.x += scroll * 20.0;
                    self.is_redraw_needed = true;
                } else if rl.is_key_down(KEY_LEFT_ALT) || rl.is_key_down(KEY_RIGHT_ALT) {
                    // pen size
                    brush.radius = (brush.radius + scroll * 0.5).max(0.5);
                } else {
                    // vertical pan
                    self.frac_pan.y += scroll * 20.0;
                    self.is_redraw_needed = true;
                }
            }

            // pen size
            const KEY_ZERO_U32: u32 = KEY_ZERO as u32;
            const KEY_ONE_U32: u32 = KEY_ONE as u32;
            const KEY_NINE_U32: u32 = KEY_NINE as u32;
            if let Some(key @ KEY_ONE_U32..=KEY_NINE_U32) = rl.get_key_pressed_number() {
                brush.radius = (key - KEY_ZERO_U32) as f32 * 0.5;
                self.is_redraw_needed = true;
            }
        }

        let zoom = 2.0f32.powi(self.zoom_pow as i32);
        let zoom_inv = zoom.recip();

        if rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE) {
            let movement = rl.get_mouse_delta();
            if movement.length_sqr() > 0.0 {
                self.frac_pan = self.frac_pan + movement * zoom_inv;
                self.is_redraw_needed = true;
            }
        }

        let pen_pos = Vector2 {
            x: (mouse_pos.x*zoom_inv - self.frac_pan.x).round(),
            y: (mouse_pos.y*zoom_inv - self.frac_pan.y).round(),
        };

        // Draw
        {
            if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                self.pen_pos_prev = None;
            }

            if !self.is_erasing && let Some(pos_prev) = &mut self.pen_pos_prev && pos_prev != &pen_pos {
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                    d.draw_line_ex(*pos_prev, pen_pos, 2.0*brush.radius, brush.color);
                    d.draw_circle_v(pen_pos, brush.radius, brush.color);
                }
                *pos_prev = pen_pos;
                self.is_redraw_needed = true;
            }

            if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
                self.pen_pos_prev = Some(pen_pos);
                self.is_erasing = false;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                    d.draw_circle_v(pen_pos, brush.radius, brush.color);
                }
                self.is_redraw_needed = true;
            }
        }

        // Erase
        {
            if rl.is_mouse_button_released(MOUSE_BUTTON_RIGHT) {
                self.pen_pos_prev = None;
            }

            if self.is_erasing && let Some(pos_prev) = &mut self.pen_pos_prev && pos_prev != &pen_pos {
                let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                {
                    let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                    d.draw_line_ex(*pos_prev, pen_pos, 2.0*brush.radius, Color::BLANK);
                    d.draw_circle_v(pen_pos, brush.radius, Color::BLANK);
                }
                *pos_prev = pen_pos;
                self.is_redraw_needed = true;
            }

            if rl.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT) {
                self.pen_pos_prev = Some(pen_pos);
                self.is_erasing = true;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut self.canvas);
                    {
                        let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                        d.draw_circle_v(pen_pos, brush.radius, Color::BLANK);
                    }
                }
                self.is_redraw_needed = true;
            }
        }

        // Draw frame
        if self.is_redraw_needed {
            self.last_redraw_str.truncate(const { "last redraw: ".len() });
            _ = write!(self.last_redraw_str, "{}", rl.get_time());

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
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_rectangle_rec(canvas_rec, Color::new(42, 42, 42, 255));

                draw_texture_custom(&mut d, &self.canvas, &canvas_rec);
                // d.draw_ring(mouse_pos, brush.radius*zoom, brush.radius*zoom + 1.0, 0.0, 360.0, 10, Color::GRAY);

                d.draw_rectangle(0, 0, 20, 20, Color::new(20, 20, 20, 255));
                d.draw_circle_v(Vector2::new(10.0, 10.0), brush.radius, Color::GRAY);
            }

            rl.swap_screen_buffer();
            self.is_redraw_needed = false;
        }
    }
}

struct ColorEditor {
    camera: Camera3D
}

impl ColorEditor {
    fn new(position: Vector3, fovy: f32) -> Self {
        Self {
            camera: Camera3D::orthographic(position, Vector3::zero(), Vector3::up(), fovy),
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush) {
        rl.poll_input_events();

        {
            {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                {
                    let mut d = d.begin_mode3D(self.camera);

                    d.draw_cube_v(Vector3::zero(), Vector3::new(25.0, 25.0, 25.0), Color::GRAY);
                }
            }

            rl.swap_screen_buffer();
        }
    }
}

enum Editor {
    Art,
    Color,
}

fn main() {
    let (mut rl, thread) = init()
        .title("Amity Raster Art")
        .resizable()
        .build();

    rl.maximize_window();

    unsafe {
        ffi::EnableEventWaiting();
    }

    let mut brush = Brush {
        radius: 0.5,
        color: Color::WHITE,
    };

    let mut art_editor = ArtEditor::new(rl.load_render_texture(&thread, 720, 480).unwrap());
    let mut color_editor = ColorEditor::new(Vector3::new(50.0, 50.0, 50.0), 45.0);
    let mut current_editor = Editor::Art;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KEY_C) {
            current_editor = match current_editor {
                Editor::Art => Editor::Color,
                Editor::Color => Editor::Art,
            };
        }
        match current_editor {
            Editor::Art => art_editor.update(&mut rl, &thread, &mut brush),
            Editor::Color => color_editor.update(&mut rl, &thread, &mut brush),
        }
    }
}
