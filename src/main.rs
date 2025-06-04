use raylib::prelude::*;

mod brush;
mod frame;
mod editor;

use brush::Brush;
use frame::Frame;
use editor::{Editor, art::ArtEditor, color::ColorEditor};

pub fn draw_texture_custom(_d: &mut impl RaylibDraw, texture: impl AsRef<ffi::Texture>, rec: &Rectangle, tint: Color) {
    use ffi::*;

    unsafe {
        let left = rec.x;
        let right = left + rec.width;
        let top = rec.y;
        let bottom = top + rec.height;

        rlSetTexture(texture.as_ref().id);
        rlBegin(RL_QUADS as i32);
        {
            rlColor4ub(tint.r, tint.g, tint.b, tint.a); // Tint
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

    let mut brush = Brush::new(&mut rl, &thread, 0.5, Color::WHITE);

    let mut art_editor = ArtEditor::new(rl.load_render_texture(&thread, 720, 480).unwrap());
    let mut color_editor = ColorEditor::new(&mut rl, &thread, &brush);
    let mut current_editor = Editor::Art;
    let mut frame = Frame::new(&mut rl, &thread);

    while !rl.window_should_close() {
        rl.poll_input_events();

        if rl.is_window_resized() {
            frame.resize(&mut rl, &thread);
            art_editor.mark_dirty();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            current_editor = match current_editor {
                Editor::Art => Editor::Color,
                Editor::Color => {
                    art_editor.mark_dirty();
                    Editor::Art
                }
            };
        }

        match current_editor {
            Editor::Art => art_editor.update(&mut rl, &thread, &mut brush, &mut frame),
            Editor::Color => color_editor.update(&mut rl, &thread, &mut brush, &mut frame),
        }

        if frame.is_dirty() {
            frame.present(&mut rl.begin_drawing(&thread));
            rl.swap_screen_buffer();
        }
    }
}
