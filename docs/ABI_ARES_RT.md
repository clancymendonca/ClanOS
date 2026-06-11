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

**Forward policy (epoch 2 decision):** explicit **recompile required** each epoch until 1.0 (`ABI_FORWARD_POLICY` in `ares-rt`).

Pre-1.0: breaking bumps allowed with compat review. Post-1.0: semver window TBD at M150.

## Build

Workspace default target is `x86_64-unknown-none` (kernel). Host demo builds use an explicit host triple:

```bash
python scripts/install_userland.py
```

`install_userland.py` selects the host triple (`x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, or Apple variants) and stages `target/userland-staging/demo-hello.txt` for the QEMU FS install hook.
