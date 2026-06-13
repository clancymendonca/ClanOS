//! ext2 on a secondary memory block device — read, bounded write, create/unlink, multi-block grow.

use alloc::{vec, vec::Vec};
use core::sync::atomic::{AtomicU64, Ordering};

use crate::block::{self, BlockDeviceId, BlockError};
use crate::storage::SECTOR_SIZE;

const BLOCK_SIZE: usize = 1024;
const SECTORS_PER_BLOCK: usize = BLOCK_SIZE / SECTOR_SIZE;
const TOTAL_BLOCKS: usize = 128;
const SUPERBLOCK_BLOCK: usize = 1;
const BG_DESC_BLOCK: usize = 2;
const BLOCK_BITMAP_BLOCK: usize = 3;
const INODE_BITMAP_BLOCK: usize = 4;
const INODE_TABLE_BLOCK: usize = 5;
const ROOT_INO: u32 = 2;
const FILE_INO: u32 = 12;
const ELF_INO: u32 = 13;
const ROOT_DIR_BLOCK: u32 = 10;
const FILE_DATA_BLOCK: u32 = 11;
const ELF_DATA_BLOCK0: u32 = 12;
const ELF_DATA_BLOCKS: usize = 4;

const MAX_FILE_BLOCKS: usize = 4;

/// Regular files allowed to be overwritten.
const WRITABLE_FILES: &[&str] = &["smoke.txt", "scratch.txt"];
/// New regular files that may be created via `create_file`.
const CREATABLE_FILES: &[&str] = &["scratch.txt"];
/// Files that may be unlinked (seed files are protected).
const UNLINKABLE_FILES: &[&str] = &["scratch.txt"];

static EXT2_DEVICE: AtomicU64 = AtomicU64::new(0);
static MOUNTED: AtomicU64 = AtomicU64::new(0);
static EXT2_WRITES: AtomicU64 = AtomicU64::new(0);
static EXT2_CREATES: AtomicU64 = AtomicU64::new(0);
static EXT2_UNLINKS: AtomicU64 = AtomicU64::new(0);
pub fn is_mounted() -> bool {
    MOUNTED.load(Ordering::Relaxed) != 0
}

pub fn device_id() -> Option<BlockDeviceId> {
    let raw = EXT2_DEVICE.load(Ordering::Relaxed);
    if raw == 0 {
        None
    } else {
        Some(BlockDeviceId::from_raw(raw))
    }
}

pub fn init() -> bool {
    if is_mounted() {
        return true;
    }
    let id = block::register_memory_fallback(TOTAL_BLOCKS * SECTORS_PER_BLOCK);
    EXT2_DEVICE.store(id.as_u64(), Ordering::Relaxed);
    if block::with_device(id, format_minimal).is_err() {
        return false;
    }
    MOUNTED.store(1, Ordering::Relaxed);
    true
}

pub fn read_file(path: &str) -> Result<Vec<u8>, ()> {
    if !is_mounted() {
        return Err(());
    }
    let id = device_id().ok_or(())?;
    block::with_device(id, || read_file_inner(path)).map_err(|_| ())?
}

pub fn write_count() -> u64 {
    EXT2_WRITES.load(Ordering::Relaxed)
}

pub fn create_count() -> u64 {
    EXT2_CREATES.load(Ordering::Relaxed)
}

pub fn unlink_count() -> u64 {
    EXT2_UNLINKS.load(Ordering::Relaxed)
}

