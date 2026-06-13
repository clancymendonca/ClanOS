"""Gate name ↔ serial-line patterns and scope-index → gate routing."""

from __future__ import annotations

BOOT_GATES: dict[str, str] = {
    "shell_storage": r"ClanOS-BootGate: name=shell_storage ok=(true|false)",
    "loader_security": r"ClanOS-BootGate: name=loader_security ok=(true|false)",
    "memory_layout": r"ClanOS-BootGate: name=memory_layout ok=(true|false)",
    "userspace_bootstrap": r"ClanOS-BootGate: name=userspace_bootstrap ok=(true|false)",
    "hw_paging": r"ClanOS-BootGate: name=hw_paging ok=(true|false)",
    "sched_userspace": r"ClanOS-BootGate: name=sched_userspace ok=(true|false)",
    "dynamic_runtime": r"ClanOS-BootGate: name=dynamic_runtime ok=(true|false)",
    "fd_mmap": r"ClanOS-BootGate: name=fd_mmap ok=(true|false)",
    "vm_fork": r"ClanOS-BootGate: name=vm_fork ok=(true|false)",
    "syscall_ring3": r"ClanOS-BootGate: name=syscall_ring3 ok=(true|false)",
    "path_exec": r"ClanOS-BootGate: name=path_exec ok=(true|false)",
    "smp_depth": r"ClanOS-BootGate: name=smp_depth ok=(true|false)",
    "constitutional": r"ClanOS-BootGate: name=constitutional ok=(true|false)",
    "capabilities": r"ClanOS-BootGate: name=capabilities ok=(true|false)",
    "service_loader": r"ClanOS-BootGate: name=service_loader ok=(true|false)",
    "platform_brokers": r"ClanOS-BootGate: name=platform_brokers ok=(true|false)",
    "build_endpoints": r"ClanOS-BootGate: name=build_endpoints ok=(true|false)",
    "virtio_blk": r"ClanOS-BootGate: name=virtio_blk ok=(true|false)",
    "network_compat": r"ClanOS-BootGate: name=network_compat ok=(true|false)",
    "scheduler_epoch": r"ClanOS-BootGate: name=scheduler_epoch ok=(true|false)",
    "boundary": r"ClanOS-BootGate: name=boundary ok=(true|false)",
    "boot": r"ClanOS-BootGate: ok=(true|false)",
}

SYSTEM_GATES: dict[str, str] = {
    "integrity": r"ClanOS-Gate: name=integrity ok=(true|false)",
    "scheduling": r"ClanOS-Gate: name=scheduling ok=(true|false)",
    "hardware": r"ClanOS-Gate: name=hardware ok=(true|false)",
    "federation": r"ClanOS-Gate: name=federation ok=(true|false)",
    "release": r"ClanOS-Gate: name=release ok=(true|false)",
    "desktop_preview": r"ClanOS-Gate: name=desktop_preview ok=(true|false)",
    "desktop": r"ClanOS-Gate: name=desktop ok=(true|false)",
    "compat_runtime": r"ClanOS-Gate: name=compat_runtime ok=(true|false)",
    "compat_fd_vm": r"ClanOS-Gate: name=compat_fd_vm ok=(true|false)",
    "compat_signal": r"ClanOS-Gate: name=compat_signal ok=(true|false)",
    "storage_depth": r"ClanOS-Gate: name=storage_depth ok=(true|false)",
    "posix_compat": r"ClanOS-Gate: name=posix_compat ok=(true|false)",
    "functional": r"ClanOS-Gate: name=functional ok=(true|false)",
    "ci": r"ClanOS-Gate: name=ci ok=(true|false)",
    "production": r"ClanOS-Gate: name=production ok=(true|false)",
    "network": r"ClanOS-Gate: name=network ok=(true|false)",
    "system": r"ClanOS-SystemGate: ok=(true|false)",
}

SCOPE_BOOT_GATE: dict[int, str] = {}
for n in range(6, 9):
    SCOPE_BOOT_GATE[n] = "shell_storage"
for n in range(9, 14):
    SCOPE_BOOT_GATE[n] = "loader_security"
for n in range(14, 17):
    SCOPE_BOOT_GATE[n] = "memory_layout"
for n in range(17, 21):
    SCOPE_BOOT_GATE[n] = "userspace_bootstrap"
for n in range(21, 31):
    SCOPE_BOOT_GATE[n] = "hw_paging"
for n in range(31, 41):
    SCOPE_BOOT_GATE[n] = "sched_userspace"
for n in range(41, 51):
    SCOPE_BOOT_GATE[n] = "dynamic_runtime"
for n in range(51, 61):
    SCOPE_BOOT_GATE[n] = "fd_mmap"
for n in range(61, 71):
    SCOPE_BOOT_GATE[n] = "vm_fork"
for n in range(71, 81):
    SCOPE_BOOT_GATE[n] = "syscall_ring3"
for n in range(81, 91):
    SCOPE_BOOT_GATE[n] = "path_exec"
for n in range(91, 101):
    SCOPE_BOOT_GATE[n] = "smp_depth"
SCOPE_BOOT_GATE.update(
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

SCOPE_SYSTEM_GATE: dict[int, str] = {
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
    500: "system",
}

PREEMPTION_BOOT_GATES = frozenset({"boot", "boundary", "shell_storage"})

# Deprecated aliases (pre-consolidation scripts).
LEGACY_MILESTONE_BOOT = SCOPE_BOOT_GATE
LEGACY_MILESTONE_SYSTEM = SCOPE_SYSTEM_GATE


def gate_for_scope(scope: int) -> str | None:
    if scope in SCOPE_SYSTEM_GATE:
        return SCOPE_SYSTEM_GATE[scope]
    return SCOPE_BOOT_GATE.get(scope)


def gate_family(gate: str) -> str:
    if gate in SYSTEM_GATES:
        return "system"
    return "boot"


def gate_cli(scope: int, timeout: int = 180) -> str:
    gate = gate_for_scope(scope)
    if gate is None:
        return f"python scripts/gate/boot.py --gate boot --timeout {timeout}"
    if gate_family(gate) == "system":
        return f"python scripts/gate/system.py --gate {gate} --timeout {timeout}"
    return f"python scripts/gate/boot.py --gate {gate} --timeout {timeout}"


def boot_gate_for_legacy_milestone(milestone: int) -> str | None:
    return SCOPE_BOOT_GATE.get(milestone)


def system_gate_for_legacy_milestone(milestone: int) -> str | None:
    return SCOPE_SYSTEM_GATE.get(milestone)
