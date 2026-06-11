//! Pure rights algebra for tier-A proptest and tier-B Kani harnesses.
//! Mirrors `kernel::kernel_object::Rights` composition laws (RIGHTS_ALGEBRA.md R-01, R-06).

/// Rights bitmask — keep in sync with `kernel/src/kernel_object.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rights(pub u32);

impl Rights {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const MAP: u32 = 1 << 2;
    pub const DELEGATE: u32 = 1 << 3;
    pub const REVOKE: u32 = 1 << 4;
    pub const ALL_FLAGS: u32 = Self::READ | Self::WRITE | Self::MAP | Self::DELEGATE | Self::REVOKE;

    pub const fn empty() -> Self {
        Rights(0)
    }

    pub const fn read_write() -> Self {
        Rights(Self::READ | Self::WRITE)
    }

    pub fn contains(self, other: Rights) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn intersect(self, other: Rights) -> Rights {
        Rights(self.0 & other.0)
    }

    pub fn union(self, other: Rights) -> Rights {
        Rights(self.0 | other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelegateVerdict {
    Allowed,
    AmplificationDenied,
    MissingDelegateRight,
}

/// R-01 / R-06: child rights must be subset of parent; parent must hold DELEGATE.
pub fn delegate_verdict(parent: Rights, child: Rights) -> DelegateVerdict {
    if !parent.contains(Rights(Rights::DELEGATE)) {
        return DelegateVerdict::MissingDelegateRight;
    }
    if !parent.contains(child) {
        return DelegateVerdict::AmplificationDenied;
    }
    DelegateVerdict::Allowed
}

/// Monotonicity along delegation: effective rights never grow vs parent.
pub fn effective_rights_after_delegate(parent: Rights, child: Rights) -> Option<Rights> {
    match delegate_verdict(parent, child) {
        DelegateVerdict::Allowed => Some(child),
        _ => None,
    }
}

/// R-01 chain: successive delegates only shrink (intersection property).
pub fn chain_delegate(parent: Rights, mid: Rights, leaf: Rights) -> Option<Rights> {
    let mid_slot = effective_rights_after_delegate(parent, mid)?;
    effective_rights_after_delegate(mid_slot, leaf)
}

#[cfg(kani)]
mod kani_harnesses {
    use super::*;
    use kani::cover;

    /// Tier B — bound: all 5 right flags (32 combinations). Threat: T-cap-amplification.
    #[kani::proof]
    fn delegate_no_amplification() {
        let parent_bits: u32 = kani::any();
        let child_bits: u32 = kani::any();
        kani::assume(parent_bits <= Rights::ALL_FLAGS);
        kani::assume(child_bits <= Rights::ALL_FLAGS);
        let parent = Rights(parent_bits);
        let child = Rights(child_bits);
        match delegate_verdict(parent, child) {
            DelegateVerdict::Allowed => {
                kani::assert!(parent.contains(child));
                kani::assert!(child.0 <= parent.0);
            }
            DelegateVerdict::AmplificationDenied => {
                kani::assert!(!parent.contains(child));
                cover!(true);
            }
            DelegateVerdict::MissingDelegateRight => {
                kani::assert!(!parent.contains(Rights(Rights::DELEGATE)));
                cover!(true);
            }
        }
    }

    /// Vacuity: amplification-denied path is reachable (not trivially dead).
    #[kani::proof]
    fn amplification_path_reachable() {
        let parent = Rights(Rights::READ | Rights::DELEGATE);
        let child = Rights::read_write();
        kani::assert_eq!(
            delegate_verdict(parent, child),
            DelegateVerdict::AmplificationDenied
        );
        cover!(true);
    }

    /// R-01 chain monotonicity at depth 2 within harness bound.
    #[kani::proof]
    fn chain_monotone() {
        let p: u32 = kani::any();
        let m: u32 = kani::any();
        let l: u32 = kani::any();
        kani::assume(p <= Rights::ALL_FLAGS);
        kani::assume(m <= Rights::ALL_FLAGS);
        kani::assume(l <= Rights::ALL_FLAGS);
        let parent = Rights(p);
        if let Some(leaf_rights) = chain_delegate(parent, Rights(m), Rights(l)) {
            kani::assert!(leaf_rights.0 <= parent.0);
        }
    }
}

#[cfg(test)]
mod exhaustive_tests {
    use super::*;

    #[test]
    fn exhaustive_delegate_laws() {
        for parent_bits in 0..=Rights::ALL_FLAGS {
            for child_bits in 0..=Rights::ALL_FLAGS {
                let parent = Rights(parent_bits);
                let child = Rights(child_bits);
                match delegate_verdict(parent, child) {
                    DelegateVerdict::Allowed => {
                        assert!(parent.contains(child));
                        assert!(child.0 <= parent.0);
                    }
                    DelegateVerdict::AmplificationDenied => {
                        assert!(!parent.contains(child));
                    }
                    DelegateVerdict::MissingDelegateRight => {
                        assert!(!parent.contains(Rights(Rights::DELEGATE)));
                    }
                }
            }
        }
    }
}
