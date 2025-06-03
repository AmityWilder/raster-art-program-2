use raylib::prelude::*;
use crate::{brush::Brush, frame::Frame};

const LENGTH: f32 = 600.0;

pub struct ColorEditor {
    camera: Camera3D,
    pitch: f32,
    yaw: f32,
}

impl ColorEditor {
    pub fn new() -> Self {
        let pitch = std::f32::consts::PI;
        let yaw = std::f32::consts::PI;
        Self {
            camera: Camera3D::perspective(Vector3::new(0.0, 0.0, LENGTH), Vector3::zero(), Vector3::up(), 45.0),
            pitch,
            yaw,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, frame: &mut Frame) {
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let Vector2 { x, y } = rl.get_mouse_delta() * 0.005;
            self.yaw += -x;
            self.pitch += y;
            let rot = Quaternion::from_euler(self.pitch, self.yaw, 0.0);
            self.camera.position = Vector3::new(0.0, 0.0, LENGTH).rotate_by(rot);
        }

        {
            let mut d = frame.begin_drawing(rl, thread);
            d.clear_background(Color::BLACK);
            {
                let mut d = d.begin_mode3D(self.camera);

                let x = brush.color.r as f32;
                let y = brush.color.g as f32;
                let z = brush.color.b as f32;
                let color_v = Vector3::new(x, y, z);
                d.draw_cube_v(Vector3::zero(), color_v, Color::GRAY);
                d.draw_cube_v(color_v - Vector3::one() * 127.5, Vector3::one(), brush.color);
            }
        }
    }
}