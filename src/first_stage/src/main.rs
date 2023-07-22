#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
#![feature(ptr_metadata)]

use core::mem::transmute;
use core::panic::PanicInfo;
use core::ptr::{read, read_volatile};
use numtoa::NumToA;
use utils;
use utils::debug;
use utils::file_system::ext4::{Debug, Ext4Partition,Superblock};
use utils::gui::gui::{clear, print, print_slice, print_str, printc, wait_input, write};
use core::str;
use utils::disk::disk::AddressPacket;
use utils::video_io::io::cprint_info;
extern crate rlibc;

#[no_mangle]
#[inline(never)]
pub extern "C" fn _start() -> ! {
    let x_size = 480;
    let y_size = 400;
    clear();
    let mut read_add;
    for j in 0..40000 {
        let mut address = 0x600;
        read_add = AddressPacket::new(5, 0x600,51 + j * 5);
        match read_add.disk_read(0x80) {
            Ok(_) => {},
            Err(_) =>write(100,100, 0xF)
        }
        for i in 0..512{
            let next_x = unsafe {
                read_volatile(address as *const u16 )
            };
            address += 2;
            let next_y = unsafe {
                read_volatile(address as *const u16 )
            };
            address += 2;
            let color = unsafe {
                read_volatile(address as *const u8)
            };
            address += 1;
            write(next_x + 320 - x_size / 2,next_y + 240 - y_size / 2, color  );
        }
    }





    loop {

    }
}

#[panic_handler]
pub fn panic(info : &PanicInfo) -> !{
    loop {

    }
}