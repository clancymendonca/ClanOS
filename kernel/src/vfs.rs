//! Virtual filesystem facade — routes paths to Clan OS primary FS (CLANFS1) or ext2.

use alloc::vec::Vec;

use crate::security::Credentials;

pub const EXT2_MOUNT_PREFIX: &str = "/ext2/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    InvalidPath,
    Backend,
}

pub fn read_bytes(path: &str) -> Result<Option<Vec<u8>>, VfsError> {
    if let Some(relative) = path.strip_prefix(EXT2_MOUNT_PREFIX) {
        return crate::ext2::read_file(relative)
            .map(Some)
            .map_err(|_| VfsError::Backend);
    }
    crate::storage::read_file_bytes(path)
        .map_err(|_| VfsError::Backend)
}

/// Read file bytes for mmap/FD paths: ext2, CLANFS1 binary, then cred-checked text storage.
pub fn read_bytes_for(creds: Credentials, path: &str) -> Result<Option<Vec<u8>>, VfsError> {
    if let Some(relative) = path.strip_prefix(EXT2_MOUNT_PREFIX) {
        return crate::ext2::read_file(relative)
            .map(Some)
            .map_err(|_| VfsError::Backend);
    }
    if let Ok(Some(bytes)) = read_bytes(path) {
        return Ok(Some(bytes));
    }
    crate::storage::read_file_checked(creds, path)
        .map(|opt| opt.map(|text| text.into_bytes()))
        .map_err(|_| VfsError::Backend)
}

pub fn write_bytes(path: &str, bytes: &[u8]) -> Result<(), VfsError> {
    if let Some(relative) = path.strip_prefix(EXT2_MOUNT_PREFIX) {
        return crate::ext2::write_file(relative, bytes).map_err(|_| VfsError::Backend);
    }
    crate::storage::write_file_bytes_checked(
        crate::security::Credentials::admin(),
        path,
        bytes,
    )
    .map_err(|_| VfsError::Backend)
}

pub fn create_bytes(path: &str, bytes: &[u8]) -> Result<(), VfsError> {
    if let Some(relative) = path.strip_prefix(EXT2_MOUNT_PREFIX) {
        return crate::ext2::create_file(relative, bytes).map_err(|_| VfsError::Backend);
    }
    crate::storage::create_file_checked(crate::security::Credentials::admin(), path)
        .map(|_| ())
        .map_err(|_| VfsError::Backend)
}

pub fn unlink_path(path: &str) -> Result<(), VfsError> {
    if let Some(relative) = path.strip_prefix(EXT2_MOUNT_PREFIX) {
        return crate::ext2::unlink_file(relative).map_err(|_| VfsError::Backend);
    }
    crate::storage::delete_file_checked(crate::security::Credentials::admin(), path)
        .map_err(|_| VfsError::Backend)
}

pub fn list_mounts() -> &'static [&'static str] {
    &["clanfs:/", "ext2:/ext2/"]
}

pub fn smoke_vfs_mount() -> bool {
    crate::storage::is_mounted()
        && crate::ext2::is_mounted()
        && read_bytes("/README.txt")
            .ok()
            .flatten()
            .is_some()
        && read_bytes("/ext2/smoke.txt")
            .ok()
            .flatten()
            .map(|bytes| bytes.starts_with(b"ext2 ok"))
            .unwrap_or(false)
}
