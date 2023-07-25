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
use utils::gui::gui::{clear, print, print_slice, print_str, printc, switch_graphic, wait_input, write};
use core::str;
use utils::data::r#abstract::{DataSegment, Disk, SourceTest};
use utils::disk::disk::AddressPacket;
use utils::graphics::geometry::{join, Point,  Rectangle};
use utils::graphics::map::{MemMap, MemSeg};
use utils::graphics::text::{move_cursor, Text, write_char_graphic};

extern crate rlibc;

#[no_mangle]
#[inline(never)]
pub extern "C" fn _start() -> ! {

    let mut map = MemMap::new(0xfffff, 600, 40, 0x2, 0x4, 30, 500, 640);
    let a = map.compute_real_x(10);

    let seg1 = MemSeg::new(0, 0xf000, 1);
    let seg2 = MemSeg::new(0x1000, 0xFFFF, 1);
    let seg3 = MemSeg::new(0xFFFF, 0xbffff, 0);
    let seg4 = MemSeg::new(0xbFFFF, 0xfffff, 1);


    switch_graphic();
    map.add_segment(seg1);
    map.add_segment(seg2);
    map.add_segment(seg3);
    map.add_segment(seg4);




    //let usable = 0x9;
    //let not_usable = 0x5;
    //switch_graphic();
    //let r = Rectangle::new(40,40, 200,100);
    //r.draw_borders(0xf);
    //r.fill(usable);
    //let t = Text::new(0xf, 25, 110, "0x000");
    //t.draw(0);
    //let t = Text::new(0xf, 165, 110, "0xfff");
    //t.draw(0);

    //let r = Rectangle::new(200,40, 600,100);
    //r.draw_borders(0xf);
    //r.fill(not_usable);
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
    loop {

    }
}