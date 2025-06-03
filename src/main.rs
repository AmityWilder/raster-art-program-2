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

fn main() {
    let (mut rl, thread) = init()
        .title("Amity Raster Art")
        .resizable()
        .build();

    rl.maximize_window();

    unsafe {
        ffi::EnableEventWaiting();
    }

    let mut canvas = rl.load_render_texture(&thread, 720, 480).unwrap();
    let mut pen_pos_prev = None;
    let mut is_erasing = false;

    let mut brush_radius = 0.5;
    let mut brush_color = Color::WHITE;

    let mut zoom_pow = 0.0;
    let mut frac_pan = Vector2::zero();

    let mut is_redraw_needed = true;

    let mut last_draw  = "last draw: 0.0000000".to_string();
    let mut last_clear = "last clear: 0.0000000".to_string();

    while !rl.window_should_close() {
        rl.poll_input_events();

        if rl.is_window_resized() {
            is_redraw_needed = true;
        }

        let mouse_pos = rl.get_mouse_position();

        // Zoom + pan
        {
            let scroll = rl.get_mouse_wheel_move();

            if scroll != 0.0 {
                if rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_RIGHT_CONTROL) {
                    // zoom
                    zoom_pow = (zoom_pow + scroll).clamp(-4.0, 4.0);
                    is_redraw_needed = true;
                } else if rl.is_key_down(KEY_LEFT_SHIFT) || rl.is_key_down(KEY_RIGHT_SHIFT) {
                    // horizontal pan
                    frac_pan.x += scroll * 20.0;
                    is_redraw_needed = true;
                } else if rl.is_key_down(KEY_LEFT_ALT) || rl.is_key_down(KEY_RIGHT_ALT) {
                    // pen size
                    brush_radius = (brush_radius + scroll * 0.5).max(0.5);
                } else {
                    // vertical pan
                    frac_pan.y += scroll * 20.0;
                    is_redraw_needed = true;
                }
            }

            // pen size
            if let Some(key) = rl.get_key_pressed_number() {
                let n = key - KEY_ZERO as u32;
                if n <= 9 {
                    brush_radius = n as f32 - 0.5;
                    is_redraw_needed = true;
                }
            }
        }

        let zoom = 2.0f32.powi(zoom_pow as i32);
        let zoom_inv = zoom.recip();

        if rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE) {
            let movement = rl.get_mouse_delta();
            if movement.length_sqr() > 0.0 {
                frac_pan = frac_pan + movement * zoom_inv;
                is_redraw_needed = true;
            }
        }

        let pen_pos = Vector2 {
            x: (mouse_pos.x*zoom_inv - frac_pan.x).round(),
            y: (mouse_pos.y*zoom_inv - frac_pan.y).round(),
        };

        // Draw
        {
            if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                pen_pos_prev = None;
            }

            if !is_erasing && let Some(pos_prev) = &mut pen_pos_prev && pos_prev != &pen_pos {
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    d.draw_line_ex(*pos_prev, pen_pos, 2.0*brush_radius, brush_color);
                    d.draw_circle_v(pen_pos, brush_radius, brush_color);
                }
                *pos_prev = pen_pos;
                is_redraw_needed = true;
            }

            if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
                pen_pos_prev = Some(pen_pos);
                is_erasing = false;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    d.draw_circle_v(pen_pos, brush_radius, brush_color);
                }
                is_redraw_needed = true;
            }
        }

        // Erase
        {
            if rl.is_mouse_button_released(MOUSE_BUTTON_RIGHT) {
                pen_pos_prev = None;
            }

            if is_erasing && let Some(pos_prev) = &mut pen_pos_prev && pos_prev != &pen_pos {
                let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                {
                    let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                    d.draw_line_ex(*pos_prev, pen_pos, 2.0*brush_radius, Color::BLANK);
                    d.draw_circle_v(pen_pos, brush_radius, Color::BLANK);
                }
                *pos_prev = pen_pos;
                is_redraw_needed = true;
            }

            if rl.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT) {
                pen_pos_prev = Some(pen_pos);
                is_erasing = true;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    {
                        let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                        d.draw_circle_v(pen_pos, brush_radius, Color::BLANK);
                    }
                }
                is_redraw_needed = true;
            }
        }


        // Draw frame
        if is_redraw_needed {
            last_clear.truncate(const { "last redraw: ".len() });
            _ = write!(last_clear, "{}", rl.get_time());

            let pan = Vector2 {
                x: frac_pan.x.round(),
                y: frac_pan.y.round(),
            };

            let canvas_rec = Rectangle {
                x: pan.x * zoom,
                y: pan.y * zoom,
                width:  canvas.texture.width  as f32 * zoom,
                height: canvas.texture.height as f32 * zoom,
            };

            {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_rectangle_rec(canvas_rec, Color::new(42, 42, 42, 255));

                draw_texture_custom(&mut d, &canvas, &canvas_rec);
                // d.draw_ring(mouse_pos, brush_radius*zoom, brush_radius*zoom + 1.0, 0.0, 360.0, 10, Color::GRAY);
                d.draw_text(&(brush_radius * 2.0).to_string(), 0, 0, 10, Color::MAGENTA);

                d.draw_rectangle(0, 10, 150, 20, Color::new(20, 20, 20, 255));
                d.draw_text(&last_draw,  0, 10, 10, Color::MAGENTA);
                d.draw_text(&last_clear, 0, 20, 10, Color::MAGENTA);
            }

            rl.swap_screen_buffer();
            is_redraw_needed = false;
        }
    }
}
