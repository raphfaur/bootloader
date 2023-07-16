#![no_std]
#![no_main]

use utils::video_io::io;
use core::arch::{asm, global_asm};
use core::mem::transmute;
use core::ops::Deref;
use utils;
use core::panic::PanicInfo;
use core::ptr::write_volatile;
use utils::disk::disk::AddressPacket;
use utils::file_system::ext4::Superblock;
use utils::gui::gui::{clear, print, print_str, printc};
use utils::partitions::mbr_partition_table::PartitionTable;
use utils::print;
use utils::video_io::io::{__bios_print, __bios_printc, cprint_info};

global_asm!(include_str!("boot.asm"));

#[no_mangle]
pub extern "C" fn main() -> !{

    let address = AddressPacket::new(50,0x7E00,1);
    match address.disk_read(0x80) {
        Ok(_) => print_str("Loaded"),
        Err(_) => print_str("Error")
    }
    let jmp : extern "C" fn() -> ! ;
    let jmp_addr = 0x7E00 as *const();

    jmp = unsafe {
        transmute(jmp_addr)
    };

    jmp();

    loop {
    }
}

#[panic_handler]
pub fn panic(info : &PanicInfo) -> !{
    loop {

    }
}