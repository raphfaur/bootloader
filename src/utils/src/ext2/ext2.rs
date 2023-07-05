#![no_std]

use alloc::string::String;

enum EXT2_STATE {
    EXT2_ERROR_FS,
    EXT2_VALID_FS,
}

enum EXT2_ERRORS {
    EXT2_ERRORS_CONTINUE,
    EXT2_ERRORS_RO,
    EXT2_ERRORS_PANIC
}

enum EXT2_OS {
    EXT2_OS_LINUX,
    EXT2_OS_HURD,
    EXT2_OS_MASIX,
    EXT2_OS_FREEBSD,
    EXT2_OS_LITES
}

enum EXT2_FEATURE_COMPAT {
    EXT2_FEATURE_COMPAT_DIR_PREALLOC,
    EXT2_FEATURE_COMPAT_IMAGIC_INODES,
    EXT3_FEATURE_COMPAT_HAS_JOURNAL,
    EXT2_FEATURE_COMPAT_EXT_ATTR,
    EXT2_FEATURE_COMPAT_RESIZE_INO,
    EXT2_FEATURE_COMPAT_DIR_INDEX
}

enum EXT2_FEATURE_INCOMPAT {
    EXT2_FEATURE_INCOMPAT_COMPRESSION,
    EXT2_FEATURE_INCOMPAT_FILETYPE,
    EXT3_FEATURE_INCOMPAT_RECOVER,
    EXT3_FEATURE_INCOMPAT_JOURNAL_DEV,
    EXT2_FEATURE_INCOMPAT_META_BG
}

enum EXT2_FEATURE_RO_COMPAT {
    EXT2_FEATURE_RO_COMPAT_SPARSE_SUPER,
    EXT2_FEATURE_RO_COMPAT_LARGE_FILE,
    EXT2_FEATURE_RO_COMPAT_BTREE_DIR
}

enum EXT2_ALG {
    EXT2_LZV1_ALG,
    EXT2_LZRW3A_ALG,
    EXT2_GZIP_ALG,
    EXT2_BZIP2_ALG,
    EXT2_LZO_ALG
}

pub struct Superblock {
    s_inodes_count : u32,
    s_blocks_count : u32,
    s_r_blocks_count : u32,
    s_free_blocks_count : u32,
    s_free_inodes_count : u32,
    s_first_datablock : u32,
    s_log_block_size : u32,
    s_log_frag_size : u32,
    s_blocks_per_group : u32,
    s_frags_per_group : u32,
    s_inodes_per_group : u32,
    s_mtime : u32,
    s_wtime : u32,
    s_mnt_count : u16,
    s_max_mnt_count : u16,
    s_magic : u16, //0xEF53
    s_state : EXT2_STATE,
    s_errors : EXT2_ERRORS,
    s_minor_rev_level : u16,
    s_lastcheck : u32,
    s_checkinterval : u32,
    s_creator_os : EXT2_OS,
    s_rev_level : u32,
    s_def_resuid : u16,
    s_def_resgid : u16,
    s_first_ino : u32,
    s_inode_size : u16,
    s_block_group_nr : u16,
    s_feature_compat : EXT2_FEATURE_COMPAT,
    s_feature_incompat : EXT2_FEATURE_INCOMPAT,
    s_feature_ro_compat : EXT2_FEATURE_RO_COMPAT,
    s_uuid : u128,
    s_volume_name : u16,
    s_last_mounted : u64,
    s_algo_bitmap : EXT2_ALG,
    s_prealloc_blocks : u8,
    s_prealloc_dir_block : u8,
    s_journal_uuid : u16,
    s_journal_inum : u32,
    s_journal_dev : u32,
    s_last_orphan : u32,
    s_hash_seed : [u32; 4],
    s_def_hash_version : u8,
    s_default_mount_options : u32,
    s_first_meta_bg : u32
}

struct BlockGroupDescriptor {
    bg_block_bitmap : u32,
    bg_inode_bitmap : u32,
    bg_inode_table : u32,
    bg_free_blocks_count : u16,
    bg_free_inodes_count : u16,
    bg_used_dirs_count : u16,
    bg_pad : u16,
    bg_reserved : [u8; 12]
}

enum EXT2_S_IF {
    EXT2_S_IFSOCK,
    EXT2_S_IFLNK,
    EXT2_S_IFREG,
    EXT2_S_IFBLK,
    EXT2_S_IFDIR,
    EXT2_S_IFCHR,
    EXT2_S_IFIFO
}

enum EXT2_S_IS {
    EXT2_S_ISUID,
    EXT2_S_ISGID,
    EXT2_S_ISVTX
}

enum EXT2_S_I {
    EXT2_S_IRUSR,
    EXT2_S_IWUSR,
    EXT2_S_IXUSR,
    EXT2_S_IRGRP,
    EXT2_S_IWGRP,
    EXT2_S_IXGRP,
    EXT2_S_IROTH,
    EXT2_S_IWOTH,
    EXT2_S_IXOTH,

}

enum EXT2_FL {

}

struct Inode {
    file_format : EXT2_S_IF,
    process_execution : EXT2_S_IS,
    access_rights : EXT2_S_I,
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
    i_flags : EXT2_FL,
    i_osd1 : u32,
    i_block : [u32; 15],
    i_generation : u32,
    i_file_acl : u32,
    i_dir_acl : u32,
    i_faddr : u32,
    i_osd2 : [u8; 12]
}

enum EXT2_FT {

}
struct LinkedDirectoryEntry {
    inode : &Inode,
    rec_len : u16,
    name_len : u8,
    file_type : EXT2_FT,
    name : String
}