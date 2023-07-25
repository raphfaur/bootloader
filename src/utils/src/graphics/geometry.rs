use crate::debug;
use crate::graphics::text::write_char_graphic;
use crate::gui::gui::{print_str, printc, switch_graphic, write};
use core::cmp::max;
use numtoa::NumToA;

pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub fn join(p1: &Point, p2: &Point, color: u8) {
    let dy = p2.y - p1.y;
    let dx = p2.x - p1.x;
    let max = max(dy, dx);
    if (dx == 0) & (dy == 0) {
        return;
    }
    if dx == 0 {
        for i in 0..(dy + 1) {
            write(p1.x, p1.y + i, color);
        }
    } else if dy == 0 {
        for i in 0..(dx + 1) {
            write(p1.x + i, p1.y, color);
        }
    } else if max == dx {
        let pas = dx / dy;
        let r = dx % dy;
        let mut cursx = 0;
        let mut cursy = 0;
        for i in 1..(dx + 1) {
            if i % pas == 0 {
                cursy += 1;
            }
            if r / dx * cursx > cursy {
                cursy += 1;
            }
            cursx += 1;
            write(p1.x + cursx, p1.y + cursy, color);
        }
    } else if max == dy {
        let pas = dy / dx;
        let r = dy % dx;
        let mut cursx = 0;
        let mut cursy = 0;
        for i in 1..(dy + 1) {
            if i % pas == 0 {
                cursx += 1;
            }
            if r / dy * cursy > cursx {
                cursx += 1;
            }
            cursy += 1;
            write(p1.x + cursx, p1.y + cursy, color);
        }
    }
}

pub struct Rectangle {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl Rectangle {
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn draw_borders(&self, color: u8) {
        let p1 = Point::new(self.x1, self.y1);
        let p2 = Point::new(self.x1, self.y2);
        let p3 = Point::new(self.x2, self.y2);
        let p4 = Point::new(self.x2, self.y1);
        join(&p1, &p2, color);
        join(&p2, &p3, color);
        join(&p4, &p3, color);
        join(&p1, &p4, color);
    }

    pub fn fill_horizontal(&self, color : u8) {
        for i in 0..(self.x2 - self.x1 - 1) {
            let p1 = Point::new(self.x1 + i + 1, self.y1 + 1);
            let p2 = Point::new(self.x1 + i + 1, self.y2 - 1);
            join(&p1, &p2, color)
        }
    }

    pub fn fill_vertical(&self, color : u8) {
        for i in 0..(self.y2 - self.y1 - 1) {
            let p1 = Point::new(self.x1 + 1, self.y1 + 1 + i);
            let p2 = Point::new(self.x2 - 1, self.y1 + i + 1);
            join(&p1, &p2, color)
        }
    }
}
