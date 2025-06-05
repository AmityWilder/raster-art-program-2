use raylib::prelude::*;
use crate::{brush::Brush, draw_texture_custom, frame::Frame};

const FRAC_1_255: f32 = 1.0/255.0;

const RGB_TO_OKLAB: [[f32; 3]; 3] = [
    [0.4122214708, 0.5363325363, 0.0514459929],
    [0.2119034982, 0.6806995451, 0.1073969566],
    [0.0883024619, 0.2817188376, 0.6299787005],
];

const fn rgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [
        [m11, m12, m13],
        [m21, m22, m23],
        [m31, m32, m33],
    ] = RGB_TO_OKLAB;
    [
        m11*r + m12*g + m13*b,
        m21*r + m22*g + m23*b,
        m31*r + m32*g + m33*b,
    ]
}

const RGB_FROM_OKLAB: [[f32; 3]; 3] = [
    [ 4.07674,    -3.30771,   0.23097 ],
    [-1.26844,     2.60976,  -0.341319],
    [-0.00419609, -0.703419,  1.70761 ],
];

const fn rgb_from_oklab(l: f32, a: f32, b: f32) -> [f32; 3] {
    let [
        [m11, m12, m13],
        [m21, m22, m23],
        [m31, m32, m33],
    ] = RGB_FROM_OKLAB;
    [
        m11*l + m12*a + m13*b,
        m21*l + m22*a + m23*b,
        m31*l + m32*a + m33*b,
    ]
}

fn gui_color_triangle<D: RaylibDraw>(_d: &mut D, center: Vector2, radius: f32, color: Color) -> Color {
    use ffi::*;
    use raylib::prelude::{Color, Vector2};

    let hue = color.color_to_hsv().x;
    let color_max = Color::color_from_hsv(hue, 1.0, 1.0);

    unsafe {
        let tex_shapes = GetShapesTexture();
        rlSetTexture(tex_shapes.id);
        let shape_rect = GetShapesTextureRectangle();
        rlBegin(RL_TRIANGLES as i32);
            rlColor4ub(color_max.r, color_max.g, color_max.b, 255);
            rlTexCoord2f(shape_rect.x/tex_shapes.width as f32, (shape_rect.y + shape_rect.height)/tex_shapes.height as f32);
            rlVertex2f(center.x + hue.to_radians().cos()*radius, center.y + hue.to_radians().sin()*radius);

            rlColor4ub(255, 255, 255, 255);
            rlTexCoord2f(shape_rect.x/tex_shapes.width as f32, shape_rect.y/tex_shapes.height as f32);
            rlVertex2f(center.x + (hue + 240.0).to_radians().cos()*radius, center.y + (hue + 240.0).to_radians().sin()*radius);

            rlColor4ub(0, 0, 0, 255);
            rlTexCoord2f((shape_rect.x + shape_rect.width)/tex_shapes.width as f32, shape_rect.y/tex_shapes.height as f32);
            rlVertex2f(center.x + (hue + 120.0).to_radians().cos()*radius, center.y + (hue + 120.0).to_radians().sin()*radius);
        rlEnd();
        rlSetTexture(0);
    }

    color // todo
}

fn gui_hue_wheel_circle<D: RaylibDraw>(_d: &mut D, center: Vector2, inner_radius: f32, outer_radius: f32, hue: f32) -> f32 {
    use ffi::*;
    use raylib::prelude::{Color, Vector2};

    unsafe {
        let tex_shapes = GetShapesTexture();
        rlSetTexture(tex_shapes.id);
        let shape_rect = GetShapesTextureRectangle();
        rlBegin(RL_QUADS as i32);
        let mut color_a = Color::new(255, 0, 0, 255);
        let mut outer_a = Vector2::new(outer_radius + center.x, center.y);
        let mut inner_a = Vector2::new(inner_radius + center.x, center.y);
        for t in (1..=360u16).map(f32::from) {
            let color_b = Color::color_from_hsv(t, 1.0, 1.0);
            let (sin, cos) = t.to_radians().sin_cos();
            let outer_b = Vector2::new(cos*outer_radius + center.x, sin*outer_radius + center.y);
            let inner_b = Vector2::new(cos*inner_radius + center.x, sin*inner_radius + center.y);

            rlColor4ub(color_a.r, color_a.g, color_a.b, 255);
            rlTexCoord2f(shape_rect.x/tex_shapes.width as f32, (shape_rect.y + shape_rect.height)/tex_shapes.height as f32);
            rlVertex2f(outer_a.x, outer_a.y);

            rlTexCoord2f(shape_rect.x/tex_shapes.width as f32, shape_rect.y/tex_shapes.height as f32);
            rlVertex2f(inner_a.x, inner_a.y);

            rlColor4ub(color_b.r, color_b.g, color_b.b, 255);
            rlTexCoord2f((shape_rect.x + shape_rect.width)/tex_shapes.width as f32, shape_rect.y/tex_shapes.height as f32);
            rlVertex2f(inner_b.x, inner_b.y);

            rlTexCoord2f((shape_rect.x + shape_rect.width)/tex_shapes.width as f32, (shape_rect.y + shape_rect.height)/tex_shapes.height as f32);
            rlVertex2f(outer_b.x, outer_b.y);

            color_a = color_b;
            outer_a = outer_b;
            inner_a = inner_b;
        }
        rlEnd();
        rlSetTexture(0);
    }

    hue // todo
}

