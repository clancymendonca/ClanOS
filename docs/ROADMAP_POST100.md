# Post-100 Roadmap (Phases 101–150)

Constitutional operating-system architecture — **documentation pass** phases 101–110; implementation phases 111+ gated ([AXIOMS.md](AXIOMS.md)).

**Prime directive:** philosophy before implementation. **Central truth:** semantic coherence across decades is harder than building the kernel.

Index: [INDEX.md](INDEX.md) · Vision: [NATIVE_MODEL.md](NATIVE_MODEL.md)

---

## Governance gates (phase 110 sign-off)

| Gate | Blocks | Document |
|------|--------|----------|
| G1 | 115+ new handle semantics | [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) |
| G2 | 112–113 cap code | [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) |
| G3 | 134+ endpoints | [ABI_IPC.md](ABI_IPC.md) |
| G4 | 128+ native enforcement | [NATIVE_MODEL.md](NATIVE_MODEL.md) |
| G5 | 111+ spec violations | [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md), [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) |

---

## Phases 101–110 — Freeze formal model (documentation)

| Phase | Title | Layer | Tag | Primary deliverable |
|------:|-------|-------|-----|---------------------|
| 101 | Compat syscall ABI freeze | kernel | compat | [ABI_SYSCALL.md](ABI_SYSCALL.md) `ares-abi-v1` |
| 102 | Memory contract freeze | kernel | compat | [ABI_MEMORY.md](ABI_MEMORY.md) |
| 103 | IPC + endpoint guarantees + E-* | kernel | native | [ABI_IPC.md](ABI_IPC.md) (G3) |
| 104 | Async OS contract | kernel | native | [ABI_ASYNC.md](ABI_ASYNC.md) |
| 105 | Security + axioms A1–A10 | kernel | governance | [ABI_SECURITY.md](ABI_SECURITY.md), [AXIOMS.md](AXIOMS.md) |
| 106 | Kernel object model | kernel | native | [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) (G1) |
| 107 | Formal rights algebra + R-* | kernel | native | [RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) (G2) |
| 108 | Revocation + temporal + T-* | kernel | governance | [TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) (G5) |
| 109 | Semantic index + lint + jurisdiction + observability outline | governance | governance | [SEMANTIC_SPECS.md](SEMANTIC_SPECS.md), [SEMANTIC_LINT.md](SEMANTIC_LINT.md), [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md), [SEMANTIC_OBSERVABILITY.md](SEMANTIC_OBSERVABILITY.md), [ABI_RUNTIME.md](ABI_RUNTIME.md), [ABI_STABILITY.md](ABI_STABILITY.md) |
| 110 | Constitutional sign-off | governance | governance | G1–G5; immutable identity; minimization audit; [NATIVE_DEVELOPER_EXPERIENCE.md](NATIVE_DEVELOPER_EXPERIENCE.md) outline |

Integration milestone: **110**.

**Status (documentation + validation):** phases 101–110 complete — `semantic_lint.py`, `python scripts/gate/legacy.py --phase 110 --timeout 180`, `AresOS-BootGate: name=constitutional ok=true` boot smoke.

---

## Phases 111–120 — Capabilities (implementation)

| Phase | Title | Layer | Tag | Notes |
|------:|-------|-------|-----|-------|
| 111 | `CapHandle` → `KernelObject` | kernel | native | G1+G5; single handle table |
| 112 | `cap_create` / `cap_close` / transfer | kernel | native | **G2** |
| 113 | Rights delegation smoke | kernel | native | R-01, R-06; no amplification |
| 114 | Storage grant object | kernel | native | No paths; FsNode cap |
| 115 | Path broker (**compat only**) | platform | compat | **G1** — no new handle semantics |
| 116 | No ambient authority | kernel | native | Zero grants → deny |
| 117 | Namespace invisibility | kernel | native | Native cannot enumerate global tree |
| 118 | Broker-issued `FsNode` caps | platform | native | Storage broker |
| 119 | Compat bridge unchanged | compat | compat | ELF + FD + path |
| 120 | Integration | kernel | governance | Cap + compat coexistence |

Integration milestone: **120**.

**Status (implementation + validation):** phases 111–120 complete — `kernel_object.rs`, `native_syscall.rs` (256–258 kernel-only), `path_broker.rs`, `storage_broker.rs`, `python scripts/gate/legacy.py --phase 120 --timeout 180`, `AresOS-BootGate: name=capabilities ok=true` boot smoke. Ring-3 native syscall allowlist deferred to phase 128 (G4).

---

## Phases 121–130 — Platform services

| Phase | Title | Layer | Tag |
|------:|-------|-------|-----|
| 121 | Service loader contract | platform | native | **complete** — `service_loader.rs` |
| 122 | Storage broker | platform | native | **complete** — IPC FS via `storage_broker` + interim bridge |
| 123 | Permission broker | platform | native | **complete** — `permission_broker.rs` |
| 124 | Device broker skeleton | platform | native | **complete** — `device_broker.rs` |
| 125 | Network broker stub | platform | native | **complete** — stub (`network_broker.rs`) |
| 126 | Clipboard broker stub | platform | native | **complete** — stub (`clipboard_broker.rs`) |
| 127 | Service crash isolation | platform | native | **complete** — tier-2 `service_isolation.rs` |
| 128 | Mandatory `ares-native-v1` manifest | platform | native | **complete** — G4 `native_manifest.rs` |
| 129 | Scoped grants in manifest | platform | native | **complete** — scoped grant validation |
| 130 | Integration | platform | governance | **complete** — `AresOS-BootGate: name=platform_brokers ok=true` smoke |

Integration milestone: **130**.

---

## Phases 131–140 — Immutable system + native async IPC

| Phase | Title | Layer | Tag |
|------:|-------|-------|-----|
| 131 | System image + identity epochs | platform | native |
| 132 | A/B slots | platform | native |
| 133 | Rollback smoke | platform | native |
| 134 | Endpoint object | kernel | native | G3 |
| 135 | Mailbox + structured cancel | kernel | native |
| 136 | Wait set over endpoints | kernel | native |
| 137 | Shared `MemoryRegion` cap IPC | kernel | native |
| 138 | Zero-copy message transfer | kernel | native |
| 139 | Compat PipeLite preserved | compat | compat |
| 140 | Integration | kernel | governance |

Integration milestone: **140**.

---

## Phases 141–150 — Scheduler, drivers, layer review

| Phase | Title | Layer | Tag |
|------:|-------|-------|-----|
| 141 | Service-centric scheduler spec | kernel | native |
| 142 | Endpoint-driven wake | kernel | native |
| 143 | Power/thermal policy stubs | kernel | native |
| 144 | Userspace driver host | platform | native |
| 145 | Compositor/GPU isolation story | platform | native |
| 146 | DMA cap + IOMMU narrative | kernel | native |
| 147 | Memory QoS per service | kernel | native |
| 148 | NUMA locality stub | kernel | native |
| 149 | Compression/THP policy doc | kernel | governance | deferred impl |
| 150 | Four-layer boundary review | governance | governance |

Integration milestone: **150**.

---

## Beyond 150

**Superseded:** phases 151–350 are defined in [ROADMAP_151_350.md](ROADMAP_151_350.md) (epochs 7–14, milestones 200/250/300/350).

---

## Phase 100 compat backlog (not native drivers)

TCP/UDP sockets; multi-fd select; full execve envp; file-backed COW; ACPI AP; IFUNC — track under **compat** milestones, not native identity.

---

## A10 minimization (ongoing)

Every new law after 110: [AXIOMS.md](AXIOMS.md) A10 review. Phase 110 records law count per hierarchy layer.
