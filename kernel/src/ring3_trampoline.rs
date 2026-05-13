//! Phase 18 controlled Ring 3 trampoline model.

use crate::{interrupts::USER_TRAP_VECTOR, user_context::UserContextDescriptor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserTrapReason {
    ControlledReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ring3TrampolineResult {
    pub entry_rip: u64,
    pub user_rsp: u64,
    pub trap_vector: u8,
    pub reason: UserTrapReason,
    pub ring3_entered: bool,
    pub trapped_back: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ring3TrampolineError {
    ContextNotReady,
}

pub fn enter_controlled_trampoline(
    context: &UserContextDescriptor,
) -> Result<Ring3TrampolineResult, Ring3TrampolineError> {
    if !context.selectors_ready || !context.entry_ready {
        return Err(Ring3TrampolineError::ContextNotReady);
    }

    Ok(Ring3TrampolineResult {
        entry_rip: context.entry.rip,
        user_rsp: context.entry.rsp,
        trap_vector: USER_TRAP_VECTOR,
        reason: UserTrapReason::ControlledReturn,
        ring3_entered: true,
        trapped_back: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn trap_vector_is_user_vector() {
        assert_eq!(USER_TRAP_VECTOR, 0x80);
    }
}
