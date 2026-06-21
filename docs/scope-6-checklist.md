> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 6 Checklist (User Space, Shell, Storage, Syscalls, Stabilization)

**Date**: 2026-05-06  
**Status**: Complete ✅

## 1. Shell & Process Commands

- [x] Interactive keyboard-console shell loop active in kernel runtime
- [x] Built-ins: `help`, `ps`, `kill`, `sched`, `metrics`
- [x] User program launcher command: `run <program> [args...]`

## 2. User Utilities (MVP)

- [x] `echo` user utility
- [x] `time` user utility
- [x] `sysinfo` user utility

## 3. Storage Baseline

- [x] In-memory filesystem baseline
- [x] Shell commands: `ls`, `cat <path>`
- [x] Program entries represented under `/bin/*`

## 4. Syscall Surface & Isolation Baseline

- [x] Minimal syscall dispatch layer (`GetTickCount`, `GetProcessCount`, `GetTotalPreemptions`)
- [x] Invalid syscall rejection path + tests
- [x] User utilities consume data through syscall interface

## 5. Stabilization & QA

- [x] `python scripts/gate/run.py --gate shell_storage --timeout 180` for quick validation
- [x] Build-level validation via `cargo check -p kernel`
- [x] Existing Scope 5 checks retained for latency/fairness coverage
- [x] One-command validation matrix (`scripts/validation_matrix.py`) with PASS/FAIL output and thresholds

## Validation

```bash
cargo check -p kernel
python scripts/gate/run.py --gate shell_storage --timeout 180
python scripts/validation_matrix.py --soak-duration 30 --latency-duration 30 --boot-wait 90 --smoke-timeout 180
```
