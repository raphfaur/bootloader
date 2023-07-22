#![no_std]
#![no_main]
use core::arch::asm;
use core::ptr::write_volatile;
use crate::debug;


pub fn print_str(str : &str) {
    for char in str.as_bytes(){
        printc(*char);
    }
}

pub fn write(x: u16, y : u16, color : u8) {
    unsafe {
        asm!(
        "pusha",
        "mov ah, 0x0c",
        "mov bh, 0x00",
        "int 0x10",
        "popa",
        in("dx") y,
        in("cx") x,
        in("al") color
        )
    }
}

pub fn print_slice(str : &[u8]) {
    for char in str{
        if char != &0 {
            printc(*char);
        }

    }
}

pub fn printc(i : u8) {
    unsafe {
        asm!(
        "pusha",
        "mov al, {}",
        "mov ah, 0x0e",
        "int 0x10",
        "popa",
        in(reg_byte) i
        )
    }
}

pub fn wait_input() -> u8{
    let al : u8;
    unsafe {
        asm!(
        "mov ah, 0x00",
        "int 0x16",
        "mov {}, al",
        out(reg_byte) al
        )
    }
    return al
}

pub fn print(str : &[u8]) {
    let mut i = 0;
    for &ch in str {
        let mut graphic_addr = (0x000b8000 + 2*i) as *mut u8;
        unsafe {
            write_volatile(graphic_addr, ch)
        }

        graphic_addr = (0x000b8000 + 2*i + 1) as *mut u8;
        unsafe  {
            write_volatile(graphic_addr, 0x07)
        }
        i += 1;
    }
}

#[inline(never)]
pub fn clear() -> () {
    unsafe {
        asm!(
        "pusha",
        "mov ah, 0x00",
        "mov al, 0x12",
        "int 0x10",
        "popa"
        )
    }
}