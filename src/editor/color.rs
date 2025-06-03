use raylib::prelude::*;
use crate::{brush::Brush, frame::Frame};

const LENGTH: f32 = 600.0;

pub struct ColorEditor {
    camera: Camera3D,
    pitch: f32,
    yaw: f32,
    shader: Shader,
    scale_loc: i32,
}

const VERT_CODE: &str = r"
#version 330

in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;

uniform mat4 mvp;
uniform vec3 scale;

out vec3 fragTexCoord;

void main() {
    fragTexCoord = vertexPosition/255.0;

    gl_Position = mvp*vec4(max(vertexPosition, (scale - 127.5)/255.0), 1.0);
}
";

const FRAG_CODE: &str = r"
#version 330

in vec3 fragTexCoord;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

out vec4 finalColor;

void main() {
    finalColor = vec4(fragTexCoord, 1.0);
}
";

impl ColorEditor {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let shader = rl.load_shader_from_memory(thread, Some(VERT_CODE), Some(FRAG_CODE));
        let scale_loc = shader.get_shader_location("scale");
        let pitch = std::f32::consts::PI;
        let yaw = std::f32::consts::PI;
        Self {
            camera: Camera3D::perspective(Vector3::new(0.0, 0.0, LENGTH), Vector3::zero(), Vector3::up(), 45.0),
            pitch,
            yaw,
            shader,
            scale_loc,
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

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            brush.color.r = brush.color.r.saturating_sub(1);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            brush.color.r = brush.color.r.saturating_add(1);
        }

        {
            let x = brush.color.r as f32;
            let y = brush.color.g as f32;
            let z = brush.color.b as f32;
            let color_v = Vector3::new(x, y, z);

            self.shader.set_shader_value(self.scale_loc, color_v);

            let mut d = frame.begin_drawing(rl, thread);
            d.clear_background(Color::BLACK);
            {
                let mut d = d.begin_mode3D(self.camera);
                {

                    let mut d = d.begin_shader_mode(&mut self.shader);
                    d.draw_cube_v(Vector3::zero(), Vector3::new(255.0, 255.0, 255.0), Color::WHITE);
                    d.draw_cube_v(color_v - Vector3::one() * 127.5, Vector3::one(), brush.color);
                }
            }
        }
    }
}