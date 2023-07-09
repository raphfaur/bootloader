#![no_std]


use core::mem::transmute;
use core::ptr::{read_volatile, write_volatile};
use crate::disk::disk::AddressPacket;
use crate::gui::gui::{print_str, printc};
use crate::video_io::io::{__bios_print, __bios_printc, cprint_info};
use numtoa;
use numtoa::NumToA;

#[repr(C, packed)]
pub struct Superblock {
    pub s_inodes_count : u32,
    pub s_blocks_count : u32,
    pub s_r_blocks_count : u32,
    pub s_free_blocks_count : u32,
    pub s_free_inodes_count : u32,
    pub s_first_datablock : u32,
    pub s_log_block_size : u32,
    pub s_log_frag_size : u32,
    pub s_blocks_per_group : u32,
    pub s_frags_per_group : u32,
    pub s_inodes_per_group : u32,
    pub s_mtime : u32,
    pub s_wtime : u32,
    pub s_mnt_count : u16,
    pub s_max_mnt_count : u16,
    pub s_magic : u16, //0xEF53
    pub s_state : u16,
    pub s_errors : u16,
    pub s_minor_rev_level : u16,
    pub s_lastcheck : u32,
    pub s_checkinterval : u32,
    pub s_creator_os : u32,
    pub s_rev_level : u32,
    pub s_def_resuid : u16,
    pub s_def_resgid : u16,
    pub s_first_ino : u32,
    pub s_inode_size : u16,
    pub s_block_group_nr : u16,
    pub s_feature_compat : u32,
    pub s_feature_incompat : u32,
    pub s_feature_ro_compat : u32,
    pub s_uuid : [u8;16],
    pub s_volume_name : [u8; 16],
    pub s_last_mounted : [u8;64],
    pub s_algo_bitmap : u32,
    pub s_prealloc_blocks : u8,
    pub s_prealloc_dir_block : u8,
    pub s_journal_uuid : [u8; 16],
    pub s_journal_inum : u32,
    pub s_journal_dev : u32,
    pub s_last_orphan : u32,
    pub s_hash_seed : [u32; 4],
    pub s_def_hash_version : u8,
    pub s_default_mount_options : u32,
    pub s_first_meta_bg : u32,
    pub s_mkfs_time : u32,
    pub s_jnl_blocks : [u32; 17],
    pub s_blocks_count_hi : u32,
    pub s_r_blocks_count_hi : u32,
    pub s_free_blocks_count_hi: u32,
    pub s_min_extra_isize : u16,
    pub s_want_extra_isize : u16,
    pub s_flags: u32,
    pub s_raid_stride : u16,
    pub s_mmp_interval : u16,
    pub s_mmp_block : u64,
    pub s_raid_stripe_width : u32,
    pub s_log_groups_per_flex : u8,
    pub s_checksum_type : u8,
    pub s_reserved_pad : u16,
    pub s_kbytes_written : u64,
    pub s_snapshot_inum : u32,
    pub s_snapshot_id : u32,
    pub s_snapshot_r_blocks_count : u64,
    s_snapshot_list : u32,
    s_error_count : u32,
    s_first_error_time : u32,
    s_first_error_ino : u32,
    s_first_error_block : u64,
    s_first_error_func : [u8; 32],
    s_first_error_line : u32,
    s_last_error_time : u32,
    s_last_error_ino : u32,
    s_last_error_line : u32,
    s_last_error_block : u64,
    s_last_error_func : [u8; 32],
    s_mount_opts : [u8; 64],
    s_usr_quota_inum : u32,
    s_grp_quota_inum : u32,
    s_overhead_blocks : u32,
    s_backup_bgs : [u32; 2],
    s_encrypt_algos : [u8; 4],
    s_encrypt_pw_salt : [u8; 16],
    s_lpf_ino : u32,
    s_prj_quota_inum : u32,
    s_checksum_seed : u32,
    s_reserved : [u32; 98],
    s_checksum : u32
}



impl Superblock {
    pub fn list_root (&self) {

    }

    pub fn load_block(&self, n : u32, partition : Ext4Partition, buffer : u32) -> Result<(),()>{
        let block_size_bytes = 2u32.pow((10 + self.s_log_block_size)) as u32;

        let result = partition.read(n * block_size_bytes, block_size_bytes, buffer);
        match result {
            Ok(_) => Ok(()),
            _ => Err(())
        }
    }

