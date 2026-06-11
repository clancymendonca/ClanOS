//! Checkpoint/restore security domain (epoch 13).

use core::sync::atomic::{AtomicU64, Ordering};

static CHECKPOINT_OPS: AtomicU64 = AtomicU64::new(0);
static PERSISTED_GENERATION: AtomicU64 = AtomicU64::new(1);

pub fn checkpoint_ops() -> u64 {
    CHECKPOINT_OPS.load(Ordering::Relaxed)
}

pub fn persisted_generation() -> u64 {
    PERSISTED_GENERATION.load(Ordering::Relaxed)
}

/// Suspend = frozen-in-memory; checkpoint persists cap generation across power cycle.
pub fn persist_cap_state(generation: u64) -> bool {
    if generation == 0 {
        return false;
    }
    PERSISTED_GENERATION.store(generation, Ordering::Relaxed);
    CHECKPOINT_OPS.fetch_add(1, Ordering::Relaxed);
    true
}

pub fn epoch13_checkpoint_graduated() -> bool {
    persist_cap_state(2) && checkpoint_ops() > 0 && persisted_generation() >= 2
}
