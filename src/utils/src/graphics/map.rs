use crate::debug;
use crate::graphics::geometry::Rectangle;
use crate::gui::gui::{print_str, printc};
use numtoa::NumToA;
use crate::graphics::text::Text;

pub struct MemSeg {
    base: u32,
    end: u32,
    mem_type: u8,
}

impl MemSeg {
    pub fn new(base: u32, end: u32, mem_type: u8) -> Self {
        Self {
            base,
            end,
            mem_type,
        }
    }
}

pub struct MemMap {
    total_size: u64,
    row_width: u16,
    row_height: u16,
    row_nb: u8,
    usable: u8,
    unusable: u8,
    padding: u16,
    padding_side : u16
}

impl MemMap {
    pub fn new(
        total_size: u64,
        row_width: u16,
        row_height: u16,
        usable: u8,
        unusable: u8,
        padding: u16,
        screen_height: u16,
        screen_width : u16,
    ) -> Self {
        let row_nb = (screen_height / (row_height + padding as u16)) as u8;
        let padding_side = (screen_width - row_width) / 2;
        Self {
            total_size,
            row_width,
            row_height,
            row_nb,
            usable,
            unusable,
            padding,
            padding_side,
        }
    }

    pub fn compute_real_x(&self, address: u32) -> (u8, u16) {
        let row = (address / (self.total_size / self.row_nb as u64) as u32) as u8;
        let row_offset = (address % (self.total_size / self.row_nb as u64) as u32) as u16;
        let x = (row_offset as u64 * self.row_width as u64 / (self.total_size / self.row_nb as u64)) as u16 + self.padding_side;
        return (row, x);
    }

    pub fn add_segment(&mut self, seg: MemSeg) {
        let (row_begin, x_begin) = self.compute_real_x(seg.base);
        let (mut row_end, mut x_end) = self.compute_real_x(seg.end);
        let delta = row_end - row_begin;
        let color = {
            if seg.mem_type == 0 {
                self.usable
            } else {
                self.unusable
            }
        };
        if delta == 0 {
            let row_y = self.padding + (row_begin as u16 * (self.padding + self.row_height)) as u16;
            let mut buffer = [0u8; 10];
            let t = Text::new(0xf, x_begin,row_y - self.padding / 3, seg.base.numtoa_str(16, &mut buffer));
            t.draw(0);
            let graphic_seg = Rectangle::new(x_begin, row_y, x_end, row_y + self.row_height);
            graphic_seg.draw_borders(0xf);
            graphic_seg.fill_horizontal(color);

            if seg.end as u64 == self.total_size {
                let t = Text::new(0xf, x_end,row_y - self.padding / 3, seg.end.numtoa_str(16, &mut buffer));
                t.draw(0);
            }

        } else {
            // Complete row
            let row_y = self.padding + (row_begin as u16 * (self.padding + self.row_height)) as u16;
            let mut buffer = [0u8; 10];
            let t = Text::new(0xf, x_begin,row_y - self.padding / 3, seg.base.numtoa_str(16, &mut buffer));
            t.draw(0);
            let graphic_seg = Rectangle::new(x_begin, row_y, self.row_width + self.padding_side, row_y + self.row_height);
            graphic_seg.draw_borders(0xf);
            graphic_seg.fill_horizontal(color);
            // Fill rows
            if x_end == self.padding_side {
                row_end -= 1;
            }
            for row in row_begin + 1..row_end {
                let row_y = self.padding + (row as u16 * (self.padding + self.row_height)) as u16;
                let graphic_seg = Rectangle::new(self.padding_side, row_y, self.row_width + self.padding_side, row_y + self.row_height);
                graphic_seg.draw_borders(0xf);
                graphic_seg.fill_horizontal(color);
            }
            // Begin of last row
            if x_end != self.padding_side {
                let row_y = self.padding + (row_end as u16 * (self.padding + self.row_height)) as u16;
                let graphic_seg = Rectangle::new(self.padding_side, row_y, x_end, row_y + self.row_height);
                graphic_seg.draw_borders(0xf);
                graphic_seg.fill_horizontal(color);

            }
            if seg.end as u64 == self.total_size {
                let t = Text::new(0xf, self.row_width ,row_y - self.padding / 3, seg.end.numtoa_str(16, &mut buffer));
                t.draw(0);
            }



        }
    }
}