pub fn write_file(path: &str, data: &[u8]) -> Result<(), ()> {
    if !is_mounted() {
        return Err(());
    }
    if !WRITABLE_FILES.contains(&path.trim_start_matches('/')) {
        return Err(());
    }
    if data.len() > BLOCK_SIZE * MAX_FILE_BLOCKS {
        return Err(());
    }
    let id = device_id().ok_or(())?;
    let _ = block::with_device(id, || write_file_inner(path, data)).map_err(|_| ())?;
    EXT2_WRITES.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn create_file(path: &str, data: &[u8]) -> Result<(), ()> {
    if !is_mounted() {
        return Err(());
    }
    let name = path.trim_start_matches('/');
    if !CREATABLE_FILES.contains(&name) {
        return Err(());
    }
    if data.len() > BLOCK_SIZE * MAX_FILE_BLOCKS {
        return Err(());
    }
    let id = device_id().ok_or(())?;
    match block::with_device(id, || create_file_inner(name, data)) {
        Ok(Ok(())) => {}
        _ => return Err(()),
    }
    EXT2_CREATES.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

pub fn unlink_file(path: &str) -> Result<(), ()> {
    if !is_mounted() {
        return Err(());
    }
    let name = path.trim_start_matches('/');
    if !UNLINKABLE_FILES.contains(&name) {
        return Err(());
    }
    let id = device_id().ok_or(())?;
    match block::with_device(id, || unlink_file_inner(name)) {
        Ok(Ok(())) => {}
        _ => return Err(()),
    }
    EXT2_UNLINKS.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

fn read_file_inner(path: &str) -> Result<Vec<u8>, ()> {
    let name = path.trim_start_matches('/');
    if name.is_empty() {
        return Err(());
    }
    let root = read_inode(ROOT_INO)?;
    if root.mode & 0xF000 != 0x4000 {
        return Err(());
    }
    let dir = read_inode_bytes(&root)?;
    let target_ino = find_dir_entry(&dir, name).ok_or(())?;
    let file = read_inode(target_ino)?;
    if file.mode & 0xF000 != 0x8000 {
        return Err(());
    }
    read_file_data(&file)
}

fn write_file_inner(path: &str, data: &[u8]) -> Result<(), ()> {
    let name = path.trim_start_matches('/');
    if name.is_empty() {
        return Err(());
    }
    let target_ino = resolve_regular_file_ino(name)?;
    write_file_data(target_ino, data)
}

fn create_file_inner(name: &str, data: &[u8]) -> Result<(), ()> {
    if name.is_empty() || find_dir_entry_in_root(name)?.is_some() {
        return Err(());
    }
    let ino = alloc_inode()?;
    append_dir_entry(ROOT_INO, name, ino)?;
    write_file_data(ino, data)
}

fn unlink_file_inner(name: &str) -> Result<(), ()> {
    let ino = remove_dir_entry(name)?;
    let file = read_inode(ino)?;
    for block_id in file.blocks.iter().copied() {
        if block_id != 0 {
            free_block(block_id)?;
        }
    }
    free_inode(ino)
}

fn resolve_regular_file_ino(name: &str) -> Result<u32, ()> {
    let ino = find_dir_entry_in_root(name)?.ok_or(())?;
    let file = read_inode(ino)?;
    if file.mode & 0xF000 != 0x8000 {
        return Err(());
    }
    Ok(ino)
}

fn find_dir_entry_in_root(name: &str) -> Result<Option<u32>, ()> {
    let root = read_inode(ROOT_INO)?;
    if root.mode & 0xF000 != 0x4000 {
        return Err(());
    }
    let dir = read_inode_bytes(&root)?;
    Ok(find_dir_entry(&dir, name))
}

fn write_file_data(ino: u32, data: &[u8]) -> Result<(), ()> {
    let file = read_inode(ino)?;
    if file.mode & 0xF000 != 0x8000 {
        return Err(());
    }
    let blocks_needed = if data.is_empty() {
        0
    } else {
        (data.len() + BLOCK_SIZE - 1) / BLOCK_SIZE
    };
    if blocks_needed > MAX_FILE_BLOCKS {
        return Err(());
    }
    let mut block_ids = file.blocks;
    let used = block_ids.iter().take_while(|&&b| b != 0).count();
    for i in used..blocks_needed {
        block_ids[i] = alloc_block()?;
    }
    for i in 0..blocks_needed {
        let mut block = [0u8; BLOCK_SIZE];
        let start = i * BLOCK_SIZE;
        let end = core::cmp::min(start + BLOCK_SIZE, data.len());
        if start < end {
            block[..end - start].copy_from_slice(&data[start..end]);
        }
        write_block(block_ids[i] as usize, &block).map_err(|_| ())?;
    }
    for i in blocks_needed..used {
        if block_ids[i] != 0 {
            free_block(block_ids[i])?;
            block_ids[i] = 0;
        }
    }
    write_regular_inode(ino, data.len() as u32, &block_ids)
}

fn write_regular_inode(ino: u32, size: u32, blocks: &[u32; 12]) -> Result<(), ()> {
    if ino == 0 {
        return Err(());
    }
    let index = (ino - 1) as usize;
    let block = INODE_TABLE_BLOCK + (index * 128) / BLOCK_SIZE;
    let offset = (index * 128) % BLOCK_SIZE;
    let mut data = read_block(block)?;
    if offset + 128 > data.len() {
        return Err(());
    }
    write_u16(&mut data, offset, 0x8000);
    write_u32(&mut data, offset + 4, size);
    for (idx, block_id) in blocks.iter().enumerate() {
        write_u32(&mut data, offset + 40 + idx * 4, *block_id);
    }
    write_block(block, &data).map_err(|_| ())
}

fn append_dir_entry(parent_ino: u32, name: &str, child_ino: u32) -> Result<(), ()> {
    let parent = read_inode(parent_ino)?;
    if parent.blocks[0] == 0 {
        return Err(());
    }
    let name_bytes = name.as_bytes();
    if name_bytes.is_empty() || name_bytes.len() > 255 {
        return Err(());
    }
    let rec_len = ((8 + name_bytes.len() + 3) / 4) * 4;
    let mut dir = read_block(parent.blocks[0] as usize)?;
    let mut offset = 0usize;
    let mut insert_at = 0usize;
    while offset + 8 <= dir.len() {
        let inode = u32::from_le_bytes([
            dir[offset],
            dir[offset + 1],
            dir[offset + 2],
            dir[offset + 3],
        ]);
        let entry_len = u16::from_le_bytes([dir[offset + 4], dir[offset + 5]]) as usize;
        if entry_len < 8 {
            break;
        }
        if inode == 0 {
            break;
        }
        insert_at = offset + entry_len;
        offset = insert_at;
    }
    if insert_at + rec_len > BLOCK_SIZE {
        return Err(());
    }
    dir[insert_at..insert_at + 4].copy_from_slice(&child_ino.to_le_bytes());
    dir[insert_at + 4..insert_at + 6].copy_from_slice(&(rec_len as u16).to_le_bytes());
    dir[insert_at + 6] = name_bytes.len() as u8;
    dir[insert_at + 7] = 1;
    dir[insert_at + 8..insert_at + 8 + name_bytes.len()].copy_from_slice(name_bytes);
    write_block(parent.blocks[0] as usize, &dir).map_err(|_| ())?;
    patch_inode_size(parent_ino, (insert_at + rec_len) as u32)
}

fn remove_dir_entry(name: &str) -> Result<u32, ()> {
    let root = read_inode(ROOT_INO)?;
    if root.blocks[0] == 0 {
        return Err(());
    }
    let mut dir = read_block(root.blocks[0] as usize)?;
    let mut offset = 0usize;
    while offset + 8 <= dir.len() {
        let inode = u32::from_le_bytes([
            dir[offset],
            dir[offset + 1],
            dir[offset + 2],
            dir[offset + 3],
        ]);
        let rec_len = u16::from_le_bytes([dir[offset + 4], dir[offset + 5]]) as usize;
        if rec_len < 8 {
            break;
        }
        let name_len = dir[offset + 6] as usize;
        let start = offset + 8;
        let end = start.saturating_add(name_len);
        if end <= dir.len() {
            if let Ok(entry_name) = core::str::from_utf8(&dir[start..end]) {
                if entry_name == name {
                    if inode == 0 {
                        return Err(());
                    }
                    dir[offset..offset + 4].copy_from_slice(&0u32.to_le_bytes());
                    write_block(root.blocks[0] as usize, &dir).map_err(|_| ())?;
                    return Ok(inode);
                }
            }
        }
        offset = offset.saturating_add(rec_len);
    }
    Err(())
}

fn alloc_block() -> Result<u32, ()> {
    let mut bitmap = read_block(BLOCK_BITMAP_BLOCK)?;
    for block in 1..TOTAL_BLOCKS {
        let byte = block / 8;
        let bit = block % 8;
        if bitmap[byte] & (1 << bit) == 0 {
            bitmap[byte] |= 1 << bit;
            write_block(BLOCK_BITMAP_BLOCK, &bitmap).map_err(|_| ())?;
            return Ok(block as u32);
        }
    }
    Err(())
}

fn free_block(block: u32) -> Result<(), ()> {
    if block == 0 {
        return Err(());
    }
    let block = block as usize;
    if block >= TOTAL_BLOCKS {
        return Err(());
    }
    let mut bitmap = read_block(BLOCK_BITMAP_BLOCK)?;
    let byte = block / 8;
    let bit = block % 8;
    bitmap[byte] &= !(1 << bit);
    write_block(BLOCK_BITMAP_BLOCK, &bitmap).map_err(|_| ())
}

fn alloc_inode() -> Result<u32, ()> {
    let mut bitmap = read_block(INODE_BITMAP_BLOCK)?;
    for ino in 1..=128u32 {
        let idx = (ino - 1) as usize;
        let byte = idx / 8;
        let bit = idx % 8;
        if bitmap[byte] & (1 << bit) == 0 {
            bitmap[byte] |= 1 << bit;
            write_block(INODE_BITMAP_BLOCK, &bitmap).map_err(|_| ())?;
            return Ok(ino);
        }
    }
    Err(())
}

fn free_inode(ino: u32) -> Result<(), ()> {
    if ino == 0 {
        return Err(());
    }
    let idx = (ino - 1) as usize;
    let byte = idx / 8;
    let bit = idx % 8;
    let mut bitmap = read_block(INODE_BITMAP_BLOCK)?;
    bitmap[byte] &= !(1 << bit);
    write_block(INODE_BITMAP_BLOCK, &bitmap).map_err(|_| ())?;
    let cleared = [0u8; 128];
    write_inode(ino, &cleared);
    Ok(())
}

fn patch_inode_size(ino: u32, size: u32) -> Result<(), ()> {
    if ino == 0 {
        return Err(());
    }
    let index = (ino - 1) as usize;
    let block = INODE_TABLE_BLOCK + (index * 128) / BLOCK_SIZE;
    let offset = (index * 128) % BLOCK_SIZE;
    let mut data = read_block(block)?;
    if offset + 8 > data.len() {
        return Err(());
    }
    data[offset + 4..offset + 8].copy_from_slice(&size.to_le_bytes());
    write_block(block, &data).map_err(|_| ())
}

fn find_dir_entry(data: &[u8], name: &str) -> Option<u32> {
    let mut offset = 0usize;
    while offset + 8 <= data.len() {
        let inode = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        let rec_len = u16::from_le_bytes([data[offset + 4], data[offset + 5]]) as usize;
        if rec_len < 8 || rec_len == 0 {
            break;
        }
        let name_len = data[offset + 6] as usize;
        let start = offset + 8;
        let end = start.saturating_add(name_len);
        if end <= data.len() {
            if let Ok(entry_name) = core::str::from_utf8(&data[start..end]) {
                if entry_name == name {
                    return Some(inode);
                }
            }
        }
        offset = offset.saturating_add(rec_len);
    }
    None
}

struct Inode {
    mode: u16,
    size: u32,
    blocks: [u32; 12],
}

fn read_inode(ino: u32) -> Result<Inode, ()> {
    if ino == 0 {
        return Err(());
    }
    let index = (ino - 1) as usize;
    let block = INODE_TABLE_BLOCK + (index * 128) / BLOCK_SIZE;
    let offset = (index * 128) % BLOCK_SIZE;
    let data = read_block(block)?;
    if offset + 128 > data.len() {
        return Err(());
    }
    let slice = &data[offset..offset + 128];
    let mut blocks = [0u32; 12];
    for (idx, slot) in blocks.iter_mut().enumerate() {
        let start = 40 + idx * 4;
        *slot = u32::from_le_bytes([
            slice[start],
            slice[start + 1],
            slice[start + 2],
            slice[start + 3],
        ]);
    }
    Ok(Inode {
        mode: u16::from_le_bytes([slice[0], slice[1]]),
        size: u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]),
        blocks,
    })
}

fn read_inode_bytes(inode: &Inode) -> Result<Vec<u8>, ()> {
    read_file_data(inode)
}

fn read_file_data(inode: &Inode) -> Result<Vec<u8>, ()> {
    let len = inode.size as usize;
    let mut out = vec![0u8; len];
    let mut copied = 0usize;
    for block_id in inode.blocks.iter().copied() {
        if copied >= len || block_id == 0 {
            break;
        }
        let data = read_block(block_id as usize)?;
        let take = core::cmp::min(BLOCK_SIZE, len - copied);
        out[copied..copied + take].copy_from_slice(&data[..take]);
        copied += take;
    }
    Ok(out)
}

fn read_block(block: usize) -> Result<[u8; BLOCK_SIZE], ()> {
    let mut out = [0u8; BLOCK_SIZE];
    let base_sector = block * SECTORS_PER_BLOCK;
    for (idx, chunk) in out.chunks_mut(SECTOR_SIZE).enumerate() {
        let mut sector = [0u8; SECTOR_SIZE];
        block::read_active_sector(base_sector + idx, &mut sector)
            .map_err(|_| ())?;
        chunk.copy_from_slice(&sector);
    }
    Ok(out)
}

fn write_block(block: usize, data: &[u8; BLOCK_SIZE]) -> Result<(), BlockError> {
    let base_sector = block * SECTORS_PER_BLOCK;
    for (idx, chunk) in data.chunks(SECTOR_SIZE).enumerate() {
        let mut sector = [0u8; SECTOR_SIZE];
        sector.copy_from_slice(chunk);
        block::write_active_sector(base_sector + idx, &sector)?;
    }
    Ok(())
}

fn format_minimal() {
    let elf_bytes = crate::embedded_ring3_io_demo::elf_bytes();
    let mut superblock = [0u8; BLOCK_SIZE];
    write_u32(&mut superblock, 0x00, 32);
    write_u32(&mut superblock, 0x04, TOTAL_BLOCKS as u32);
    write_u32(&mut superblock, 0x10, 0);
    write_u16(&mut superblock, 0x38, 0xEF53);
    write_u16(&mut superblock, 0x54, 128);
    let _ = write_block(SUPERBLOCK_BLOCK, &superblock);

    let mut bg = [0u8; BLOCK_SIZE];
    write_u32(&mut bg, 0x00, BLOCK_BITMAP_BLOCK as u32);
    write_u32(&mut bg, 0x04, INODE_BITMAP_BLOCK as u32);
    write_u32(&mut bg, 0x08, INODE_TABLE_BLOCK as u32);
    write_u16(&mut bg, 0x0C, 0xFFFE);
    write_u16(&mut bg, 0x0E, 0xFFFE);
    write_u16(&mut bg, 0x10, 0x003E);
    write_u16(&mut bg, 0x12, 0x0001);
    let _ = write_block(BG_DESC_BLOCK, &bg);

    let mut block_bitmap = [0u8; BLOCK_SIZE];
    for block in [
        SUPERBLOCK_BLOCK,
        BG_DESC_BLOCK,
        BLOCK_BITMAP_BLOCK,
        INODE_BITMAP_BLOCK,
        INODE_TABLE_BLOCK,
        INODE_TABLE_BLOCK + 1,
        INODE_TABLE_BLOCK + 2,
        INODE_TABLE_BLOCK + 3,
        ROOT_DIR_BLOCK as usize,
        FILE_DATA_BLOCK as usize,
        ELF_DATA_BLOCK0 as usize,
        ELF_DATA_BLOCK0 as usize + 1,
        ELF_DATA_BLOCK0 as usize + 2,
        ELF_DATA_BLOCK0 as usize + 3,
    ] {
        let byte = block / 8;
        let bit = block % 8;
        block_bitmap[byte] |= 1 << bit;
    }
    let _ = write_block(BLOCK_BITMAP_BLOCK, &block_bitmap);

    let mut inode_bitmap = [0u8; BLOCK_SIZE];
    inode_bitmap[0] = 0b0000_0010;
    inode_bitmap[1] = 0b0001_1000;
    let _ = write_block(INODE_BITMAP_BLOCK, &inode_bitmap);

    let mut root = [0u8; 128];
    write_u16(&mut root, 0x00, 0x4000);
    write_u32(&mut root, 0x04, 48);
    write_u32(&mut root, 0x28, ROOT_DIR_BLOCK);
    write_inode(ROOT_INO, &root);

    let mut dir_block = [0u8; BLOCK_SIZE];
    let smoke_name = b"smoke.txt";
    dir_block[0..4].copy_from_slice(&FILE_INO.to_le_bytes());
    dir_block[4..6].copy_from_slice(&20u16.to_le_bytes());
    dir_block[6] = smoke_name.len() as u8;
    dir_block[7] = 1;
    dir_block[8..8 + smoke_name.len()].copy_from_slice(smoke_name);
    let elf_name = b"ring3-io-demo.elf";
    let elf_dentry_off = 20usize;
    dir_block[elf_dentry_off..elf_dentry_off + 4].copy_from_slice(&ELF_INO.to_le_bytes());
    dir_block[elf_dentry_off + 4..elf_dentry_off + 6].copy_from_slice(&28u16.to_le_bytes());
    dir_block[elf_dentry_off + 6] = elf_name.len() as u8;
    dir_block[elf_dentry_off + 7] = 1;
    dir_block[elf_dentry_off + 8..elf_dentry_off + 8 + elf_name.len()].copy_from_slice(elf_name);
    let _ = write_block(ROOT_DIR_BLOCK as usize, &dir_block);

    let mut file = [0u8; 128];
    write_u16(&mut file, 0x00, 0x8000);
    write_u32(&mut file, 0x04, 8);
    write_u32(&mut file, 0x28, FILE_DATA_BLOCK);
    write_inode(FILE_INO, &file);

    let mut payload = [0u8; BLOCK_SIZE];
    payload[..8].copy_from_slice(b"ext2 ok\n");
    let _ = write_block(FILE_DATA_BLOCK as usize, &payload);

    let mut elf_inode = [0u8; 128];
    write_u16(&mut elf_inode, 0x00, 0x8000);
    write_u32(
        &mut elf_inode,
        0x04,
        elf_bytes.len().min(ELF_DATA_BLOCKS * BLOCK_SIZE) as u32,
    );
    for (idx, block_id) in (0..ELF_DATA_BLOCKS).map(|i| (i, ELF_DATA_BLOCK0 + i as u32)) {
        let off = 40 + idx * 4;
        write_u32(&mut elf_inode, off, block_id);
    }
    write_inode(ELF_INO, &elf_inode);

    for block_idx in 0..ELF_DATA_BLOCKS {
        let mut block = [0u8; BLOCK_SIZE];
        let start = block_idx * BLOCK_SIZE;
        if start < elf_bytes.len() {
            let end = core::cmp::min(start + BLOCK_SIZE, elf_bytes.len());
            block[..end - start].copy_from_slice(&elf_bytes[start..end]);
        }
        let _ = write_block(ELF_DATA_BLOCK0 as usize + block_idx, &block);
    }
}

fn write_inode(ino: u32, bytes: &[u8; 128]) {
    let index = (ino - 1) as usize;
    let block = INODE_TABLE_BLOCK + (index * 128) / BLOCK_SIZE;
    let offset = (index * 128) % BLOCK_SIZE;
    let mut data = read_block(block).unwrap_or([0u8; BLOCK_SIZE]);
    data[offset..offset + 128].copy_from_slice(bytes);
    let _ = write_block(block, &data);
}

fn write_u16(out: &mut [u8], offset: usize, value: u16) {
    out[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(out: &mut [u8], offset: usize, value: u32) {
    out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

pub fn smoke_ext2_read() -> bool {
    init()
        && read_file("smoke.txt")
            .map(|bytes| bytes.starts_with(b"ext2 ok"))
            .unwrap_or(false)
        && read_file("ring3-io-demo.elf")
            .map(|bytes| bytes.len() > 256 && bytes.starts_with(b"\x7fELF"))
            .unwrap_or(false)
}

pub fn smoke_ext2_write() -> bool {
    if !init() {
        return false;
    }
    let reject_elf = write_file("ring3-io-demo.elf", b"bad").is_err();
    let wrote = write_file("smoke.txt", b"ext2 wr\n").is_ok();
    let read_back = read_file("smoke.txt")
        .map(|bytes| bytes.as_slice() == b"ext2 wr\n")
        .unwrap_or(false);
    let elf_intact = read_file("ring3-io-demo.elf")
        .map(|bytes| bytes.len() > 256 && bytes.starts_with(b"\x7fELF"))
        .unwrap_or(false);
    reject_elf && wrote && read_back && elf_intact && write_count() > 0
}

pub fn smoke_ext2_create_unlink() -> bool {
    if !init() {
        return false;
    }
    let absent = read_file("scratch.txt").is_err();
    let created = create_file("scratch.txt", b"new file\n").is_ok();
    let read_ok = read_file("scratch.txt")
        .map(|bytes| bytes.as_slice() == b"new file\n")
        .unwrap_or(false);
    let big = vec![b'X'; BLOCK_SIZE + 512];
    let grew = write_file("scratch.txt", &big).is_ok();
    let big_read = read_file("scratch.txt")
        .map(|bytes| bytes.len() == BLOCK_SIZE + 512)
        .unwrap_or(false);
    let unlinked = unlink_file("scratch.txt").is_ok();
    let gone = read_file("scratch.txt").is_err();
    let protect_smoke = unlink_file("smoke.txt").is_err();
    let protect_elf = unlink_file("ring3-io-demo.elf").is_err();
    let smoke_intact = read_file("smoke.txt")
        .map(|bytes| !bytes.is_empty())
        .unwrap_or(false);
    absent
        && created
        && read_ok
        && grew
        && big_read
        && unlinked
        && gone
        && protect_smoke
        && protect_elf
        && smoke_intact
        && create_count() > 0
        && unlink_count() > 0
}
