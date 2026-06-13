//! identity and access-control primitives.

use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserId(u64);

impl UserId {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Kernel,
    Admin,
    User,
    Guest,
}

impl Role {
    pub const fn as_u64(self) -> u64 {
        match self {
            Role::Kernel => 0,
            Role::Admin => 1,
            Role::User => 2,
            Role::Guest => 3,
        }
    }

    pub const fn from_u64(raw: u64) -> Self {
        match raw {
            0 => Role::Kernel,
            1 => Role::Admin,
            3 => Role::Guest,
            _ => Role::User,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Role::Kernel => "kernel",
            Role::Admin => "admin",
            Role::User => "user",
            Role::Guest => "guest",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Credentials {
    pub user: UserId,
    pub role: Role,
}

impl Credentials {
    pub const fn kernel() -> Self {
        Self {
            user: UserId::from_raw(0),
            role: Role::Kernel,
        }
    }

    pub const fn admin() -> Self {
        Self {
            user: UserId::from_raw(1),
            role: Role::Admin,
        }
    }

    pub const fn shell_user() -> Self {
        Self {
            user: UserId::from_raw(100),
            role: Role::User,
        }
    }

    pub const fn guest() -> Self {
        Self {
            user: UserId::from_raw(65534),
            role: Role::Guest,
        }
    }

    pub const fn can_manage(self) -> bool {
        matches!(self.role, Role::Kernel | Role::Admin)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKind {
    Read,
    Write,
    Execute,
    Manage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    PermissionDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMode {
    bits: u8,
}

impl FileMode {
    pub const READ: u8 = 0b001;
    pub const WRITE: u8 = 0b010;
    pub const EXECUTE: u8 = 0b100;

    pub const fn from_bits(bits: u8) -> Self {
        Self { bits: bits & 0b111 }
    }

    pub const fn bits(self) -> u8 {
        self.bits
    }

    pub const fn read_only() -> Self {
        Self::from_bits(Self::READ)
    }

    pub const fn user_file() -> Self {
        Self::from_bits(Self::READ | Self::WRITE)
    }

    pub const fn executable() -> Self {
        Self::from_bits(Self::READ | Self::EXECUTE)
    }

    pub const fn system_executable() -> Self {
        Self::from_bits(Self::READ | Self::EXECUTE)
    }

    pub const fn allows(self, access: AccessKind) -> bool {
        match access {
            AccessKind::Read => self.bits & Self::READ != 0,
            AccessKind::Write => self.bits & Self::WRITE != 0,
            AccessKind::Execute => self.bits & Self::EXECUTE != 0,
            AccessKind::Manage => false,
        }
    }

    pub fn set_execute(&mut self, enabled: bool) {
        if enabled {
            self.bits |= Self::EXECUTE;
        } else {
            self.bits &= !Self::EXECUTE;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessDecision {
    pub allowed: bool,
}

static CURRENT_USER: AtomicU64 = AtomicU64::new(100);
static CURRENT_ROLE: AtomicU64 = AtomicU64::new(Role::User.as_u64());
static DENIED_ACCESS_COUNT: AtomicU64 = AtomicU64::new(0);
static DENIED_EXECUTE_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn current_credentials() -> Credentials {
    Credentials {
        user: UserId::from_raw(CURRENT_USER.load(Ordering::Relaxed)),
        role: Role::from_u64(CURRENT_ROLE.load(Ordering::Relaxed)),
    }
}

pub fn set_current_credentials(credentials: Credentials) {
    CURRENT_USER.store(credentials.user.as_u64(), Ordering::Relaxed);
    CURRENT_ROLE.store(credentials.role.as_u64(), Ordering::Relaxed);
}

pub fn denied_access_count() -> u64 {
    DENIED_ACCESS_COUNT.load(Ordering::Relaxed)
}

pub fn denied_execute_count() -> u64 {
    DENIED_EXECUTE_COUNT.load(Ordering::Relaxed)
}

pub fn record_denial(access: AccessKind) {
    DENIED_ACCESS_COUNT.fetch_add(1, Ordering::Relaxed);
    if access == AccessKind::Execute {
        DENIED_EXECUTE_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn can_access(
    credentials: Credentials,
    owner: UserId,
    mode: FileMode,
    access: AccessKind,
) -> Result<AccessDecision, SecurityError> {
    let allowed = match access {
        AccessKind::Manage => credentials.can_manage(),
        AccessKind::Write => {
            credentials.can_manage() || (credentials.user == owner && mode.allows(access))
        }
        AccessKind::Read | AccessKind::Execute => {
            credentials.can_manage()
                || mode.allows(access)
                || (credentials.user == owner && mode.allows(access))
        }
    };

    if allowed {
        Ok(AccessDecision { allowed: true })
    } else {
        Err(SecurityError::PermissionDenied)
    }
}

pub fn can_manage_process(actor: Credentials, owner: Credentials) -> bool {
    actor.can_manage() || actor.user == owner.user
}

pub fn smoke_access_policy() -> bool {
    let user = Credentials::shell_user();
    let admin = Credentials::admin();
    let owner = user.user;
    can_access(user, owner, FileMode::user_file(), AccessKind::Write).is_ok()
        && can_access(
            user,
            UserId::from_raw(1),
            FileMode::system_executable(),
            AccessKind::Write,
        )
        .is_err()
        && can_access(
            admin,
            UserId::from_raw(1),
            FileMode::system_executable(),
            AccessKind::Manage,
        )
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn permission_predicates_allow_owner_and_admin() {
        let user = Credentials::shell_user();
        let admin = Credentials::admin();
        assert!(can_access(user, user.user, FileMode::user_file(), AccessKind::Write).is_ok());
        assert!(can_access(admin, user.user, FileMode::read_only(), AccessKind::Manage).is_ok());
    }

    #[test_case]
    fn permission_predicates_reject_unowned_writes() {
        let user = Credentials::shell_user();
        assert_eq!(
            can_access(
                user,
                UserId::from_raw(1),
                FileMode::system_executable(),
                AccessKind::Write
            ),
            Err(SecurityError::PermissionDenied)
        );
    }
}
