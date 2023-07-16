#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
#![feature(ptr_metadata)]

use core::mem::transmute;
use core::panic::PanicInfo;
use numtoa::NumToA;
use utils;
use utils::debug;
use utils::file_system::ext4::{Debug, Ext4Partition,Superblock};
use utils::gui::gui::{clear, print_slice, print_str, printc, wait_input};
use core::str;
use utils::video_io::io::cprint_info;
extern crate rlibc;

#[no_mangle]
#[inline(never)]
pub extern "C" fn _start() -> ! {
    clear();
    let ext4 = Ext4Partition {
        offset : 51,
        drive: 0x80
    };

    let result = ext4.read(0x400, 1024, 0x500);
    match result {
        Err(_) => print_str("Error"),
        Ok(_) => print_str("Loaded partition")
    }
    printc(10);
    printc(13);
    let sb : &mut Superblock;
    let addr = 0x500 as *mut Superblock;
    sb = unsafe {
        transmute(addr)
    };

    let block_size = 2u32.pow(10 + sb.s_log_block_size);
    if block_size == 4096 {
        print_str("Good block size")
    } else {
        print_str("Bad block size")
    }
    printc(10);
    printc(13);

    let mut root = sb.get_inode(0x2, &ext4);
    root = sb.get_inode(0x2, &ext4);
    root = sb.get_inode(0x2, &ext4);
    clear();
    root.parse_as_directory(0x1700,&ext4, block_size);
    let mut buffer = [0u8; 255];
    cprint_info(&buffer);
    let mut c = 0;
    let mut bob: &str;
    let mut buffer = [0u8; 255];
    loop {
        c = 0;
        root.parse_as_directory(0x1700,&ext4,block_size);
        printc(0xa);
        printc(13);
        print_str("--------------------------------------------------------------------------------");
        printc(0xa);
        printc(13);
        printc(0xa);
        printc(13);
        print_str("Enter a directory to navigate to : ");
        while c < 254 {
            let a = wait_input();
            printc(a);
            if a == 0xD {
                buffer[c] = 0;
                unsafe {
                    let path : *const str = core::ptr::from_raw_parts(buffer.as_ptr() as *const (), c) as *const str;
                    bob = transmute(path);
                    clear();
                    let a = root.search(0x1700,&ext4,block_size,0x2, bob);
                    if a == 0 {
                        buffer = [0u8; 255];
                        break
                    } else {
                        root = sb.get_inode(a, &ext4);
                        print_str(bob);
                        root.parse_as_directory(0x1700,&ext4,block_size);
                        buffer = [0u8; 255];
                        break
                    }

                }
            } else {
                buffer[c] = a;
                c += 1;
            }

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