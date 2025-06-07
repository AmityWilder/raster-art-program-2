#![windows_subsystem = "windows"]

use raylib::prelude::*;

mod brush;
mod frame;
mod editor;

use brush::Brush;
use frame::Frame;
use editor::{EditorID, art::ArtEditor, color::ColorEditor};
use rfd::FileDialog;
use crate::editor::Editor;

fn main() {
    let (mut rl, thread) = init()
        .title("Amity Raster Art")
        .resizable()
        .build();

    rl.set_exit_key(None);
    rl.maximize_window();

    unsafe {
        ffi::EnableEventWaiting();
    }

    let mut brush = Brush::new(0.5, Color::WHITE);

    let mut art_editor = ArtEditor::new(rl.load_render_texture(&thread, 720, 480).unwrap());
    let mut color_editor = ColorEditor::new(&mut rl, &thread, &brush);
    let mut current_editor = EditorID::Art;
    let mut frame = Frame::new(&mut rl, &thread);
    let mut current_path = None;

    art_editor.set_pan(Vector2::new(0.0, ColorEditor::HEIGHT as f32));

    while !rl.window_should_close() {
        rl.poll_input_events();

        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) {
            if rl.is_key_pressed(KeyboardKey::KEY_S) {
                if current_path.is_none() || (rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT)) {
                    current_path = FileDialog::new()
                        .set_can_create_directories(true)
                        .add_filter("image", &["png"])
                        .set_file_name("image.png")
                        .save_file();
                }

                if let Some(path) = &current_path {
                    art_editor.save(&mut rl, &thread, path).unwrap();
                }
            } else if rl.is_key_pressed(KeyboardKey::KEY_O) {
                let load_path = FileDialog::new()
                    .add_filter("image", &["png"])
                    .pick_file();

                if let Some(path) = &load_path {
                    art_editor.import(&mut rl, &thread, path).unwrap();
                }
            }
        }

        let mouse_pos = rl.get_mouse_position();

        if rl.is_window_resized() {
            frame.resize(&mut rl, &thread);
            art_editor.mark_dirty();
            color_editor.mark_dirty();
        }

        let is_color_editor_focused = color_editor.is_focused();
        let is_art_editor_focused = art_editor.is_focused();

        debug_assert!([is_color_editor_focused, is_art_editor_focused].into_iter().map(|x| x as usize).sum::<usize>() <= 1, "only one editor should be focused at a time");

        if !is_color_editor_focused && !art_editor.is_focused() {
            current_editor = match mouse_pos.y as i32 {
                ..ColorEditor::HEIGHT => EditorID::Color,
                ColorEditor::HEIGHT.. => EditorID::Art,
            };
        }

        let color_viewport = rrect(0, 0, rl.get_screen_width(), ColorEditor::HEIGHT);
        color_editor.update(&mut rl, &thread, &mut brush, color_viewport, &mut frame, current_editor == EditorID::Color);

        let art_viewport = rrect(0, ColorEditor::HEIGHT, rl.get_screen_width(), rl.get_screen_height() - ColorEditor::HEIGHT);
        art_editor.update(&mut rl, &thread, &mut brush, art_viewport, &mut frame, current_editor == EditorID::Art);

        if frame.is_dirty() {
            #[cfg(debug_assertions)] {
                let mut d = frame.begin_drawing(&mut rl, &thread);
                d.draw_rectangle(0, 0, 60, 10, Color::BLACK);
                d.draw_text(&d.get_time().to_string(), 0, 0, 10, Color::MAGENTA);
            }
            frame.present(&mut rl.begin_drawing(&thread));
            rl.swap_screen_buffer();
        }
    }
}
