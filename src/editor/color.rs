use std::collections::VecDeque;

use raylib::prelude::*;
use amygui::prelude::*;
use crate::{brush::{Brush, InterpStyle}, editor::Editor, frame::Frame};

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
    is_colorwheel_dirty: bool,
    palette: VecDeque<Vector3>,
}

impl ColorEditor {
    const PADDING: f32 = 5.0;
    const PALETTE_GAP: f32 = 8.0;
    const OUTER_RADIUS: f32 = 100.0;
    const THICK: f32 = 20.0;
    const INNER_SEP: f32 = 15.0;
    const CENTER: Vector2 = Vector2::new(Self::OUTER_RADIUS + Self::PADDING, Self::OUTER_RADIUS + Self::PADDING);
    pub const HEIGHT: i32 = (Self::CENTER.y * 2.0) as i32;
    const BRUSH_SLOT_WIDTH: f32 = 40.0;
    const BRUSH_SLOT_X: f32 = Self::CENTER.x + Self::OUTER_RADIUS + Self::PADDING;
    const COLOR_SLOT_WIDTH: f32 = 20.0;
    const PALETTE_X: f32 = Self::BRUSH_SLOT_X + Self::BRUSH_SLOT_WIDTH + Self::PALETTE_GAP;
    const SEGMENT_WIDTH: f32 = Self::COLOR_SLOT_WIDTH + Self::PALETTE_GAP;
    const PALETTE_CAP: usize = 16;
    const PALETTE_REC: Rectangle = Rectangle::new(
        Self::PALETTE_X,
        Self::PADDING,
        Self::SEGMENT_WIDTH*Self::PALETTE_CAP as f32,
        Self::COLOR_SLOT_WIDTH,
    );
    const BRUSH_PREVIEW_REC: Rectangle = Rectangle::new(
        Self::BRUSH_SLOT_X,
        Self::PADDING + Self::BRUSH_SLOT_WIDTH + Self::PALETTE_GAP,
        Self::BRUSH_SLOT_WIDTH,
        Self::BRUSH_SLOT_WIDTH,
    );

    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread, brush: &Brush) -> Self {
        Self {
            color_hsv: brush.color.color_to_hsv(),
            is_colorwheel_dirty: true,
            palette: VecDeque::with_capacity(Self::PALETTE_CAP),
        }
    }
}

impl Editor for ColorEditor {
    #[inline]
    fn mark_dirty(&mut self) {
        self.is_colorwheel_dirty = true;
    }

    #[inline]
    fn is_dirty(&self) -> bool {
        self.is_colorwheel_dirty
    }

    #[inline]
    fn is_focused(&self) -> bool {
        false
    }

    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, viewport: Rectangle, frame: &mut Frame, is_awake: bool) {
        if is_awake {
            let mouse_pos = rl.get_mouse_position();

            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                if check_collision_point_circle(mouse_pos, Self::CENTER, Self::OUTER_RADIUS) {
                    self.is_colorwheel_dirty = true;
                }
            }

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if Self::PALETTE_REC.check_collision_point_rec(mouse_pos) {
                    let mouse_x_rel = mouse_pos.x - Self::PALETTE_X;
                    let (i, p) = (mouse_x_rel/Self::SEGMENT_WIDTH, mouse_x_rel % Self::SEGMENT_WIDTH);
                    let i = i as usize;
                    if i < self.palette.len() && p <= Self::COLOR_SLOT_WIDTH {
                        self.color_hsv = self.palette[i as usize];
                        self.is_colorwheel_dirty = true;
                    }
                } else if Self::BRUSH_PREVIEW_REC.check_collision_point_rec(mouse_pos) {
                    brush.interp = match brush.interp {
                        InterpStyle::Curve => InterpStyle::Line,
                        InterpStyle::Line  => InterpStyle::Space,
                        InterpStyle::Space => InterpStyle::Curve,
                    };
                }
            }

            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                if self.palette.len() == self.palette.capacity() {
                    self.palette.pop_front();
                }
                self.palette.push_back(self.color_hsv);
            }
        }

        if self.is_colorwheel_dirty {
            self.is_colorwheel_dirty = false;

            let mut d = frame.begin_drawing(rl, thread);
            let mut d = d.begin_scissor_mode(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
            {
                let mut d = d.begin_scissor_mode(viewport.x as i32, viewport.y as i32, viewport.width as i32, viewport.height as i32);
                d.clear_background(Color::new(8, 8, 8, 255));
                self.color_hsv = d.gui_color_picker_hsv_wheel(
                    Self::CENTER,
                    3.0,
                    Self::OUTER_RADIUS - Self::THICK - Self::INNER_SEP,
                    Self::OUTER_RADIUS - Self::THICK,
                    Self::OUTER_RADIUS,
                    60,
                    self.color_hsv,
                );
                brush.color = Color::color_from_hsv(self.color_hsv.x, self.color_hsv.y, self.color_hsv.z);
            }
            let mut color_slot = Rectangle::new(
                Self::BRUSH_SLOT_X,
                Self::PADDING,
                Self::BRUSH_SLOT_WIDTH,
                Self::BRUSH_SLOT_WIDTH,
            );

            // brush slot
            {
                d.draw_rectangle_rec(Rectangle::new(
                    color_slot.x - 1.0,
                    color_slot.y - 1.0,
                    color_slot.width + 2.0,
                    color_slot.height + 2.0,
                ), Color::GRAY);
                d.draw_rectangle_rec(color_slot, brush.color);
            }

            // brush preview
            {
                const POINTS: [Vector2; 7] = const { [
                    Vector2::new(0.1, 0.9),
                    Vector2::new(0.2, 0.5),
                    Vector2::new(0.4, 0.4),
                    Vector2::new(0.5, 0.5),
                    Vector2::new(0.6, 0.6),
                    Vector2::new(0.8, 0.5),
                    Vector2::new(0.9, 0.1),
                ] };

                let points = POINTS.map(|p| Vector2::new(
                    Self::BRUSH_PREVIEW_REC.x + p.x*Self::BRUSH_PREVIEW_REC.width,
                    Self::BRUSH_PREVIEW_REC.y + p.y*Self::BRUSH_PREVIEW_REC.height,
                ));

                d.draw_rectangle_rec(Self::BRUSH_PREVIEW_REC, Color::GRAY);
                brush.paint(&mut d, points, false);
            }

            // palette
            color_slot.x = Self::PALETTE_X;
            color_slot.width  = Self::COLOR_SLOT_WIDTH;
            color_slot.height = Self::COLOR_SLOT_WIDTH;

            for color in self.palette.iter().copied().map(Some).chain(std::iter::repeat(None)).take(Self::PALETTE_CAP) {
                d.draw_rectangle_rec(Rectangle::new(
                    color_slot.x - 2.0,
                    color_slot.y - 2.0,
                    color_slot.width + 4.0,
                    color_slot.height + 4.0,
                ), Color::GRAY);
                if let Some(Vector3 { x: hue, y: sat, z: val }) = color {
                    d.draw_rectangle_rec(Rectangle::new(
                        color_slot.x - 1.0,
                        color_slot.y - 1.0,
                        color_slot.width + 2.0,
                        color_slot.height + 2.0,
                    ), Color::WHITE);
                    d.draw_rectangle_rec(color_slot, Color::color_from_hsv(hue, sat, val));
                }
                color_slot.x += Self::SEGMENT_WIDTH;
            }
        }
    }
}
