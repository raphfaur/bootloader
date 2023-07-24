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
use utils::data::r#abstract::{DataSegment, Disk, SourceTest};
use utils::disk::disk::AddressPacket;
use utils::graphics::image::{From, Image, Point};

extern crate rlibc;

#[no_mangle]
#[inline(never)]
pub extern "C" fn _start() -> ! {
    clear();
    let source = Disk::new(26112);
    if let Ok(img) = Image::from_disk(51, 0x600) {
        img.draw(Point::new(320, 240), source, 0x600);

    } else {
        print_str("error");
    }

    //debug!(img.x_size());

    loop {

    }
}

#[panic_handler]
pub fn panic(info : &PanicInfo) -> !{
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        print_str(s);
    } else {
        print_str("Panic occured");
    }
    print_str("Bob");
    loop {

    }
}