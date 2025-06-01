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

    let mut canvas_rec = Rectangle::new(0.0, 0.0, 720.0, 480.0);
    let mut canvas = rl.load_render_texture(&thread, canvas_rec.width as u32, canvas_rec.height as u32).unwrap();
    let mut pen_pos_prev = None;
    let mut is_frame_updated = true;
    let mut is_erasing = false;

    let mut brush = Brush {
        radius: 5.0,
        color: Color::default(),
    };

    while !rl.window_should_close() {
        rl.poll_input_events();

        if rl.is_window_resized() {
            is_frame_updated = true;
        }

        let mouse_pos = rl.get_mouse_position();
        let is_on_canvas = canvas_rec.check_collision_point_rec(mouse_pos);

        // Zoom + pan
        {
            let scroll = rl.get_mouse_wheel_move();

            if rl.is_key_down(KEY_LEFT_CONTROL) || rl.is_key_down(KEY_RIGHT_CONTROL) {
                // zoom

            } else if rl.is_key_down(KEY_LEFT_SHIFT) || rl.is_key_down(KEY_RIGHT_SHIFT) {
                // horizontal pan

            } else {
                // vertical pan

            }
        }

        // Draw
        {
            let begin_drawing = is_on_canvas && rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT);

            if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
                pen_pos_prev = None;
            }

            if !is_erasing && let Some(pos_prev) = &mut pen_pos_prev && pos_prev != &mouse_pos {
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    d.draw_line_ex(*pos_prev, mouse_pos, 2.0*brush.radius, Color::WHITE);
                    d.draw_circle_v(mouse_pos, brush.radius, Color::WHITE);
                }
                *pos_prev = mouse_pos;
                is_frame_updated = true;
            } else if begin_drawing {
                pen_pos_prev = Some(mouse_pos);
                is_erasing = false;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    d.draw_circle_v(mouse_pos, 5.0, Color::WHITE);
                }
                is_frame_updated = true;
            }
        }

        // Erase
        {
            let begin_drawing = is_on_canvas && rl.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT);

            if rl.is_mouse_button_released(MOUSE_BUTTON_RIGHT) {
                pen_pos_prev = None;
            }

            if is_erasing && let Some(pos_prev) = &mut pen_pos_prev && pos_prev != &mouse_pos {
                let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                {
                    let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                    d.draw_line_ex(*pos_prev, mouse_pos, 2.0*brush.radius, Color::BLANK);
                    d.draw_circle_v(mouse_pos, brush.radius, Color::BLANK);
                }
                *pos_prev = mouse_pos;
                is_frame_updated = true;
            } else if begin_drawing {
                pen_pos_prev = Some(mouse_pos);
                is_erasing = true;
                {
                    let mut d = rl.begin_texture_mode(&thread, &mut canvas);
                    {
                        let mut d = d.begin_blend_mode(BLEND_CUSTOM_SEPARATE);
                        d.draw_circle_v(mouse_pos, 5.0, Color::BLANK);
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

                // Debug update frequency
                d.draw_text(&d.get_time().to_string(), 0, 0, 10, Color::MAGENTA);
            }

            rl.swap_screen_buffer();
        }

        is_frame_updated = false;
    }
}
