> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

# Phase 86 Checklist: `ExecLite` + Close-on-Exec

## Scope

- [x] `ExecLite = 81` replaces process image from allowlisted name; sweeps `FD_CLOEXEC` fds.
- [x] Covered by boot gate `path_exec` (`AresOS-BootGate: name=path_exec ok=true`)

## Validation

- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/gate/boot.py --phase 86 --timeout 180

## Deferred

- [ ] Full `execve` argv/env and unsigned ELF load.