    // Returns a reference to an Inode given its number (assuming default inode record size is 256 bytes)
    pub fn get_inode(&mut self, inode_nb : u32, partition : &Ext4Partition) -> &Inode {

        let block_group = (inode_nb -1) / self.s_inodes_per_group;
        let index = (inode_nb - 1) % self.s_inodes_per_group;

        let block_size =  2u32.pow((10 + self.s_log_block_size)) as u32;
        partition.read( block_size + 64 * block_group, 512, 0x7E00);
        let grp_descriptor_addr = 0x7E00 as *mut BlockGroupDescriptor32;
        let grp_desc : &BlockGroupDescriptor32;
        grp_desc = unsafe {
            transmute(grp_descriptor_addr)
        };

        if self.s_inode_size == 0 {
            self.s_inode_size = 256
        }

        let inode_table_address = grp_desc.bg_inode_table * block_size;
        let inode_address = inode_table_address + (self.s_inode_size as u32) * index;

        let result = partition.read(inode_address, 512, 0x7E00);
        match result {
            Ok(_) => print_str("Ok"),
            _ => print_str("Error")
        }

        let inode : &Inode;
        let inode_addr = (0x7E00 + inode_address % 512) as *mut Inode;
        inode = unsafe {
            transmute(inode_addr)
        };

        return inode;
    }
}


#[repr(C, packed)]
struct BlockGroupDescriptor32 {
    bg_block_bitmap : u32,
    bg_inode_bitmap : u32,
    bg_inode_table : u32,
    bg_free_blocks_count : u16,
    bg_free_inodes_count : u16,
    bg_used_dirs_count : u16,
    bg_flags : u16,
    bg_exclude_bitmap_lo : u32,
    bg_block_bitmap_csum_lo : u16,
    bg_inode_bitmap_csum_lo : u16,
    bg_itable_unused_lo : u16,
    bg_checksum : u16,
    bg_reserved : [u8; 32]
}

#[repr(C, packed)]
pub struct Inode {
    i_mode : u16,
    i_uid : u16,
    i_size : i32,
    i_atime : u32,
    i_ctime : u32,
    i_mtime : u32,
    i_dtime : u32,
    i_gid : u16,
    i_links_count : u16,
    i_blocks : u32,
    i_flags : u32,
    i_osd1 : u32,
    i_block : [u32; 15],
    i_generation : u32,
    i_file_acl : u32,
    i_dir_acl : u32,
    i_faddr : u32,
    i_osd2 : [u8; 12],
    i_extra_isize : u16,
    i_checksum_hi : u16,
    i_ctime_extra : u32,
    i_mtime_extra : u32,
    i_atime_extra : u32,
    i_crtime : u32,
    i_crtime_extra : u32,
    i_version_hi : u32,
    i_projid : u32,
}

#[repr(C, packed)]
struct ext4_extent_header {
    eh_magic : u16,
    eh_entries : u16,
    eh_max : u16,
    eh_depth : u16,
    eh_generation : u32
}

struct ext4_extent {
    ee_block : u32,
    ee_len : u16,
    ee_start_hi : u16,
    ee_start_lo : u32
}
#[repr(C, packed)]
struct ext4_extent_idx {
    ei_block : u32,
    ei_leaf_lo : u32,
    ei_leaf_hi : u16,
    ei_unused : u16
}


impl Inode {

    pub fn uses_extent_tree(&self) -> bool {
        return self.i_flags == 0x80000
    }

    pub fn get_nth_data_block(&self) -> u64 {
        if self.uses_extent_tree() {

        }
        return 0
    }

    fn get_nth_data_block_extent(&self, block_size : u32, n : u32, partition : &Ext4Partition) -> u64 {
        let mem_offset = 0;
        let root_entries = self.i_block[0] >> 16 as u16;
        let max_depth = self.i_block[1] >> 16 as u16;
        for i in 1..root_entries {
            let result = self.explore_next_layer(block_size,n, block_size, partition);
            if result != 0 {
                return result
            }
        }
        return 0
    }

    fn explore_next_layer(&self, mem_offset : u32, n : u32, block_size : u32, partition : &Ext4Partition) -> u64 {
        let header_address = mem_offset as *const ext4_extent_header;
        let header : &ext4_extent_header;
        header = unsafe {
            transmute(header_address)
        };
        let leaf_number = header.eh_entries;
        // Handle leaves
        if header.eh_depth == 0 {
            for i in 0..leaf_number {
                let extent_addr = (mem_offset + 12*(i+1) as u32) as *const ext4_extent;
                let extent : &ext4_extent;
                extent = unsafe {
                    transmute(extent_addr)
                };
                let len = {
                    if extent.ee_len <= 32768 {
                        extent.ee_len
                    } else {
                        extent.ee_len - 32768
                    }
                };
                if extent.ee_block + len as u32 >= n {
                    // Return address of n-th block
                    return (extent.ee_start_lo as u64 + (extent.ee_start_hi as u64) << 32) + (block_size * (n - extent.ee_block)) as u64
                }
            }
            return 0
        } else {
            // Offset + block_size in memory and iterate over entries recursively
            for i in 0..leaf_number {
                let extent_idx : &ext4_extent_idx;
                let extent_idx_addr = (mem_offset + 12*(i+1) as u32) as *const ext4_extent_idx;
                extent_idx = unsafe {
                    transmute(extent_idx_addr)
                };
                let next_block_address = extent_idx.ei_leaf_lo as u64 + (extent_idx.ei_leaf_hi as u64) << 2;
                let _ = partition.read(next_block_address as u32, block_size, mem_offset + block_size);
                return self.explore_next_layer(mem_offset + block_size, n, block_size, partition);
            }
            return 0
        }

    }

