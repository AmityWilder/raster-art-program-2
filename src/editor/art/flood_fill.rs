use std::collections::VecDeque;
use raylib::prelude::*;

fn get_color(img: &mut Image, x: i32, y: i32) -> Option<Color> {
    (0 <= x && x < img.width && 0 <= y && y < img.height)
        .then(|| img.get_color(x, y))
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
// enum Direction {
//     #[default]
//     Front,
//     Right,
//     Back,
//     Left,
// }

// impl Direction {
//     const fn turn(self, dir: Direction) -> Direction {
//         unsafe { std::mem::transmute((self as u8 + dir as u8) & 3) }
//     }
// }

pub fn flood_fill(img: &mut Image, x: i32, y: i32, new_color: Color) {
    let Some(start_color) = get_color(img, x, y) else { return; };
    if start_color == new_color { return; }

    let is_inside = move |img: &mut Image, x: i32, y: i32| -> bool {
        get_color(img, x, y)
            .is_some_and(|c| c == start_color)
    };

    let set = move |img: &mut Image, x: i32, y: i32| {
        img.draw_pixel(x, y, new_color);
    };

    let mut s = VecDeque::new();
    s.push_back((x, x, y, 1));
    s.push_back((x, x, y - 1, -1));
    while let Some((mut x1, x2, y, dy)) = s.pop_front() {
        let mut x = x1;
        if is_inside(img, x, y) {
            while is_inside(img, x - 1, y) {
                set(img, x - 1, y);
                x -= 1;
            }
            if x < x1 {
                s.push_back((x, x1 - 1, y - dy, -dy));
            }
        }
        while x1 <= x2 {
            while is_inside(img, x1, y) {
                set(img, x1, y);
                x1 += 1;
            }
            if x1 > x {
                s.push_back((x, x1 - 1, y + dy, dy));
            }
            if x1 - 1 > x2 {
                s.push_back((x2 + 1, x1 - 1, y - dy, -dy));
            }
            x1 += 1;
            while x1 < x2 && !is_inside(img, x1, y) {
                x1 += 1;
            }
            x = x1;
        }
    }
}
