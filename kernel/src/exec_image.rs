//! executable image parser and validation model.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{security::UserId, task::program_loader::ProgramTrust};

pub const MAX_IMAGE_SIZE: usize = 512;
const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EM_X86_64: u16 = 0x3e;
const PT_LOAD: u32 = 1;
const MAX_SEGMENTS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutableFormat {
    BuiltinAlias,
    Elf64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentFlags {
    bits: u8,
}

impl SegmentFlags {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageSegment {
    pub virtual_address: u64,
    pub file_offset: usize,
    pub file_size: usize,
    pub memory_size: usize,
    pub flags: SegmentFlags,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutableImage {
    pub name: String,
    pub source_path: String,
    pub format: ExecutableFormat,
    pub entry_point: u64,
    pub image_size: usize,
    pub trust: ProgramTrust,
    pub owner: UserId,
    pub segments: Vec<ImageSegment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageLoadError {
    InvalidMagic,
    UnsupportedArchitecture,
    InvalidHeader,
    InvalidSegmentLayout,
    MissingExecutePermission,
    OversizedImage,
    UnsupportedExecution,
}

pub fn builtin_image(
    name: &str,
    source_path: &str,
    trust: ProgramTrust,
    owner: UserId,
) -> ExecutableImage {
    ExecutableImage {
        name: name.to_string(),
        source_path: source_path.to_string(),
        format: ExecutableFormat::BuiltinAlias,
        entry_point: 0,
        image_size: 0,
        trust,
        owner,
        segments: Vec::new(),
    }
}

pub fn parse_elf64_image(
    name: &str,
    source_path: &str,
    bytes: &[u8],
    trust: ProgramTrust,
    owner: UserId,
) -> Result<ExecutableImage, ImageLoadError> {
    if bytes.len() > MAX_IMAGE_SIZE {
        return Err(ImageLoadError::OversizedImage);
    }
    if bytes.len() < 64 || &bytes[..4] != ELF_MAGIC {
        return Err(ImageLoadError::InvalidMagic);
    }
    if bytes[4] != ELFCLASS64 || bytes[5] != ELFDATA2LSB || read_u16(bytes, 18)? != EM_X86_64 {
        return Err(ImageLoadError::UnsupportedArchitecture);
    }

    let entry = read_u64(bytes, 24)?;
    let phoff = read_u64(bytes, 32)? as usize;
    let phentsize = read_u16(bytes, 54)? as usize;
    let phnum = read_u16(bytes, 56)? as usize;
    if entry == 0 || phoff == 0 || phentsize < 56 || phnum == 0 || phnum > MAX_SEGMENTS {
        return Err(ImageLoadError::InvalidHeader);
    }

    let table_end = phoff
        .checked_add(
            phentsize
                .checked_mul(phnum)
                .ok_or(ImageLoadError::InvalidHeader)?,
        )
        .ok_or(ImageLoadError::InvalidHeader)?;
    if table_end > bytes.len() {
        return Err(ImageLoadError::InvalidHeader);
    }

    let mut segments = Vec::new();
    for index in 0..phnum {
        let start = phoff + index * phentsize;
        if read_u32_at(bytes, start)? != PT_LOAD {
            continue;
        }
        let raw_flags = read_u32_at(bytes, start + 4)?;
        let file_offset = read_u64_at(bytes, start + 8)? as usize;
        let virtual_address = read_u64_at(bytes, start + 16)?;
        let file_size = read_u64_at(bytes, start + 32)? as usize;
        let memory_size = read_u64_at(bytes, start + 40)? as usize;
        let flags = SegmentFlags::from_bits(
            ((raw_flags & 0x4 != 0) as u8) * SegmentFlags::READ
                | ((raw_flags & 0x2 != 0) as u8) * SegmentFlags::WRITE
                | ((raw_flags & 0x1 != 0) as u8) * SegmentFlags::EXECUTE,
        );

        validate_segment(
            bytes.len(),
            file_offset,
            file_size,
            memory_size,
            virtual_address,
            flags,
        )?;
        segments.push(ImageSegment {
            virtual_address,
            file_offset,
            file_size,
            memory_size,
            flags,
        });
    }

    validate_segments(&segments)?;
    Ok(ExecutableImage {
        name: name.to_string(),
        source_path: source_path.to_string(),
        format: ExecutableFormat::Elf64,
        entry_point: entry,
        image_size: bytes.len(),
        trust,
        owner,
        segments,
    })
}

fn validate_segment(
    image_len: usize,
    file_offset: usize,
    file_size: usize,
    memory_size: usize,
    virtual_address: u64,
    flags: SegmentFlags,
) -> Result<(), ImageLoadError> {
    if virtual_address == 0 || memory_size == 0 || file_size == 0 || memory_size < file_size {
        return Err(ImageLoadError::InvalidSegmentLayout);
    }
    if file_offset
        .checked_add(file_size)
        .ok_or(ImageLoadError::InvalidSegmentLayout)?
        > image_len
    {
        return Err(ImageLoadError::InvalidSegmentLayout);
    }
    if flags.writable() && flags.executable() {
        return Err(ImageLoadError::InvalidSegmentLayout);
    }
    Ok(())
}

pub fn validate_segments(segments: &[ImageSegment]) -> Result<(), ImageLoadError> {
    if segments.is_empty() {
        return Err(ImageLoadError::InvalidSegmentLayout);
    }
    for (index, segment) in segments.iter().enumerate() {
        let end = segment
            .virtual_address
            .checked_add(segment.memory_size as u64)
            .ok_or(ImageLoadError::InvalidSegmentLayout)?;
        for other in segments.iter().skip(index + 1) {
            let other_end = other
                .virtual_address
                .checked_add(other.memory_size as u64)
                .ok_or(ImageLoadError::InvalidSegmentLayout)?;
            if segment.virtual_address < other_end && other.virtual_address < end {
                return Err(ImageLoadError::InvalidSegmentLayout);
            }
        }
    }
    Ok(())
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ImageLoadError> {
    read_u16_at(bytes, offset)
}

fn read_u16_at(bytes: &[u8], offset: usize) -> Result<u16, ImageLoadError> {
    let end = offset.checked_add(2).ok_or(ImageLoadError::InvalidHeader)?;
    let slice = bytes
        .get(offset..end)
        .ok_or(ImageLoadError::InvalidHeader)?;
    Ok(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32_at(bytes: &[u8], offset: usize) -> Result<u32, ImageLoadError> {
    let end = offset.checked_add(4).ok_or(ImageLoadError::InvalidHeader)?;
    let slice = bytes
        .get(offset..end)
        .ok_or(ImageLoadError::InvalidHeader)?;
    Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, ImageLoadError> {
    read_u64_at(bytes, offset)
}

fn read_u64_at(bytes: &[u8], offset: usize) -> Result<u64, ImageLoadError> {
    let end = offset.checked_add(8).ok_or(ImageLoadError::InvalidHeader)?;
    let slice = bytes
        .get(offset..end)
        .ok_or(ImageLoadError::InvalidHeader)?;
    Ok(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn valid_minimal_elf64_image_parses() {
        let bytes = crate::storage::sample_elf_fixture_image();
        let image = parse_elf64_image(
            "hello",
            "/bin/hello.elf",
            bytes.as_bytes(),
            ProgramTrust::User,
            UserId::from_raw(100),
        )
        .expect("sample image should parse");
        assert_eq!(image.format, ExecutableFormat::Elf64);
        assert_eq!(image.entry_point, 0x400000);
        assert_eq!(image.segments.len(), 1);
        assert!(image.segments[0].flags.executable());
    }

    #[test_case]
    fn invalid_magic_is_rejected() {
        assert_eq!(
            parse_elf64_image(
                "bad",
                "/bin/bad",
                b"not-elf",
                ProgramTrust::User,
                UserId::from_raw(100)
            ),
            Err(ImageLoadError::InvalidMagic)
        );
    }

    #[test_case]
    fn overlapping_segments_are_rejected() {
        let first = ImageSegment {
            virtual_address: 0x400000,
            file_offset: 120,
            file_size: 1,
            memory_size: 0x1000,
            flags: SegmentFlags::from_bits(SegmentFlags::READ),
        };
        let second = ImageSegment {
            virtual_address: 0x400800,
            file_offset: 121,
            file_size: 1,
            memory_size: 0x1000,
            flags: SegmentFlags::from_bits(SegmentFlags::READ),
        };
        assert_eq!(
            validate_segments(&[first, second]),
            Err(ImageLoadError::InvalidSegmentLayout)
        );
    }
}
