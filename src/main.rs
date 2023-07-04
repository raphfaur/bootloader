#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

mod gdt;

use gdt::gdt::{Gdtr, SegmentDescriptor};
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::{write, write_volatile};


#[no_mangle]
pub extern "C" fn _start()-> !{
    _init();
    let mut table= Gdtr::new();
    table.set_offset(0x12);
    // Code segment
    let mut cs = SegmentDescriptor::new();
    cs.set_base(0x00);
    cs.set_limit(0xFFFFF);
    cs.set_access_byte(0x9A);
    cs.set_flags(0xC);
    table.add_segment(cs);
    // Data segment
    let mut ds = SegmentDescriptor::new();
    ds.set_base(0x00);
    ds.set_limit(0xFFFFF);
    ds.set_access_byte(0x92);
    ds.set_flags(0xC);
    table.add_segment(ds);
    switch_protected(table);

    loop {
    }
}

#[inline(never)]
pub fn printc(i : i8) -> () {
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

pub fn _init() {
    clear();
    let a = "Welcome !";
    for i in a.bytes() {
        printc(i as i8);
    }
}

pub fn switch_protected(gdt : Gdtr) {
    // Deactivate interrupts
    unsafe {
        asm!(
        "cli"
        )
    }
    //Write gdt to memory
    gdt.write();
    //Load gdtr
    gdt.load();
    // Set protected bit
    unsafe {
        asm!(
        "mov eax, cr0",
        "or al, 1",
        "mov cr0, eax"
        )
    }
    let os_ptr = 0xff as *mut fn();
    unsafe { write_volatile(os_ptr, kernel)}

    //Far jump
    unsafe {
        asm!(
        "jmp cs:0xff"
        )
    }


}

fn kernel() {
    loop {

    }
}

#[panic_handler]
pub fn panic(info : &PanicInfo) -> !{
    loop {

    }
}