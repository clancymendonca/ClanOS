"""Gate name ↔ serial-line patterns and scope-index → gate routing (ADR-0001 unified validation gate)."""

from __future__ import annotations

# Subsystem inventory — early phase runs before late phase at boot
# (single module since v2.0.0 unified merge; current gate version 2.1.0).
EARLY_SUBSYSTEMS: tuple[str, ...] = (
    "shell_storage",
    "loader_security",
    "memory_layout",
    "userspace_bootstrap",
    "hw_paging",
    "sched_userspace",
    "dynamic_runtime",
    "fd_mmap",
    "vm_fork",
    "syscall_ring3",
    "path_exec",
    "smp_depth",
    "constitutional",
    "capabilities",
    "service_loader",
    "platform_brokers",
    "build_endpoints",
    "virtio_blk",
    "network_compat",
    "scheduler_epoch",
    "boundary",
)

LATE_SUBSYSTEMS: tuple[str, ...] = (
    "integrity",
    "scheduling",
    "hardware",
    "federation",
    "release",
    "desktop_preview",
    "desktop",
    "compat_runtime",
    "compat_fd_vm",
    "compat_signal",
    "storage_depth",
    "posix_compat",
    "functional",
    "ci",
    "production",
    "network",
)

ALL_SUBSYSTEMS: tuple[str, ...] = EARLY_SUBSYSTEMS + LATE_SUBSYSTEMS

# Deprecated aliases (pre-ADR-0001 boot/system gate vocabulary).
BOOT_SUBSYSTEMS = EARLY_SUBSYSTEMS
SYSTEM_SUBSYSTEMS = LATE_SUBSYSTEMS

VALIDATION_GATES: dict[str, str] = {
    name: rf"ClanOS-Gate: name={name} ok=(true|false)" for name in ALL_SUBSYSTEMS
}
VALIDATION_GATES["all"] = r"ClanOS-Gate: ok=(true|false)"

# Deprecated summary aliases (still emitted during legacy alias epoch).
VALIDATION_GATES["boot"] = r"ClanOS-BootGate: ok=(true|false)"
VALIDATION_GATES["system"] = r"ClanOS-SystemGate: ok=(true|false)"

LEGACY_PATTERNS: dict[str, str] = {
    name: rf"ClanOS-BootGate: name={name} ok=(true|false)" for name in EARLY_SUBSYSTEMS
}
LEGACY_PATTERNS["boot"] = r"ClanOS-BootGate: ok=(true|false)"
LEGACY_PATTERNS["system"] = r"ClanOS-SystemGate: ok=(true|false)"

# Backward-compatible exports for older scripts.
BOOT_GATES: dict[str, str] = {
    **{k: LEGACY_PATTERNS[k] for k in EARLY_SUBSYSTEMS},
    "boot": LEGACY_PATTERNS["boot"],
}
SYSTEM_GATES: dict[str, str] = {
    **{k: VALIDATION_GATES[k] for k in LATE_SUBSYSTEMS},
    "system": LEGACY_PATTERNS["system"],
}

SCOPE_EARLY_GATE: dict[int, str] = {}
for n in range(6, 9):
    SCOPE_EARLY_GATE[n] = "shell_storage"
for n in range(9, 14):
    SCOPE_EARLY_GATE[n] = "loader_security"
for n in range(14, 17):
    SCOPE_EARLY_GATE[n] = "memory_layout"
for n in range(17, 21):
    SCOPE_EARLY_GATE[n] = "userspace_bootstrap"
for n in range(21, 31):
    SCOPE_EARLY_GATE[n] = "hw_paging"
for n in range(31, 41):
    SCOPE_EARLY_GATE[n] = "sched_userspace"
for n in range(41, 51):
    SCOPE_EARLY_GATE[n] = "dynamic_runtime"
for n in range(51, 61):
    SCOPE_EARLY_GATE[n] = "fd_mmap"
for n in range(61, 71):
    SCOPE_EARLY_GATE[n] = "vm_fork"
for n in range(71, 81):
    SCOPE_EARLY_GATE[n] = "syscall_ring3"
for n in range(81, 91):
    SCOPE_EARLY_GATE[n] = "path_exec"
for n in range(91, 101):
    SCOPE_EARLY_GATE[n] = "smp_depth"
SCOPE_EARLY_GATE.update(
    {
        110: "constitutional",
        120: "capabilities",
        121: "service_loader",
        130: "platform_brokers",
        134: "build_endpoints",
        140: "build_endpoints",
        149: "scheduler_epoch",
        150: "boundary",
        201: "virtio_blk",
        404: "network_compat",
    }
)

SCOPE_LATE_GATE: dict[int, str] = {
    175: "integrity",
    200: "scheduling",
    250: "hardware",
    300: "federation",
    350: "release",
    351: "desktop_preview",
    375: "desktop",
    400: "functional",
    425: "ci",
    450: "production",
    475: "network",
    500: "all",
}

# Deprecated aliases (pre-ADR-0001 scope routing names).
SCOPE_BOOT_GATE = SCOPE_EARLY_GATE
SCOPE_SYSTEM_GATE = SCOPE_LATE_GATE

PREEMPTION_GATES = frozenset({"all", "boot", "boundary", "shell_storage", "system"})

LEGACY_MILESTONE_BOOT = SCOPE_EARLY_GATE
LEGACY_MILESTONE_SYSTEM = SCOPE_LATE_GATE


def gate_for_scope(scope: int) -> str | None:
    if scope in SCOPE_LATE_GATE:
        return SCOPE_LATE_GATE[scope]
    return SCOPE_EARLY_GATE.get(scope)


def pattern_for_gate(gate: str) -> str:
    """Primary + legacy serial patterns combined for QEMU matching."""
    if gate not in VALIDATION_GATES:
        raise KeyError(gate)
    parts = [VALIDATION_GATES[gate]]
    legacy = LEGACY_PATTERNS.get(gate)
    if legacy and legacy not in parts:
        parts.append(legacy)
    if len(parts) == 1:
        return parts[0]
    return "|".join(f"(?:{p})" for p in parts)


def gate_cli(scope: int, timeout: int = 180) -> str:
    gate = gate_for_scope(scope)
    if gate is None:
        return f"python scripts/gate/run.py --gate all --timeout {timeout}"
    return f"python scripts/gate/run.py --gate {gate} --timeout {timeout}"


def scope_index_gate(scope: int) -> str | None:
    """Map scope index to subsystem gate name."""
    return gate_for_scope(scope)


def boot_gate_for_legacy_milestone(milestone: int) -> str | None:
    return SCOPE_EARLY_GATE.get(milestone)


def system_gate_for_legacy_milestone(milestone: int) -> str | None:
    g = SCOPE_LATE_GATE.get(milestone)
    if g == "all":
        return "system"
    return g


def gate_family(gate: str) -> str:
    """Deprecated — all gates route through run.py."""
    if gate in ("boot",) or gate in EARLY_SUBSYSTEMS:
        return "boot"
    if gate in ("system", "all") or gate in LATE_SUBSYSTEMS:
        return "system"
    return "validation"
