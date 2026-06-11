# Build Integrity

```yaml
status: authoritative
semantics_version: 1.0.0
```

Epoch 2 prereq; phases 131–133 implementation. Epoch 0 stub.

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
| `install_userland.py` | FS install hook for `ares-rt` demo |
| Signed images | phases 131–133 |

### Reproducibility manifest (stub)

```toml
# scripts/repro-manifest.toml
rustc = "stable"
target = "x86_64-unknown-none"
```

QEMU scripts: `phase201_virtio_blk_check.py`, `phase134_endpoint_check.py`, `phase404_network_check.py`, `phase149_epoch5_check.py`, `phase150_milestone_check.py`.
