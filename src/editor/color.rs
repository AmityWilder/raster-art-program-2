use raylib::prelude::*;
use amygui::prelude::*;
use crate::{brush::Brush, frame::Frame};

const _FRAC_1_255: f32 = 1.0/255.0;

const _RGB_TO_OKLAB: [[f32; 3]; 3] = [
    [0.4122214708, 0.5363325363, 0.0514459929],
    [0.2119034982, 0.6806995451, 0.1073969566],
    [0.0883024619, 0.2817188376, 0.6299787005],
];

const fn _rgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [
        [m11, m12, m13],
        [m21, m22, m23],
        [m31, m32, m33],
    ] = _RGB_TO_OKLAB;
    [
        m11*r + m12*g + m13*b,
        m21*r + m22*g + m23*b,
        m31*r + m32*g + m33*b,
    ]
}

const _RGB_FROM_OKLAB: [[f32; 3]; 3] = [
    [ 4.07674,    -3.30771,   0.23097 ],
    [-1.26844,     2.60976,  -0.341319],
    [-0.00419609, -0.703419,  1.70761 ],
];

const fn _rgb_from_oklab(l: f32, a: f32, b: f32) -> [f32; 3] {
    let [
        [m11, m12, m13],
        [m21, m22, m23],
        [m31, m32, m33],
    ] = _RGB_FROM_OKLAB;
    [
        m11*l + m12*a + m13*b,
        m21*l + m22*a + m23*b,
        m31*l + m32*a + m33*b,
    ]
}

pub struct ColorEditor {
    color_hsv: Vector3,
}

impl ColorEditor {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread, brush: &Brush) -> Self {
        Self {
            color_hsv: brush.color.color_to_hsv(),
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, frame: &mut Frame) {
        {
            let mut d = frame.begin_drawing(rl, thread);
            d.clear_background(Color::BLACK);
            self.color_hsv = d.gui_color_picker_hsv_wheel(Vector2::new(125.0, 125.0), 3.0, 65.0, 80.0, 100.0, 60, self.color_hsv);
            brush.color = Color::color_from_hsv(self.color_hsv.x, self.color_hsv.y, self.color_hsv.z);
            d.draw_rectangle(300, 5, 34, 34, Color::GRAY);
            d.draw_rectangle(301, 6, 32, 32, brush.color);
        }
    }
}
