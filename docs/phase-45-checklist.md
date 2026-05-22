# Phase 45 Checklist: File Descriptor Table

## Scope

- [x] Bring-up FD table with `OpenFile` / `CloseFile` syscalls.
- [x] Emit `Phase45-FileFd` boot smoke output.

## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration`
- [x] `python scripts/phase45_file_fd_check.py --timeout 180`

## Deferred

- [ ] Per-process FD tables; dup; cwd-relative paths.
