# Memory ABI (`ares-abi-v1` compat)

VMA, mmap, COW, and user copy contracts from phases 1–100. Native **MemoryRegion** caps extend this without breaking compat VMA behavior.

See: [AXIOMS.md](AXIOMS.md), [USER_PAGE_TABLES.md](USER_PAGE_TABLES.md), [kernel/src/vma.rs](../kernel/src/vma.rs), [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md).

---

## User copy (phase 26)

- `copy_from_user` / `copy_to_user` — bounded, fault-safe
- `UserCopyProbe` syscall (60) — validation path

Native cap transfers may move **MemoryRegion** handles instead of implicit shared address space (post-111).

---

## VMA registry (phase 63+)

Per-process `VmaRegion`: `base`, `len`, `prot`, `backing` (`Anon`, `File`, `Image`).

| Feature | Phase |
|---------|------:|
| mmap | 54 |
| munmap + length | 73 |
| in-region split | 84 |
| adjacent coalesce | 96 |
| gap-aware anon hint | 93 |

Overlaps rejected; counters `REGIONS_REGISTERED`, `OVERLAPS_REJECTED`.

---

## W^X (phase 48)

Executable mappings policy enforced for user images — compat ELFs remain under existing allowlist/trust model ([SECURITY.md](SECURITY.md)).

---

## Fork-lite COW (phase 91)

Anonymous page **COW break** on write after `ForkLite` — parent/child isolation smoke. File-backed shared COW deferred (phase 100 checklist).

---

## Native extensions (future — semantic version)

| Feature | Law |
|---------|-----|
| Cap-scoped shared memory | MemoryRegion object + RIGHTS_ALGEBRA move/borrow |
| Mapping via grant not path | Phase 114 storage grant |

Compat `Mmap`/`Munmap` remain for ELF/compat processes.
