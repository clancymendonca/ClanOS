//! Phase 8 block-device manager.

use alloc::{string::String, vec, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{
    device::{self, DeviceKind, DeviceState},
    storage::{StorageError, SECTOR_SIZE},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeviceId(u64);

impl BlockDeviceId {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockBackendKind {
    MemoryFallback,
    SimulatedQemu,
    VirtioBlk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDeviceInfo {
    pub id: BlockDeviceId,
    pub name: String,
    pub backend: BlockBackendKind,
    pub sector_size: usize,
    pub sector_count: usize,
    pub read_only: bool,
    pub driver_backed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    NotFound,
    NoActiveDevice,
    OutOfRange,
    ReadOnly,
}

struct BlockDeviceEntry {
    info: BlockDeviceInfo,
    media: Vec<[u8; SECTOR_SIZE]>,
}

struct BlockManager {
    next_id: u64,
    active: Option<BlockDeviceId>,
    primary: Option<BlockDeviceId>,
    devices: Vec<BlockDeviceEntry>,
}

impl BlockManager {
    fn new() -> Self {
        Self {
            next_id: 1,
            active: None,
            primary: None,
            devices: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.next_id = 1;
        self.active = None;
        self.primary = None;
        self.devices.clear();
    }

    fn register(
        &mut self,
        name: &str,
        backend: BlockBackendKind,
        sector_count: usize,
        read_only: bool,
        driver_backed: bool,
    ) -> BlockDeviceId {
        let id = BlockDeviceId(self.next_id);
        self.next_id += 1;
        let info = BlockDeviceInfo {
            id,
            name: String::from(name),
            backend,
            sector_size: SECTOR_SIZE,
            sector_count,
            read_only,
            driver_backed,
        };
        self.devices.push(BlockDeviceEntry {
            info,
            media: vec![[0; SECTOR_SIZE]; sector_count],
        });
        if self.primary.is_none() {
            self.primary = Some(id);
        }
        if self.active.is_none() {
            self.active = Some(id);
        }
        id
    }

    fn active_entry(&self) -> Result<&BlockDeviceEntry, BlockError> {
        let id = self.active.ok_or(BlockError::NoActiveDevice)?;
        self.devices
            .iter()
            .find(|entry| entry.info.id == id)
            .ok_or(BlockError::NotFound)
    }

    fn active_entry_mut(&mut self) -> Result<&mut BlockDeviceEntry, BlockError> {
        let id = self.active.ok_or(BlockError::NoActiveDevice)?;
        self.devices
            .iter_mut()
            .find(|entry| entry.info.id == id)
            .ok_or(BlockError::NotFound)
    }
}

lazy_static! {
    static ref MANAGER: Mutex<BlockManager> = Mutex::new(BlockManager::new());
}

pub fn init() {
    let mut manager = MANAGER.lock();
    manager.clear();
    manager.register(
        "qemu-sim-block0",
        BlockBackendKind::SimulatedQemu,
        crate::storage::DEFAULT_SECTOR_COUNT,
        false,
        true,
    );
    drop(manager);
    device::register_device(
        "qemu-sim-block0",
        DeviceKind::Block,
        DeviceState::Ready,
        None,
        None,
        Some(0x01),
        Some(0x00),
        None,
    );
}

pub fn register_virtio_blk(name: &str, sector_count: usize, read_only: bool) -> BlockDeviceId {
    MANAGER.lock().register(
        name,
        BlockBackendKind::VirtioBlk,
        sector_count,
        read_only,
        true,
    )
}

pub fn register_memory_fallback(sector_count: usize) -> BlockDeviceId {
    let id = MANAGER.lock().register(
        "memory-fallback-block0",
        BlockBackendKind::MemoryFallback,
        sector_count,
        false,
        false,
    );
    device::register_device(
        "memory-fallback-block0",
        DeviceKind::Block,
        DeviceState::Ready,
        None,
        None,
        Some(0x01),
        Some(0x00),
        None,
    );
    id
}

pub fn primary_id() -> Option<BlockDeviceId> {
    MANAGER.lock().primary
}

pub fn set_active(id: BlockDeviceId) -> Result<(), BlockError> {
    let mut manager = MANAGER.lock();
    if manager.devices.iter().any(|entry| entry.info.id == id) {
        manager.active = Some(id);
        Ok(())
    } else {
        Err(BlockError::NotFound)
    }
}

pub fn list_block_devices() -> Vec<BlockDeviceInfo> {
    MANAGER
        .lock()
        .devices
        .iter()
        .map(|entry| entry.info.clone())
        .collect()
}

pub fn active_info() -> Result<BlockDeviceInfo, BlockError> {
    MANAGER
        .lock()
        .active_entry()
        .map(|entry| entry.info.clone())
}

pub fn active_sector_count() -> Result<usize, BlockError> {
    MANAGER
        .lock()
        .active_entry()
        .map(|entry| entry.info.sector_count)
}

pub fn read_active_sector(sector: usize, buffer: &mut [u8; SECTOR_SIZE]) -> Result<(), BlockError> {
    let manager = MANAGER.lock();
    let entry = manager.active_entry()?;
    let source = entry.media.get(sector).ok_or(BlockError::OutOfRange)?;
    buffer.copy_from_slice(source);
    Ok(())
}

pub fn write_active_sector(sector: usize, buffer: &[u8; SECTOR_SIZE]) -> Result<(), BlockError> {
    let mut manager = MANAGER.lock();
    let entry = manager.active_entry_mut()?;
    if entry.info.read_only {
        return Err(BlockError::ReadOnly);
    }
    let target = entry.media.get_mut(sector).ok_or(BlockError::OutOfRange)?;
    target.copy_from_slice(buffer);
    Ok(())
}

pub fn summary() -> (usize, usize, &'static str) {
    let devices = list_block_devices();
    let driver_backed = devices.iter().filter(|device| device.driver_backed).count();
    let backend = active_backend_name();
    (devices.len(), driver_backed, backend)
}

pub fn active_backend_name() -> &'static str {
    match active_info().map(|info| info.backend) {
        Ok(BlockBackendKind::SimulatedQemu) => "qemu-sim-block0",
        Ok(BlockBackendKind::MemoryFallback) => "memory-fallback-block0",
        Ok(BlockBackendKind::VirtioBlk) => "virtio-blk0",
        Err(_) => "none",
    }
}

impl From<BlockError> for StorageError {
    fn from(_: BlockError) -> Self {
        StorageError::Io
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn manager_registers_and_lists_block_devices() {
        init();
        let devices = list_block_devices();
        assert!(!devices.is_empty());
        assert_eq!(devices[0].sector_size, SECTOR_SIZE);
    }

    #[test_case]
    fn manager_rejects_unknown_active_device() {
        init();
        assert_eq!(
            set_active(BlockDeviceId::from_raw(999)),
            Err(BlockError::NotFound)
        );
    }

    #[test_case]
    fn active_device_reads_and_writes_sectors() {
        init();
        let mut write = [0; SECTOR_SIZE];
        write[0] = 99;
        write_active_sector(1, &write).expect("write should succeed");

        let mut read = [0; SECTOR_SIZE];
        read_active_sector(1, &mut read).expect("read should succeed");
        assert_eq!(read[0], 99);
    }
}
