# Memory ABI (`clan-abi-v1` compat)

VMA, mmap, COW, and user copy contracts from scopes 1–100. Native **MemoryRegion** caps extend this without breaking compat VMA behavior.

See: [AXIOMS.md](AXIOMS.md), [USER_PAGE_TABLES.md](USER_PAGE_TABLES.md), [kernel/src/vma.rs](../kernel/src/vma.rs), [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md).

---

## User copy (scope 26)

- `copy_from_user` / `copy_to_user` — bounded, fault-safe
- `UserCopyProbe` syscall (60) — validation path

Native cap transfers may move **MemoryRegion** handles instead of implicit shared address space (post-111).

---

## VMA registry (scope 63+)

Per-process `VmaRegion`: `base`, `len`, `prot`, `backing` (`Anon`, `File`, `Image`).

| Feature | Scope |
|---------|------:|
| mmap | 54 |
| munmap + length | 73 |
| in-region split | 84 |
| adjacent coalesce | 96 |
| gap-aware anon hint | 93 |

Overlaps rejected; counters `REGIONS_REGISTERED`, `OVERLAPS_REJECTED`.

---

## W^X (scope 48)

Executable mappings policy enforced for user images — compat ELFs remain under existing allowlist/trust model ([SECURITY.md](SECURITY.md)).

---

## Fork-lite COW (scope 91+)

Anonymous pages duplicated by `fork_lite` / `fork_duplicate_cr3` are shared read-only with frame refcounts (`kernel/src/cow_fork.rs`). User writes trigger `#PF` → `try_break_on_write` → private writable copy. File-backed shared COW remains deferred.

---

## Native extensions (future — semantic version)

| Feature | Law |
|---------|-----|
| Cap-scoped shared memory | MemoryRegion object + RIGHTS_ALGEBRA move/borrow |
| Mapping via grant not path | Scope 114 storage grant |

Compat `Mmap`/`Munmap` remain for ELF/compat processes.
