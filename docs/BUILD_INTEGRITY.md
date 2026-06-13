```yaml
status: superseded-by: docs/process/BUILD_INTEGRITY.md
semantics_version: 1.0.0
```

> **Canonical:** [`docs/process/BUILD_INTEGRITY.md`](process/BUILD_INTEGRITY.md). This flat copy retained until migration squash reconciles any differences.

# Build Integrity

```yaml
status: authoritative
semantics_version: 1.0.0
```

Epoch 2 prereq; scopes 131–133 implementation. Epoch 0 stub.

---

## Epoch 0

- CI scripts in reproducibility manifest (stub)
- `project_health.py` deterministic output
- QEMU integration scripts CI-checked

---

## Epoch 2

| Item | Status |
|------|--------|
| Tool manifest | `scripts/repro-manifest.toml` (rustc, llvm, linker pins) |
| Dual-build hash | stub CI — compare `target/` kernel hash twice same source |
| `install_userland.py` | FS install hook for `clan-rt` demo |
| Signed images | scopes 131–133 |

### Reproducibility manifest (stub)

```toml
# scripts/repro-manifest.toml
rustc = "stable"
target = "x86_64-unknown-none"
```

QEMU scripts: `python scripts/gate/boot.py --gate virtio_blk --timeout 180`, `python scripts/gate/boot.py --gate build_endpoints --timeout 180`, `python scripts/gate/boot.py --gate network_compat --timeout 180`, `python scripts/gate/boot.py --gate scheduler_epoch --timeout 180`, `python scripts/gate/boot.py --gate boundary --timeout 180`.
