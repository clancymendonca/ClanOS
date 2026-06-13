#!/usr/bin/env python3
"""Generate scope-101..150 checklist stubs (one-time doc pass)."""

from pathlib import Path

SCOPES = [
    (101, "Compat Syscall ABI Freeze", "governance", "compat", "ABI_SYSCALL.md clan-abi-v1"),
    (102, "Memory Contract Freeze", "kernel", "compat", "ABI_MEMORY.md"),
    (103, "IPC Endpoint Guarantees", "kernel", "native", "ABI_IPC.md G3 E-*"),
    (104, "Async OS Contract", "kernel", "native", "ABI_ASYNC.md"),
    (105, "Security and AXIOMS A1-A10", "governance", "governance", "ABI_SECURITY.md AXIOMS.md"),
    (106, "Kernel Object Model", "kernel", "native", "KERNEL_OBJECT_MODEL.md G1 immutable identity"),
    (107, "Formal Rights Algebra", "kernel", "native", "RIGHTS_ALGEBRA.md G2 R-*"),
    (108, "Revocation and Temporal Semantics", "kernel", "governance", "TEMPORAL_SEMANTICS.md G5 T-*"),
    (109, "Semantic Index Lint Jurisdiction", "governance", "governance", "SEMANTIC_SPECS SEMANTIC_LINT SEMANTIC_JURISDICTION"),
    (110, "Constitutional Sign-Off", "governance", "governance", "G1-G5 minimization audit"),
    (111, "CapHandle KernelObject", "kernel", "native", "G1 G5 single handle table"),
    (112, "Cap Create Close Transfer", "kernel", "native", "G2 implementation"),
    (113, "Rights Delegation Smoke", "kernel", "native", "R-01 R-06"),
    (114, "Storage Grant Object", "kernel", "native", "no paths"),
    (115, "Path Broker Compat Only", "platform", "compat", "G1 compat only"),
    (116, "No Ambient Authority", "kernel", "native", "zero grants deny"),
    (117, "Namespace Invisibility", "kernel", "native", "native no global tree"),
    (118, "Broker Issued FsNode Caps", "platform", "native", "storage broker"),
    (119, "Compat Bridge Unchanged", "compat", "compat", "ELF FD path"),
    (120, "Integration Cap Compat", "kernel", "governance", "milestone 120"),
    (121, "Service Loader Contract", "platform", "native", "service-centric"),
    (122, "Storage Broker", "platform", "native", "IPC FS"),
    (123, "Permission Broker", "platform", "native", "clan-native-v1"),
    (124, "Device Broker Skeleton", "platform", "native", "distrustful device"),
    (125, "Network Broker Stub", "platform", "native", ""),
    (126, "Clipboard Broker Stub", "platform", "native", ""),
    (127, "Service Crash Isolation", "platform", "native", ""),
    (128, "Mandatory Native Manifest", "platform", "native", "G4"),
    (129, "Scoped Grants Manifest", "platform", "native", ""),
    (130, "Integration Platform", "platform", "governance", "milestone 130"),
    (131, "System Image Identity Epochs", "platform", "native", ""),
    (132, "A/B Slots", "platform", "native", ""),
    (133, "Rollback Smoke", "platform", "native", ""),
    (134, "Endpoint Object", "kernel", "native", "G3 E-*"),
    (135, "Mailbox Structured Cancel", "kernel", "native", ""),
    (136, "Wait Set Endpoints", "kernel", "native", ""),
    (137, "MemoryRegion Cap IPC", "kernel", "native", ""),
    (138, "Zero-Copy Transfer", "kernel", "native", ""),
    (139, "Compat PipeLite Preserved", "compat", "compat", "E compat"),
    (140, "Integration Immutable Async IPC", "kernel", "governance", "milestone 140"),
    (141, "Service-Centric Scheduler Spec", "kernel", "native", "S-01 outline"),
    (142, "Endpoint-Driven Wake", "kernel", "native", ""),
    (143, "Power Thermal Stubs", "kernel", "native", ""),
    (144, "Userspace Driver Host", "platform", "native", ""),
    (145, "Compositor GPU Isolation", "platform", "native", ""),
    (146, "DMA Cap IOMMU Narrative", "kernel", "native", ""),
    (147, "Memory QoS Per Service", "kernel", "native", ""),
    (148, "NUMA Locality Stub", "kernel", "native", ""),
    (149, "Compression THP Policy Doc", "kernel", "governance", "deferred impl"),
    (150, "Four-Layer Boundary Review", "governance", "governance", "milestone 150"),
]

ROOT = Path(__file__).resolve().parents[1] / "docs"


def extra_gates(n: int) -> str:
    if n == 110:
        return """
## Constitutional sign-off

- [ ] G1–G5 satisfied ([AXIOMS.md](AXIOMS.md))
- [ ] AXIOMS A1–A10 ratified
- [ ] Immutable identity + generation adopted ([KERNEL_OBJECT_MODEL.md](KERNEL_OBJECT_MODEL.md))
- [ ] Minimization audit recorded (A10)
- [ ] [SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md) ratified
- [ ] Law ↔ spec case matrix for R-/E-/T- ([SEMANTIC_SPECS.md](SEMANTIC_SPECS.md))
"""
    if n == 106:
        return "\n## Gate G1\n\nNo new handle semantics after scope 115.\n"
    if n in (107, 112, 113):
        return "\n## Gate G2\n\n[RIGHTS_ALGEBRA.md](RIGHTS_ALGEBRA.md) required before cap implementation.\n"
    if n in (103, 134):
        return "\n## Gate G3\n\n[ABI_IPC.md](ABI_IPC.md) required before endpoint implementation.\n"
    if n == 108:
        return "\n## Gate G5\n\n[TEMPORAL_SEMANTICS.md](TEMPORAL_SEMANTICS.md) + spec cases.\n"
    if n == 115:
        return "\n## Gate G1\n\nPath broker is compat-only; no new handle semantics.\n"
    if n == 128:
        return "\n## Gate G4\n\n[NATIVE_MODEL.md](NATIVE_MODEL.md) native definition.\n"
    return ""


def main() -> None:
    for n, title, layer, tag, deliv in SCOPES:
        impl = "documentation" if n <= 110 else "future implementation"
        path = ROOT / f"scope-{n}-checklist.md"
        path.write_text(
            f"""# Scope {n} Checklist: {title}

## Layer
{layer}

## Tag
{tag}

## Mode
{impl}

## Scope

- [ ] Deliverable: {deliv}
- [ ] Consistent with [AXIOMS.md](AXIOMS.md)
- [ ] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)
{extra_gates(n)}
## Validation

- [ ] Scopes 101–110: documentation review (no kernel change required)
- [ ] Scopes 111+: `cargo check -p kernel` + gate check via [VALIDATION_GATES.md](VALIDATION_GATES.md)

## Deferred

- See [ROADMAP_POST100.md](ROADMAP_POST100.md) and [AXIOMS.md](AXIOMS.md) gates.
""",
            encoding="utf-8",
        )
    print(f"wrote {len(SCOPES)} checklists to {ROOT}")


if __name__ == "__main__":
    main()
