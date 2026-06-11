# ares-rt ABI (Epoch 2)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Userspace runtime for native services. Forward ABI stability policy per epoch gate.

---

## Epoch 2 deliverables

- `userland/` crate with libc subset + demo programs
- Syscall stubs for compat + native service entry
- FS install build hook for QEMU image

---

## Stability

Pre-1.0: breaking bumps allowed with compat review. Post-1.0: semver window TBD at M150.