fn gui_color_picker_custom<D: RaylibDraw>(d: &mut D, bounds: Rectangle, color: Color) -> Color {
    let thick = 20.0;
    let half_width = 0.5*bounds.width;
    let half_height = 0.5*bounds.height;
    let center = Vector2::new(bounds.x + half_width, bounds.y + half_height);
    let outer_radius = half_width.min(half_height);
    let inner_radius = outer_radius - thick;
    let triangle_radius = inner_radius - 5.0;
    let hue = color.color_to_hsv().x;
    _ = gui_hue_wheel_circle(d, center, inner_radius, outer_radius, hue);
    gui_color_triangle(d, center, triangle_radius, color)
}

pub struct ColorEditor {}

impl ColorEditor {
    fn generate_tex<D: RaylibDraw>(d: &mut D, sat: f32) {
        for hue in 0..360 {
            for value in 0..255 {
                let color = Color::color_from_hsv(hue as f32, sat, value as f32*FRAC_1_255);
                d.draw_pixel(hue, 255 - value, color);
            }
        }
    }

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, brush: &Brush) -> Self {
        let Vector3 { x: hue, y: sat, z: lum } = brush.color.color_to_hsv();

        let mut hsv_tex = rl.load_render_texture(thread, 360, 255).unwrap();
        {
            let mut d = rl.begin_texture_mode(thread, &mut hsv_tex);
            Self::generate_tex(&mut d, sat);
        }

        Self {}
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, frame: &mut Frame) {
        {
            let mut d = frame.begin_drawing(rl, thread);
            d.clear_background(Color::BLACK);
            let bounds = Rectangle::new(5.0, 5.0, 255.0, 255.0);
            brush.color = gui_color_picker_custom(&mut d, bounds, brush.color);
            d.draw_rectangle(300, 5, 34, 34, Color::GRAY);
            d.draw_rectangle(301, 6, 32, 32, brush.color);
        }

        // if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        //     let mouse_pos = rl.get_mouse_position();
        //     let Vector2 { mut x, mut y } = mouse_pos;
        //     x = x.clamp(0.0, 360.0);
        //     y = (1.0 - y*FRAC_1_255).clamp(0.0, 1.0);
        //     if x != self.hue || y != self.lum {
        //         self.is_color_dirty = true;
        //         self.hue = x;
        //         self.lum = y;
        //     }
        // }

        // let scroll = rl.get_mouse_wheel_move();
        // if scroll != 0.0 {
        //     self.is_color_dirty = true;
        //     self.sat = (self.sat + scroll*FRAC_1_255).clamp(0.0, 1.0);
        //     {
        //         let mut d = rl.begin_texture_mode(thread, &mut self.hsv_tex);
        //         Self::generate_tex(&mut d, self.sat);
        //     }
        // }

        // if self.is_color_dirty {
        //     brush.color = Color::color_from_hsv(self.hue, self.sat, self.lum);

        //     let mut d = frame.begin_drawing(rl, thread);
        //     let rec = Rectangle { x: 0.0, y: 0.0, width: 360.0, height: 255.0 };
        //     draw_texture_custom(&mut d, &self.hsv_tex, &rec, Color::WHITE);
        //     let rec = Rectangle { x: 365.0, y: 20.0, width: 32.0, height: 32.0 };
        //     d.draw_rectangle_rec(rec, brush.color);
        // }
    }
}
