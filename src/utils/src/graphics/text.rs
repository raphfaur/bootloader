use core::arch::asm;
use crate::gui::gui::switch_graphic;

pub fn write_char_graphic(i : u8, color : u8) {
    unsafe {
        asm!(
        "pusha",
        "mov al, {}",
        "mov ah, 0x0a",
        "mov bh, 0x0",
        "mov bl, {}",
        "mov cx, 0x01",
        "int 0x10",
        "popa",
        in(reg_byte) i,
        in(reg_byte) color
        )
    }
}

pub fn move_cursor(x : u8, y : u8) {
    unsafe {
        asm!(
        "pusha",
        "mov dh, {}",
        "mov dl, {}",
        "mov ah, 0x02",
        "mov bh, 0x0",
        "int 0x10",
        "popa",
        in(reg_byte) y,
        in(reg_byte) x
        )
    }
}



pub struct Text<'taudelejaune> {
    color : u8,
    x : u16,
    y : u16,
    text : &'taudelejaune str,
}

impl<'a> Text<'a> {
    pub fn new(color : u8, x : u16, y : u16, text : &'a str) -> Self {
        return Self {
            color,
            x,
            y,
            text,
        }
    }

    pub fn draw(&self, direction : u8) {
        let mut x = self.x / 8;
        let mut y = self.y / 16;
        for char in self.text.bytes() {
            move_cursor(x as u8, y as u8);
            write_char_graphic(char, self.color);
            if direction == 0 {
                x += 1
            } else {
                y += 1
            }
        }
    }
}