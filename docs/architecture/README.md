# Architecture documentation index

```yaml
status: authoritative
version: 0.1.0
epoch: 14
authored_by: architecture
```

Canonical architecture docs live here per the Clan OS repository structure spec. During the M400 era, many authoritative specs remain at flat `docs/*.md` paths; this directory is the migration target. Cross-references in new docs should prefer paths under `docs/architecture/`.

## Authoritative in this directory

| Document | Flat `docs/` status |
|----------|---------------------|
| [KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md) | `docs/KERNEL_OBJECT_MODEL.md` → **superseded-by** (reconcile at squash) |
| [SECURITY_MODEL.md](SECURITY_MODEL.md) | No flat equivalent yet |
| [DESIGN_NORTH_STAR.md](DESIGN_NORTH_STAR.md) | `docs/DESIGN_NORTH_STAR.md` → **superseded-by** (scorecard rows pending reconcile) |

## Redirected (flat `docs/` canonical until migrated)

| Spec path | Current canonical path |
|-----------|------------------------|
| SCHEDULER_MODEL.md | [../SCHEDULER_MODEL.md](../SCHEDULER_MODEL.md) |
| MEMORY_MODEL.md | [../FRAME_OWNERSHIP.md](../FRAME_OWNERSHIP.md), [../USER_PAGE_TABLES.md](../USER_PAGE_TABLES.md) |
| IPC_MODEL.md | [../ABI_IPC.md](../ABI_IPC.md), [../CAP_TRANSFER_PROTOCOL.md](../CAP_TRANSFER_PROTOCOL.md) |
| FILESYSTEM_MODEL.md | [../STORAGE.md](../STORAGE.md) |
| NETWORK_MODEL.md | [../EPOCH4_NETWORK_PLAN.md](../EPOCH4_NETWORK_PLAN.md) |
| AI_SUBSYSTEM.md | Deferred post-M400 |
| THREAT_MODEL.md | [../THREAT_MODEL.md](../THREAT_MODEL.md) |
| FAULT_ESCALATION.md | [../FAULT_ESCALATION.md](../FAULT_ESCALATION.md) |

## ADRs

Architecture Decision Records: `docs/architecture/ADR/` (create before implementation tradeoffs).

## Specs

Wire formats and ABI: `docs/specs/` (target); current canonical: flat `docs/ABI_*.md`, `docs/ERROR_TAXONOMY.md`.
