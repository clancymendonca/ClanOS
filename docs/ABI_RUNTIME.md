# Native Runtime ABI (language-neutral draft)

Rust is the **first implementation language** for kernel and platform code — not the **definition** of native.

Phase **109** outline; language adapters phases **171–180**.

See: [AXIOMS.md](AXIOMS.md), [NATIVE_MODEL.md](NATIVE_MODEL.md), [ABI_ASYNC.md](ABI_ASYNC.md), [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md).

---

## Goals

| Goal | Detail |
|------|--------|
| Language-neutral | C, Rust, Zig, managed runtimes — without POSIX libc as native core |
| No POSIX assumptions | Threading, IO, and FS via caps/endpoints |
| Adapter layer | Lives in **runtime** stratum, not kernel |

---

## Contract areas (hard)

| Area | OS owns | Runtime adapter owns |
|------|---------|----------------------|
| Authority | Cap ops, generation | Wrapper types, linear handles |
| Async wait | Wait sets, cancel tokens | `async/await` lowering, executor |
| Panic / unwind | Documented cross-language policy | Language-specific handlers |
| TLS | Per-runtime thread storage layout | |
| Structured concurrency | Cancel scope propagation | Task trees |

---

## Entry (draft)

Native binary format `ares-native-v1` (manifest + image) — distinct from ELF compat:

- Entry symbol `ares_native_main`
- Receives **cap table handle** and service catalog — not `argc/argv` paths
- Manifest: [NATIVE_MODEL.md](NATIVE_MODEL.md) — permissions as cap requests

ELF entry remains `0x400000` style for compat ([PROGRAMS.md](PROGRAMS.md)).

---

## FFI ownership transfer

Cross-language calls use **move** of buffer caps or copied slabs per [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) — never implicit shared global pointers.

---

## Non-goals (v1 outline)

- Full C++ exception interop
- JVM bytecode host
- POSIX pthread as native default
