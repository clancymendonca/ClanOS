//! Phase 8 device registry and PCI discovery skeleton.

use alloc::{string::String, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceId(u64);

impl DeviceId {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    Pci,
    Block,
    Storage,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Discovered,
    Ready,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub id: DeviceId,
    pub name: String,
    pub kind: DeviceKind,
    pub state: DeviceState,
    pub vendor_id: Option<u16>,
    pub device_id: Option<u16>,
    pub class_code: Option<u8>,
    pub subclass: Option<u8>,
    pub bus: Option<u8>,
    pub slot: Option<u8>,
    pub function: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceSummary {
    pub total: usize,
    pub pci: usize,
    pub block: usize,
    pub storage: usize,
}

struct DeviceRegistry {
    next_id: u64,
    devices: Vec<DeviceInfo>,
}

impl DeviceRegistry {
    fn new() -> Self {
        Self {
            next_id: 1,
            devices: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.next_id = 1;
        self.devices.clear();
    }

    fn register(&mut self, mut info: DeviceInfo) -> DeviceId {
        let id = DeviceId(self.next_id);
        self.next_id += 1;
        info.id = id;
        self.devices.push(info);
        id
    }
}

lazy_static! {
    static ref REGISTRY: Mutex<DeviceRegistry> = Mutex::new(DeviceRegistry::new());
}

pub fn init() {
    let mut registry = REGISTRY.lock();
    registry.clear();
    drop(registry);
    scan_pci();
}

pub fn register_device(
    name: &str,
    kind: DeviceKind,
    state: DeviceState,
    vendor_id: Option<u16>,
    device_id: Option<u16>,
    class_code: Option<u8>,
    subclass: Option<u8>,
    location: Option<(u8, u8, u8)>,
) -> DeviceId {
    let (bus, slot, function) = location
        .map(|(bus, slot, function)| (Some(bus), Some(slot), Some(function)))
        .unwrap_or((None, None, None));
    REGISTRY.lock().register(DeviceInfo {
        id: DeviceId::from_raw(0),
        name: String::from(name),
        kind,
        state,
        vendor_id,
        device_id,
        class_code,
        subclass,
        bus,
        slot,
        function,
    })
}

pub fn list_devices() -> Vec<DeviceInfo> {
    REGISTRY.lock().devices.clone()
}

pub fn get_device(id: DeviceId) -> Result<DeviceInfo, DeviceError> {
    REGISTRY
        .lock()
        .devices
        .iter()
        .find(|device| device.id == id)
        .cloned()
        .ok_or(DeviceError::NotFound)
}

pub fn summary() -> DeviceSummary {
    let registry = REGISTRY.lock();
    DeviceSummary {
        total: registry.devices.len(),
        pci: registry
            .devices
            .iter()
            .filter(|device| device.kind == DeviceKind::Pci)
            .count(),
        block: registry
            .devices
            .iter()
            .filter(|device| device.kind == DeviceKind::Block)
            .count(),
        storage: registry
            .devices
            .iter()
            .filter(|device| device.kind == DeviceKind::Storage)
            .count(),
    }
}

pub fn scan_pci() {
    let mut discovered = 0usize;
    for bus in 0u8..=0 {
        for slot in 0u8..32 {
            for function in 0u8..8 {
                let vendor_id = pci_config_read_u16(bus, slot, function, 0x00);
                if vendor_id == 0xffff {
                    continue;
                }
                let device_id = pci_config_read_u16(bus, slot, function, 0x02);
                let class_code = pci_config_read_u8(bus, slot, function, 0x0b);
                let subclass = pci_config_read_u8(bus, slot, function, 0x0a);
                let name = classify_pci_device(class_code, subclass);
                register_device(
                    name,
                    DeviceKind::Pci,
                    DeviceState::Discovered,
                    Some(vendor_id),
                    Some(device_id),
                    Some(class_code),
                    Some(subclass),
                    Some((bus, slot, function)),
                );
                discovered += 1;
            }
        }
    }

    if discovered == 0 {
        register_device(
            "pci-scan-empty",
            DeviceKind::Unknown,
            DeviceState::Discovered,
            None,
            None,
            None,
            None,
            None,
        );
    }
}

fn classify_pci_device(class_code: u8, subclass: u8) -> &'static str {
    match (class_code, subclass) {
        (0x01, 0x01) => "pci-ide-controller",
        (0x01, 0x06) => "pci-sata-controller",
        (0x01, 0x08) => "pci-nvme-controller",
        (0x02, _) => "pci-network-controller",
        (0x03, _) => "pci-display-controller",
        (0x06, 0x00) => "pci-host-bridge",
        (0x06, 0x01) => "pci-isa-bridge",
        _ => "pci-device",
    }
}

fn pci_config_read_u8(bus: u8, slot: u8, function: u8, offset: u8) -> u8 {
    let shift = (offset & 3) * 8;
    ((pci_config_read_u32(bus, slot, function, offset & !3) >> shift) & 0xff) as u8
}

fn pci_config_read_u16(bus: u8, slot: u8, function: u8, offset: u8) -> u16 {
    let shift = (offset & 2) * 8;
    ((pci_config_read_u32(bus, slot, function, offset & !3) >> shift) & 0xffff) as u16
}

fn pci_config_read_u32(bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
    use x86_64::instructions::port::Port;

    let address = 0x8000_0000u32
        | ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xfc);
    unsafe {
        let mut address_port = Port::<u32>::new(0xcf8);
        let mut data_port = Port::<u32>::new(0xcfc);
        address_port.write(address);
        data_port.read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn registry_registers_and_queries_devices() {
        REGISTRY.lock().clear();
        let id = register_device(
            "test-device",
            DeviceKind::Storage,
            DeviceState::Ready,
            None,
            None,
            None,
            None,
            None,
        );
        let device = get_device(id).expect("device should be registered");
        assert_eq!(device.name, "test-device");
        assert_eq!(summary().storage, 1);
    }

    #[test_case]
    fn missing_device_returns_not_found() {
        REGISTRY.lock().clear();
        assert_eq!(
            get_device(DeviceId::from_raw(999)),
            Err(DeviceError::NotFound)
        );
    }
}
