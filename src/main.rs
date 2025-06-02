use raylib::prelude::*;
use MouseButton::*;
use BlendMode::*;
use KeyboardKey::*;

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

    let canvas_rec = Rectangle::new(0.0, 0.0, 720.0, 480.0);
    let mut canvas = rl.load_render_texture(&thread, canvas_rec.width as u32, canvas_rec.height as u32).unwrap();
    let mut pen_pos_prev = None;
    let mut is_frame_updated = true;
    let mut is_erasing = false;

    let mut brush = Brush {
        radius: 0.5,
        color: Color::default(),
    };

    let mut zoom_pow = 0.0;
    let mut pan = Vector2::zero();

    while !rl.window_should_close() {
        rl.poll_input_events();

        if rl.is_window_resized() {
            is_frame_updated = true;
        }

        let mouse_pos = rl.get_mouse_position();

        // Zoom + pan
        {
            let scroll = rl.get_mouse_wheel_move();

            if scroll != 0.0 {
                if rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_RIGHT_CONTROL) {
                    // zoom
                    zoom_pow = (zoom_pow + scroll).clamp(-4.0, 4.0);
                } else if rl.is_key_down(KEY_LEFT_SHIFT) || rl.is_key_down(KEY_RIGHT_SHIFT) {
                    // horizontal pan
                    pan.x += scroll * 10.0;
                } else if rl.is_key_down(KEY_LEFT_ALT) || rl.is_key_down(KEY_RIGHT_ALT) {
                    // pen size
                    brush.radius = (brush.radius + scroll * 0.5).max(0.5);
                } else {
                    // vertical pan
                    pan.y += scroll * 10.0;
                }
                is_frame_updated = true;
            }
        }

        let zoom = 2.0f32.powi(zoom_pow as i32);

        let canvas_rec = Rectangle {
            x: pan.x * zoom,
            y: pan.y * zoom,
            width: canvas_rec.width * zoom,
            height: canvas_rec.height * zoom,
        };

        let pen_pos = (mouse_pos - pan * zoom) / zoom;

        // Draw
        {
            if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                pen_pos_prev = None;
            }

            if !is_erasing && let Some(pos_prev) = &mut pen_pos_prev && pos_prev != &pen_pos {
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    d.draw_line_ex(*pos_prev, pen_pos, 2.0*brush.radius, Color::WHITE);
                    d.draw_circle_v(pen_pos, brush.radius, Color::WHITE);
                }
                *pos_prev = pen_pos;
                is_frame_updated = true;
            }

            if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
                pen_pos_prev = Some(pen_pos);
                is_erasing = false;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    d.draw_circle_v(pen_pos, brush.radius, Color::WHITE);
                }
                is_frame_updated = true;
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
                    d.draw_line_ex(*pos_prev, pen_pos, 2.0*brush.radius, Color::BLANK);
                    d.draw_circle_v(pen_pos, brush.radius, Color::BLANK);
                }
                *pos_prev = pen_pos;
                is_frame_updated = true;
            }

            if rl.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT) {
                pen_pos_prev = Some(pen_pos);
                is_erasing = true;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    {
                        let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                        d.draw_circle_v(pen_pos, brush.radius, Color::BLANK);
                    }
                }
                is_frame_updated = true;
            }
        }

        // Draw frame (only on change)
        if is_frame_updated {
            {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);

                d.draw_rectangle_rec(canvas_rec, Color::new(42, 42, 42, 255));

                draw_texture_custom(&mut d, &canvas, &canvas_rec);
                d.draw_text(&(brush.radius * 2.0).to_string(), 0, 0, 10, Color::MAGENTA);

                // Debug update frequency
                d.draw_text(&d.get_time().to_string(), 0, 10, 10, Color::MAGENTA);
            }

            rl.swap_screen_buffer();
        }

        is_frame_updated = false;
    }
}
