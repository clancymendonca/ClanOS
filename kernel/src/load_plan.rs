//! Phase 12 executable load-plan model.

use alloc::vec::Vec;

use crate::exec_image::{ExecutableFormat, ExecutableImage, SegmentFlags};

pub const PAGE_SIZE: usize = 4096;
pub const MAX_LOAD_REGIONS: usize = 4;
pub const MAX_IMAGE_PAGES: usize = 16;
pub const STACK_RESERVATION_PAGES: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadPermissions {
    bits: u8,
}

impl LoadPermissions {
    pub const READ: u8 = 0b001;
    pub const WRITE: u8 = 0b010;
    pub const EXECUTE: u8 = 0b100;

    pub const fn from_bits(bits: u8) -> Self {
        Self { bits: bits & 0b111 }
    }

    pub const fn bits(self) -> u8 {
        self.bits
    }

    pub const fn readable(self) -> bool {
        self.bits & Self::READ != 0
    }

    pub const fn writable(self) -> bool {
        self.bits & Self::WRITE != 0
    }

    pub const fn executable(self) -> bool {
        self.bits & Self::EXECUTE != 0
    }
}

impl From<SegmentFlags> for LoadPermissions {
    fn from(flags: SegmentFlags) -> Self {
        Self::from_bits(
            ((flags.readable() as u8) * Self::READ)
                | ((flags.writable() as u8) * Self::WRITE)
                | ((flags.executable() as u8) * Self::EXECUTE),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadAction {
    Copy {
        file_offset: usize,
        target_address: u64,
        len: usize,
    },
    ZeroFill {
        target_address: u64,
        len: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadRegion {
    pub start: u64,
    pub size: usize,
    pub page_count: usize,
    pub permissions: LoadPermissions,
    pub actions: Vec<LoadAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadPlan {
    pub image_name: alloc::string::String,
    pub source_path: alloc::string::String,
    pub entry_point: u64,
    pub regions: Vec<LoadRegion>,
    pub total_pages: usize,
    pub stack_pages: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadPlanError {
    UnsupportedFormat,
    InvalidAlignment,
    InvalidBounds,
    Overlap,
    WritableExecutable,
    MissingExecutableSegment,
    EntryOutsideExecutableSegment,
    BudgetExceeded,
}

pub fn build_load_plan(image: &ExecutableImage) -> Result<LoadPlan, LoadPlanError> {
    if image.format != ExecutableFormat::Elf64 {
        return Err(LoadPlanError::UnsupportedFormat);
    }
    if image.segments.len() > MAX_LOAD_REGIONS {
        return Err(LoadPlanError::BudgetExceeded);
    }

    let mut regions = Vec::new();
    for segment in &image.segments {
        let permissions = LoadPermissions::from(segment.flags);
        if permissions.writable() && permissions.executable() {
            return Err(LoadPlanError::WritableExecutable);
        }

        let start = align_down(segment.virtual_address, PAGE_SIZE as u64);
        let end = align_up(
            segment
                .virtual_address
                .checked_add(segment.memory_size as u64)
                .ok_or(LoadPlanError::InvalidBounds)?,
            PAGE_SIZE as u64,
        )?;
        if end <= start {
            return Err(LoadPlanError::InvalidBounds);
        }
        let size = (end - start) as usize;
        let page_count = pages_for_size(size)?;

        let mut actions = Vec::new();
        actions.push(LoadAction::Copy {
            file_offset: segment.file_offset,
            target_address: segment.virtual_address,
            len: segment.file_size,
        });
        if segment.memory_size > segment.file_size {
            actions.push(LoadAction::ZeroFill {
                target_address: segment.virtual_address + segment.file_size as u64,
                len: segment.memory_size - segment.file_size,
            });
        }

        regions.push(LoadRegion {
            start,
            size,
            page_count,
            permissions,
            actions,
        });
    }

    validate_regions(&regions)?;
    let total_pages = regions
        .iter()
        .map(|region| region.page_count)
        .sum::<usize>();
    if total_pages + STACK_RESERVATION_PAGES > MAX_IMAGE_PAGES {
        return Err(LoadPlanError::BudgetExceeded);
    }
    if !regions.iter().any(|region| {
        region.permissions.executable()
            && image.entry_point >= region.start
            && image.entry_point < region.start + region.size as u64
    }) {
        return Err(LoadPlanError::EntryOutsideExecutableSegment);
    }

    Ok(LoadPlan {
        image_name: image.name.clone(),
        source_path: image.source_path.clone(),
        entry_point: image.entry_point,
        regions,
        total_pages,
        stack_pages: STACK_RESERVATION_PAGES,
    })
}

pub fn validate_regions(regions: &[LoadRegion]) -> Result<(), LoadPlanError> {
    if !regions.iter().any(|region| region.permissions.executable()) {
        return Err(LoadPlanError::MissingExecutableSegment);
    }
    for (index, region) in regions.iter().enumerate() {
        if region.start % PAGE_SIZE as u64 != 0 || region.size == 0 || region.size % PAGE_SIZE != 0
        {
            return Err(LoadPlanError::InvalidAlignment);
        }
        if region.permissions.writable() && region.permissions.executable() {
            return Err(LoadPlanError::WritableExecutable);
        }
        let end = region
            .start
            .checked_add(region.size as u64)
            .ok_or(LoadPlanError::InvalidBounds)?;
        for other in regions.iter().skip(index + 1) {
            let other_end = other
                .start
                .checked_add(other.size as u64)
                .ok_or(LoadPlanError::InvalidBounds)?;
            if region.start < other_end && other.start < end {
                return Err(LoadPlanError::Overlap);
            }
        }
    }
    Ok(())
}

fn pages_for_size(size: usize) -> Result<usize, LoadPlanError> {
    if size == 0 {
        return Err(LoadPlanError::InvalidBounds);
    }
    Ok((size + PAGE_SIZE - 1) / PAGE_SIZE)
}

fn align_down(value: u64, align: u64) -> u64 {
    value / align * align
}

fn align_up(value: u64, align: u64) -> Result<u64, LoadPlanError> {
    value
        .checked_add(align - 1)
        .map(|value| value / align * align)
        .ok_or(LoadPlanError::InvalidBounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_image() -> ExecutableImage {
        crate::exec_image::parse_elf64_image(
            "hello",
            "/bin/hello.elf",
            crate::storage::phase11_sample_elf_image().as_bytes(),
            crate::task::program_loader::ProgramTrust::User,
            crate::security::Credentials::shell_user().user,
        )
        .expect("sample image should parse")
    }

    #[test_case]
    fn sample_image_builds_page_aligned_plan() {
        let plan = build_load_plan(&sample_image()).expect("plan should build");
        assert_eq!(plan.total_pages, 1);
        assert_eq!(plan.stack_pages, STACK_RESERVATION_PAGES);
        assert_eq!(plan.regions[0].start, 0x400000);
        assert_eq!(plan.regions[0].size, PAGE_SIZE);
        assert!(matches!(
            plan.regions[0].actions[0],
            LoadAction::Copy { len: 4, .. }
        ));
        assert!(matches!(
            plan.regions[0].actions[1],
            LoadAction::ZeroFill { len: 4092, .. }
        ));
    }

    #[test_case]
    fn writable_executable_region_is_rejected() {
        let region = LoadRegion {
            start: 0x400000,
            size: PAGE_SIZE,
            page_count: 1,
            permissions: LoadPermissions::from_bits(
                LoadPermissions::WRITE | LoadPermissions::EXECUTE,
            ),
            actions: Vec::new(),
        };
        assert_eq!(
            validate_regions(&[region]),
            Err(LoadPlanError::WritableExecutable)
        );
    }

    #[test_case]
    fn overlapping_regions_are_rejected() {
        let permissions =
            LoadPermissions::from_bits(LoadPermissions::READ | LoadPermissions::EXECUTE);
        let regions = [
            LoadRegion {
                start: 0x400000,
                size: PAGE_SIZE,
                page_count: 1,
                permissions,
                actions: Vec::new(),
            },
            LoadRegion {
                start: 0x400000,
                size: PAGE_SIZE,
                page_count: 1,
                permissions,
                actions: Vec::new(),
            },
        ];
        assert_eq!(validate_regions(&regions), Err(LoadPlanError::Overlap));
    }
}
