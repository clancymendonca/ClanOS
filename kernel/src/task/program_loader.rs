//! Phase 9 stored program manifest loader.

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramKind {
    BuiltinAlias,
    Elf64Image,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramTrust {
    System,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramManifest {
    pub name: String,
    pub kind: ProgramKind,
    pub entry: String,
    pub image_path: Option<String>,
    pub description: String,
    pub requires_execute: bool,
    pub trust: ProgramTrust,
    pub owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedProgram {
    pub name: String,
    pub source_path: String,
    pub kind: ProgramKind,
    pub entry: String,
    pub image_path: Option<String>,
    pub description: String,
    pub requires_execute: bool,
    pub trust: ProgramTrust,
    pub owner: String,
    pub image: Option<crate::exec_image::ExecutableImage>,
    pub image_error: Option<crate::exec_image::ImageLoadError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramLoadError {
    InvalidVersion,
    MissingName,
    MissingEntry,
    UnsupportedKind,
    UnsupportedTrust,
    UnsupportedRequirement,
    MissingImage,
    InvalidField,
    Storage,
    NotFound,
    PermissionDenied,
    UnsupportedExecution,
    ImageInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoaderStatus {
    pub program_count: usize,
    pub launch_count: u64,
    pub failed_launch_count: u64,
    pub denied_launch_count: u64,
    pub image_count: usize,
    pub valid_image_count: usize,
    pub invalid_image_count: usize,
    pub unsupported_execution_count: u64,
}

static LAUNCH_COUNT: AtomicU64 = AtomicU64::new(0);
static FAILED_LAUNCH_COUNT: AtomicU64 = AtomicU64::new(0);
static DENIED_LAUNCH_COUNT: AtomicU64 = AtomicU64::new(0);
static UNSUPPORTED_EXECUTION_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn parse_manifest(contents: &str) -> Result<ProgramManifest, ProgramLoadError> {
    let mut lines = contents.lines();
    if lines.next() != Some("ares-exec-v1") {
        return Err(ProgramLoadError::InvalidVersion);
    }

    let mut name: Option<String> = None;
    let mut kind: Option<ProgramKind> = None;
    let mut entry: Option<String> = None;
    let mut image_path: Option<String> = None;
    let mut description = String::new();
    let mut requires_execute = true;
    let mut trust = ProgramTrust::User;
    let mut owner = String::from("user");

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(ProgramLoadError::InvalidField);
        };
        match key {
            "name" if !value.is_empty() => name = Some(value.to_string()),
            "kind" if value == "builtin-alias" => kind = Some(ProgramKind::BuiltinAlias),
            "kind" if value == "elf64-image" => kind = Some(ProgramKind::Elf64Image),
            "kind" => return Err(ProgramLoadError::UnsupportedKind),
            "entry" if !value.is_empty() => entry = Some(value.to_string()),
            "image" if !value.is_empty() => image_path = Some(value.to_string()),
            "description" => description = value.to_string(),
            "requires" if value == "execute" => requires_execute = true,
            "requires" => return Err(ProgramLoadError::UnsupportedRequirement),
            "trust" if value == "system" => trust = ProgramTrust::System,
            "trust" if value == "user" => trust = ProgramTrust::User,
            "trust" => return Err(ProgramLoadError::UnsupportedTrust),
            "owner" => owner = value.to_string(),
            _ => return Err(ProgramLoadError::InvalidField),
        }
    }

    let kind = kind.ok_or(ProgramLoadError::UnsupportedKind)?;
    if kind == ProgramKind::Elf64Image && image_path.is_none() {
        return Err(ProgramLoadError::MissingImage);
    }

    Ok(ProgramManifest {
        name: name.ok_or(ProgramLoadError::MissingName)?,
        kind,
        entry: entry.ok_or(ProgramLoadError::MissingEntry)?,
        image_path,
        description,
        requires_execute,
        trust,
        owner,
    })
}

pub fn discover_programs() -> Vec<LoadedProgram> {
    let Ok(files) = crate::storage::list_files() else {
        return Vec::new();
    };

    let mut programs = Vec::new();
    for path in files {
        if !path.starts_with("/bin/") {
            continue;
        }
        let Ok(Some(contents)) = crate::storage::read_file(&path) else {
            continue;
        };
        let Ok(manifest) = parse_manifest(&contents) else {
            continue;
        };
        let (image, image_error) = validate_manifest_image(&manifest);
        programs.push(LoadedProgram {
            name: manifest.name,
            source_path: path,
            kind: manifest.kind,
            entry: manifest.entry,
            image_path: manifest.image_path,
            description: manifest.description,
            requires_execute: manifest.requires_execute,
            trust: manifest.trust,
            owner: manifest.owner,
            image,
            image_error,
        });
    }
    programs
}

pub fn resolve_program(name: &str) -> Result<LoadedProgram, ProgramLoadError> {
    discover_programs()
        .into_iter()
        .find(|program| program.name == name)
        .ok_or(ProgramLoadError::NotFound)
}

pub fn resolve_program_for(
    credentials: crate::security::Credentials,
    name: &str,
) -> Result<LoadedProgram, ProgramLoadError> {
    let program = resolve_program(name)?;
    if program.requires_execute {
        crate::storage::can_execute(credentials, &program.source_path).map_err(|_| {
            record_launch_denied();
            ProgramLoadError::PermissionDenied
        })?;
    }
    if let Some(image_path) = &program.image_path {
        crate::storage::can_execute(credentials, image_path).map_err(|_| {
            record_launch_denied();
            ProgramLoadError::PermissionDenied
        })?;
    }
    if program.kind == ProgramKind::Elf64Image {
        if program.image_error.is_some() {
            record_launch_failure();
            return Err(ProgramLoadError::ImageInvalid);
        }
        record_unsupported_execution();
        return Err(ProgramLoadError::UnsupportedExecution);
    }
    Ok(program)
}

pub fn program_info(name: &str) -> Result<LoadedProgram, ProgramLoadError> {
    resolve_program(name)
}

pub fn status() -> LoaderStatus {
    let programs = discover_programs();
    let image_count = programs
        .iter()
        .filter(|program| program.kind == ProgramKind::Elf64Image)
        .count();
    let valid_image_count = programs
        .iter()
        .filter(|program| program.kind == ProgramKind::Elf64Image && program.image.is_some())
        .count();
    LoaderStatus {
        program_count: programs.len(),
        launch_count: LAUNCH_COUNT.load(Ordering::Relaxed),
        failed_launch_count: FAILED_LAUNCH_COUNT.load(Ordering::Relaxed),
        denied_launch_count: DENIED_LAUNCH_COUNT.load(Ordering::Relaxed),
        image_count,
        valid_image_count,
        invalid_image_count: image_count.saturating_sub(valid_image_count),
        unsupported_execution_count: UNSUPPORTED_EXECUTION_COUNT.load(Ordering::Relaxed),
    }
}

pub fn record_launch_success() {
    LAUNCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn record_launch_failure() {
    FAILED_LAUNCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn record_launch_denied() {
    DENIED_LAUNCH_COUNT.fetch_add(1, Ordering::Relaxed);
    FAILED_LAUNCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn record_unsupported_execution() {
    UNSUPPORTED_EXECUTION_COUNT.fetch_add(1, Ordering::Relaxed);
    FAILED_LAUNCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub fn manifest_for_builtin(name: &str, description: &str) -> String {
    format!(
        "ares-exec-v1\nname={}\nkind=builtin-alias\nentry={}\nrequires=execute\ntrust=system\nowner=admin\ndescription={}",
        name, name, description
    )
}

pub fn phase9_smoke_check() -> bool {
    let before = status().launch_count;
    let programs = discover_programs();
    let has_echo = programs.iter().any(|program| {
        program.name == "echo" && program.source_path == "/bin/echo" && program.entry == "echo"
    });
    let launch_ok = crate::task::userspace::run_program("echo", &["phase9-loader"])
        .map(|output| output == "phase9-loader")
        .unwrap_or(false);
    let after = status();
    has_echo && launch_ok && after.launch_count > before && after.program_count >= 4
}

pub fn validate_program_image(
    credentials: crate::security::Credentials,
    name: &str,
) -> Result<crate::exec_image::ExecutableImage, ProgramLoadError> {
    let program = resolve_program(name)?;
    if program.kind != ProgramKind::Elf64Image {
        return Ok(crate::exec_image::builtin_image(
            &program.name,
            &program.source_path,
            program.trust,
            owner_id_for_manifest(&program.owner),
        ));
    }
    crate::storage::can_execute(credentials, &program.source_path)
        .map_err(|_| ProgramLoadError::PermissionDenied)?;
    let image_path = program.image_path.as_ref().ok_or(ProgramLoadError::MissingImage)?;
    crate::storage::can_execute(credentials, image_path)
        .map_err(|_| ProgramLoadError::PermissionDenied)?;
    program.image.ok_or(ProgramLoadError::ImageInvalid)
}

pub fn phase11_smoke_check() -> bool {
    let initial_status = status();
    let before = initial_status.unsupported_execution_count;
    let validate_ok = validate_program_image(crate::security::Credentials::shell_user(), "hello")
        .map(|image| {
            crate::address_space::descriptor_for_image(
                crate::address_space::AddressSpaceId::from_raw(1),
                &image,
            )
            .map(|descriptor| !descriptor.regions.is_empty())
            .unwrap_or(false)
        })
        .unwrap_or(false);
    let blocked_ok = crate::task::userspace::run_program("hello", &[])
        .map(|_| false)
        .unwrap_or(true)
        && status().unsupported_execution_count > before;
    validate_ok && initial_status.image_count >= 1 && initial_status.valid_image_count >= 1 && blocked_ok
}

fn validate_manifest_image(
    manifest: &ProgramManifest,
) -> (Option<crate::exec_image::ExecutableImage>, Option<crate::exec_image::ImageLoadError>) {
    if manifest.kind != ProgramKind::Elf64Image {
        return (None, None);
    }
    let Some(image_path) = &manifest.image_path else {
        return (None, Some(crate::exec_image::ImageLoadError::InvalidHeader));
    };
    let Ok(Some(contents)) = crate::storage::read_file(image_path) else {
        return (None, Some(crate::exec_image::ImageLoadError::InvalidHeader));
    };
    match crate::exec_image::parse_elf64_image(
        &manifest.name,
        image_path,
        contents.as_bytes(),
        manifest.trust,
        owner_id_for_manifest(&manifest.owner),
    ) {
        Ok(image) => (Some(image), None),
        Err(err) => (None, Some(err)),
    }
}

fn owner_id_for_manifest(owner: &str) -> crate::security::UserId {
    match owner {
        "admin" => crate::security::Credentials::admin().user,
        "kernel" => crate::security::Credentials::kernel().user,
        "guest" => crate::security::Credentials::guest().user,
        _ => crate::security::Credentials::shell_user().user,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn valid_manifest_parses() {
        let manifest = parse_manifest(
            "ares-exec-v1\nname=echo\nkind=builtin-alias\nentry=echo\ndescription=Echo text",
        )
        .expect("manifest should parse");
        assert_eq!(manifest.name, "echo");
        assert_eq!(manifest.kind, ProgramKind::BuiltinAlias);
        assert_eq!(manifest.entry, "echo");
        assert!(manifest.requires_execute);
    }

    #[test_case]
    fn invalid_manifest_version_is_rejected() {
        assert_eq!(
            parse_manifest("bad-version\nname=echo\nkind=builtin-alias\nentry=echo"),
            Err(ProgramLoadError::InvalidVersion)
        );
    }

    #[test_case]
    fn missing_required_fields_are_rejected() {
        assert_eq!(
            parse_manifest("ares-exec-v1\nkind=builtin-alias\nentry=echo"),
            Err(ProgramLoadError::MissingName)
        );
        assert_eq!(
            parse_manifest("ares-exec-v1\nname=echo\nkind=builtin-alias"),
            Err(ProgramLoadError::MissingEntry)
        );
    }

    #[test_case]
    fn unsupported_kind_is_rejected() {
        assert_eq!(
            parse_manifest("ares-exec-v1\nname=x\nkind=elf\nentry=x"),
            Err(ProgramLoadError::UnsupportedKind)
        );
    }

    #[test_case]
    fn unsupported_trust_and_requirement_are_rejected() {
        assert_eq!(
            parse_manifest("ares-exec-v1\nname=x\nkind=builtin-alias\nentry=x\ntrust=unsigned"),
            Err(ProgramLoadError::UnsupportedTrust)
        );
        assert_eq!(
            parse_manifest("ares-exec-v1\nname=x\nkind=builtin-alias\nentry=x\nrequires=network"),
            Err(ProgramLoadError::UnsupportedRequirement)
        );
    }
}
