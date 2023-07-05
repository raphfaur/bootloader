#![no_std]
use core::arch::asm;

#[inline(always)]
pub fn print_str(str : &str) {
    for char in str.bytes() {
        printc(char as i8);
    }
}

#[inline(always)]
pub fn printc(i : i8) {
    unsafe {
        asm!(
        "mov al, {}",
        "mov ah, 0x0e",
        "int 0x10",
        in(reg_byte) i
        )
    }
}

pub fn clear() -> () {
    unsafe {
        asm!(
        "mov ah, 0x00",
        "mov al, 0x03",
        "int 0x10"
        )
    }
}