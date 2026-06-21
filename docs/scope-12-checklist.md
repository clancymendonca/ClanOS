> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy numbered boot serial lines are retired.

# Scope 12 Checklist

Status: Complete

- [x] Add load-plan, load-region, load-action, permission, and error types.
- [x] Convert validated ELF64 segments into page-aligned load regions.
- [x] Track copy and zero-fill actions separately.
- [x] Reject writable+executable, overlapping, over-budget, and invalid-entry plans.
- [x] Add descriptor-only frame/page reservation accounting.
- [x] Add loader `prepare_program_image` path and load-plan counters.
- [x] Add process load metadata for prepared image records.
- [x] Add `bin prepare`, richer `bin info`, and load-plan summary commands.
- [x] Covered by validation gate `loader_security` (`ClanOS-Gate: name=loader_security ok=true`)
- [x] Add Scope 12 QEMU validation and validation matrix coverage.

Exit gate:

- [x] `/bin/hello` prepares a deterministic load plan.
- [x] Load planning does not mutate active page tables.
- [x] ELF execution remains blocked with a clear unsupported-execution result.
- [x] Existing built-in aliases continue to launch.

## Validation

```bash
cargo check -p kernel
python scripts/gate/run.py --gate loader_security --timeout 180
python scripts/validation_matrix.py --smoke-timeout 180
```

See [VALIDATION_GATES.md](VALIDATION_GATES.md).
