//! frame-backed executable image records.

use alloc::{string::String, vec::Vec};

use crate::{
    address_space::{AddressSpaceId, MappingState},
    frame_ownership::{self, FrameOwner, OwnedFrame, OwnedFrameToken},
    load_plan::{LoadPermissions, PAGE_SIZE},
    mapping_stub::{MappedImage, MappingActionKind, MappingId},
    security::Credentials,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameBackedPage {
    pub virtual_address: u64,
    pub frame: OwnedFrame,
    pub permissions: LoadPermissions,
    pub copied_bytes: usize,
    pub zero_filled_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameBackedRegion {
    pub start: u64,
    pub size: usize,
    pub permissions: LoadPermissions,
    pub pages: Vec<FrameBackedPage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameBackedImage {
    pub mapping_id: MappingId,
    pub image_name: String,
    pub source_path: String,
    pub address_space_id: AddressSpaceId,
    pub regions: Vec<FrameBackedRegion>,
    pub total_pages: usize,
    pub executable_pages: usize,
    pub writable_pages: usize,
    pub read_only_pages: usize,
    pub copied_bytes: usize,
    pub zero_filled_bytes: usize,
    pub owner: Credentials,
    pub state: MappingState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameBackingError {
    EmptyMapping,
    FrameUnavailable,
    ReleaseFailed,
}

pub fn back_mapped_image(mapped: &MappedImage) -> Result<FrameBackedImage, FrameBackingError> {
    if mapped.total_pages == 0 || mapped.regions.is_empty() {
        return Err(FrameBackingError::EmptyMapping);
    }

    let mut allocated = Vec::new();
    let mut regions = Vec::new();
    for region in &mapped.regions {
        let mut pages = Vec::new();
        for page in &region.pages {
            let frame = match frame_ownership::allocate_frame(FrameOwner::Image) {
                Ok(frame) => frame,
                Err(_) => {
                    release_allocated(&allocated)?;
                    return Err(FrameBackingError::FrameUnavailable);
                }
            };
            allocated.push(frame.token);
            let (copied_bytes, zero_filled_bytes) =
                bytes_for_page(page.virtual_address, &region.actions);
            pages.push(FrameBackedPage {
                virtual_address: page.virtual_address,
                frame,
                permissions: page.permissions,
                copied_bytes,
                zero_filled_bytes,
            });
        }
        regions.push(FrameBackedRegion {
            start: region.start,
            size: region.size,
            permissions: region.permissions,
            pages,
        });
    }

    Ok(FrameBackedImage {
        mapping_id: mapped.id,
        image_name: mapped.image_name.clone(),
        source_path: mapped.source_path.clone(),
        address_space_id: mapped.address_space_id,
        regions,
        total_pages: mapped.total_pages,
        executable_pages: mapped.executable_pages,
        writable_pages: mapped.writable_pages,
        read_only_pages: mapped.read_only_pages,
        copied_bytes: mapped.copied_bytes,
        zero_filled_bytes: mapped.zero_filled_bytes,
        owner: mapped.owner,
        state: MappingState::FrameBacked,
    })
}

fn release_allocated(tokens: &[OwnedFrameToken]) -> Result<(), FrameBackingError> {
    for token in tokens {
        frame_ownership::release_frame(*token).map_err(|_| FrameBackingError::ReleaseFailed)?;
    }
    Ok(())
}

fn bytes_for_page(
    page_start: u64,
    actions: &[crate::mapping_stub::MappingActionResult],
) -> (usize, usize) {
    let page_end = page_start.saturating_add(PAGE_SIZE as u64);
    let mut copied = 0usize;
    let mut zeroed = 0usize;
    for action in actions {
        let action_start = action.target_address;
        let action_end = action.target_address.saturating_add(action.len as u64);
        let overlap_start = core::cmp::max(page_start, action_start);
        let overlap_end = core::cmp::min(page_end, action_end);
        if overlap_end <= overlap_start {
            continue;
        }
        let overlap = (overlap_end - overlap_start) as usize;
        match action.kind {
            MappingActionKind::Copy => copied = copied.saturating_add(overlap),
            MappingActionKind::ZeroFill => zeroed = zeroed.saturating_add(overlap),
        }
    }
    (copied, zeroed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn page_action_accounting_splits_by_page() {
        let actions = [
            crate::mapping_stub::MappingActionResult {
                target_address: 0x1000,
                len: 4,
                kind: MappingActionKind::Copy,
            },
            crate::mapping_stub::MappingActionResult {
                target_address: 0x1004,
                len: 4092,
                kind: MappingActionKind::ZeroFill,
            },
        ];
        assert_eq!(bytes_for_page(0x1000, &actions), (4, 4092));
        assert_eq!(bytes_for_page(0x2000, &actions), (0, 0));
    }
}