    pub fn get_path(&self, offset : u64, s_log_block_size : u32) -> Result<([u32; 4], (usize, u64)), u8> {
        if offset > self.i_size as u64 {
            // Offset outside file : 3
            return Err(3);
        }
        if s_log_block_size as u64 * 12 > offset {
            return self.get_path_recursive(offset - 12 * s_log_block_size as u64, 1, s_log_block_size);
        } else {
            let mut path = [0u32;4];
            path[0] = (offset / s_log_block_size as u64) as u32;
            return Ok((path, (0, offset % s_log_block_size as u64)))
        }
    }

    pub fn get_path_recursive(&self, mut offset : u64, depth : usize, s_log_block_size : u32) -> Result<([u32;4], (usize,u64)), u8> {
        if depth > 3 {
            // Max Depth reached : 1
            return Err(1)
        }
        // Compute the number of bytes contained
        let bytes_count = (s_log_block_size / 32).pow(depth as u32) * s_log_block_size;
        if offset > bytes_count as u64 {
            return self.get_path_recursive(offset - bytes_count as u64, depth + 1, s_log_block_size)
        } else {
            let mut path = [0u32; 4];
            let mut i = 0;
            while offset > s_log_block_size as u64 {
                path[i] = (offset / (s_log_block_size as u64 / 32).pow((depth - i) as u32) as u64) as u32;
                offset = offset - offset % (s_log_block_size as u64 / 32).pow((depth - i) as u32) as u64;
                i += 1;
                if i > 2 {
                    // Error in recursivity : 2
                    return Err(2);
                }
            };
            return Ok((path, (depth, offset as u64)))
        }
    }

    pub fn get_address(&self, offset : u64, s_log_block_size : u32) -> Result<u64, u8>{
        match self.get_path(offset, s_log_block_size) {
            Err(e) => Err(e),
            Ok((path, (depth, off))) => {
                match depth {
                    0 => Ok((self.i_block[path[0] as usize] as u64 + off) as u64),
                    _ => Ok(0)
                }
            }
        }
    }
}


#[inline(never)]
pub fn parse_directory(offset : u32) {
    let mut parser = offset;
    let mut inode = unsafe {
        read_volatile(offset as *const u32)
    };

    let mut begin = parser;
    parser += 4;
    let rec_len = unsafe {
        read_volatile(parser as *const u16)
    };
    parser += 2;
    let name_len = unsafe {
        read_volatile(parser as *const u8)
    };
    parser += 1;
    let type_flag = unsafe {
        read_volatile(parser as *const u8)
    };
    parser += 1;
    for i in 0..name_len {
        let char = unsafe {
            read_volatile(parser as *const u8)
        };
        parser +=1;
    }
    parser = begin + rec_len as u32;
    // The end is defined by a 0x00 inode pointer
    while inode != 0x00 {
        let mut begin = parser;
        parser += 4;
        let rec_len = unsafe {
            read_volatile(parser as *const u16)
        };
        parser += 2;
        let name_len = unsafe {
            read_volatile(parser as *const u8)
        };
        parser += 1;
        let type_flag = unsafe {
            read_volatile(parser as *const u8)
        };

        match type_flag {
            0x1 => cprint_info(b"(File)       "),
            0x2 => cprint_info(b"(Directory)  "),
            _ =>  ()

        }
        parser += 1;
        for i in 0..name_len {
            let char = unsafe {
                read_volatile(parser as *const u8)
            };
            parser +=1;
            __bios_printc(char);
        }
        __bios_printc(0x0a);
        __bios_printc(0x0d);
        parser = begin + rec_len as u32;

        inode = unsafe {
            read_volatile(parser as *const u32)
        };
    }
}

pub struct Ext4Partition {
    pub offset : u32,
    pub drive : u8
}

impl Ext4Partition {
    #[inline(never)]
    pub fn read(&self, offset : u32, length : u32, buffer : u32) -> Result<(), ()> {
        let address = AddressPacket::new((length / 512) as u16 , buffer, (offset/512 + self.offset - 1) as u64);
        return address.disk_read(self.drive);
    }
}


enum EXT2_FT {

}

#[repr(C, packed)]
struct LinkedDirectoryEntry {
    inode : u16,
    rec_len : u16,
    name_len : u8,
    file_type : EXT2_FT,
    name : u32
}