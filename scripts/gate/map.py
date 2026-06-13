"""Gate name ↔ serial-line patterns and legacy phase routing."""

from __future__ import annotations

BOOT_GATES: dict[str, str] = {
    "shell_storage": r"AresOS-BootGate: name=shell_storage ok=(true|false)",
    "loader_security": r"AresOS-BootGate: name=loader_security ok=(true|false)",
    "memory_layout": r"AresOS-BootGate: name=memory_layout ok=(true|false)",
    "userspace_bootstrap": r"AresOS-BootGate: name=userspace_bootstrap ok=(true|false)",
    "hw_paging": r"AresOS-BootGate: name=hw_paging ok=(true|false)",
    "sched_userspace": r"AresOS-BootGate: name=sched_userspace ok=(true|false)",
    "dynamic_runtime": r"AresOS-BootGate: name=dynamic_runtime ok=(true|false)",
    "fd_mmap": r"AresOS-BootGate: name=fd_mmap ok=(true|false)",
    "vm_fork": r"AresOS-BootGate: name=vm_fork ok=(true|false)",
    "syscall_ring3": r"AresOS-BootGate: name=syscall_ring3 ok=(true|false)",
    "path_exec": r"AresOS-BootGate: name=path_exec ok=(true|false)",
    "smp_depth": r"AresOS-BootGate: name=smp_depth ok=(true|false)",
    "constitutional": r"AresOS-BootGate: name=constitutional ok=(true|false)",
    "capabilities": r"AresOS-BootGate: name=capabilities ok=(true|false)",
    "service_loader": r"AresOS-BootGate: name=service_loader ok=(true|false)",
    "platform_brokers": r"AresOS-BootGate: name=platform_brokers ok=(true|false)",
    "build_endpoints": r"AresOS-BootGate: name=build_endpoints ok=(true|false)",
    "virtio_blk": r"AresOS-BootGate: name=virtio_blk ok=(true|false)",
    "network_compat": r"AresOS-BootGate: name=network_compat ok=(true|false)",
    "scheduler_epoch": r"AresOS-BootGate: name=scheduler_epoch ok=(true|false)",
    "boundary": r"AresOS-BootGate: name=boundary ok=(true|false)",
    "boot": r"AresOS-BootGate: ok=(true|false)",
}

SYSTEM_GATES: dict[str, str] = {
    "integrity": r"AresOS-Gate: name=integrity ok=(true|false)",
    "scheduling": r"AresOS-Gate: name=scheduling ok=(true|false)",
    "hardware": r"AresOS-Gate: name=hardware ok=(true|false)",
    "federation": r"AresOS-Gate: name=federation ok=(true|false)",
    "release": r"AresOS-Gate: name=release ok=(true|false)",
    "desktop_preview": r"AresOS-Gate: name=desktop_preview ok=(true|false)",
    "desktop": r"AresOS-Gate: name=desktop ok=(true|false)",
    "functional": r"AresOS-Gate: name=functional ok=(true|false)",
    "ci": r"AresOS-Gate: name=ci ok=(true|false)",
    "production": r"AresOS-Gate: name=production ok=(true|false)",
    "network": r"AresOS-Gate: name=network ok=(true|false)",
    "system": r"AresOS-SystemGate: ok=(true|false)",
}

BOOT_PHASE: dict[int, str] = {}
for n in range(6, 9):
    BOOT_PHASE[n] = "shell_storage"
for n in range(9, 14):
    BOOT_PHASE[n] = "loader_security"
for n in range(14, 17):
    BOOT_PHASE[n] = "memory_layout"
for n in range(17, 21):
    BOOT_PHASE[n] = "userspace_bootstrap"
for n in range(21, 31):
    BOOT_PHASE[n] = "hw_paging"
for n in range(31, 41):
    BOOT_PHASE[n] = "sched_userspace"
for n in range(41, 51):
    BOOT_PHASE[n] = "dynamic_runtime"
for n in range(51, 61):
    BOOT_PHASE[n] = "fd_mmap"
for n in range(61, 71):
    BOOT_PHASE[n] = "vm_fork"
for n in range(71, 81):
    BOOT_PHASE[n] = "syscall_ring3"
for n in range(81, 91):
    BOOT_PHASE[n] = "path_exec"
for n in range(91, 101):
    BOOT_PHASE[n] = "smp_depth"
BOOT_PHASE.update(
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

SYSTEM_PHASE: dict[int, str] = {
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


def boot_gate_for_phase(phase: int) -> str | None:
    return BOOT_PHASE.get(phase)


def system_gate_for_phase(phase: int) -> str | None:
    return SYSTEM_PHASE.get(phase)
