#!/usr/bin/env python3
"""Migrate phase checklist docs and deep-dive guides to unified gate references."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"

BANNER = """> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md). Legacy `PhaseN-*` boot serial lines are retired.

"""

PHASE_GATE: dict[int, str] = {}
for n in range(6, 9):
    PHASE_GATE[n] = "shell_storage"
for n in range(9, 14):
    PHASE_GATE[n] = "loader_security"
for n in range(14, 17):
    PHASE_GATE[n] = "memory_layout"
for n in range(17, 21):
    PHASE_GATE[n] = "userspace_bootstrap"
for n in range(21, 31):
    PHASE_GATE[n] = "hw_paging"
for n in range(31, 41):
    PHASE_GATE[n] = "sched_userspace"
for n in range(41, 51):
    PHASE_GATE[n] = "dynamic_runtime"
for n in range(51, 61):
    PHASE_GATE[n] = "fd_mmap"
for n in range(61, 71):
    PHASE_GATE[n] = "vm_fork"
for n in range(71, 81):
    PHASE_GATE[n] = "syscall_ring3"
for n in range(81, 91):
    PHASE_GATE[n] = "path_exec"
for n in range(91, 101):
    PHASE_GATE[n] = "smp_depth"
PHASE_GATE.update(
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
)

SYSTEM_PHASES = set(range(151, 501)) | {175, 200, 250, 300, 350, 351, 375, 400, 425, 450, 475, 500}

PHASE_SMOKE_RE = re.compile(r"`Phase\d+[^`]*`")
PHASE_SMOKE_LINE_RE = re.compile(r"Phase\d+-[\w]+[^\n]*")


def phase_num_from_path(path: Path) -> int | None:
    m = re.match(r"phase-(\d+)-checklist\.md", path.name)
    return int(m.group(1)) if m else None


def gate_line(phase: int) -> str:
    gate = PHASE_GATE.get(phase)
    if gate is None:
        return "unified boot/system gate (see VALIDATION_GATES.md)"
    if phase in SYSTEM_PHASES and phase >= 151:
        return f"system gate `{gate}` (`AresOS-Gate: name={gate} ok=true` or `AresOS-SystemGate`)"
    return f"boot gate `{gate}` (`AresOS-BootGate: name={gate} ok=true`)"


def validation_block(phase: int) -> str:
    if phase in SYSTEM_PHASES and phase >= 175:
        gate = PHASE_GATE.get(phase, "system")
        if gate == "system":
            cmd = "python scripts/gate/system.py --gate system --timeout 360"
        else:
            cmd = f"python scripts/gate/system.py --gate {gate} --timeout 360"
    elif phase in PHASE_GATE:
        cmd = f"python scripts/gate/boot.py --phase {phase} --timeout 180"
    else:
        cmd = "python scripts/gate/boot.py --gate boot --timeout 360"
    return f"""## Validation

```bash
cargo check -p kernel
{cmd}
python scripts/validation_matrix.py --smoke-timeout 180
```

See [VALIDATION_GATES.md](VALIDATION_GATES.md).
"""


def patch_checklist(path: Path) -> bool:
    phase = phase_num_from_path(path)
    if phase is None:
        return False
    text = path.read_text(encoding="utf-8")
    if "VALIDATION_GATES.md" in text.split("\n", 1)[0]:
        return False

    # Banner
    if not text.startswith("> **Historical"):
        text = BANNER + text

    # Emit PhaseN smoke checklist items
    text = re.sub(
        r"- \[x\] Emit `Phase\d+[^`]*` boot smoke[^\n]*",
        lambda m: f"- [x] Covered by {gate_line(phase)}",
        text,
    )
    text = re.sub(
        r"- \[x\] `Phase\d+[^`]*` boot[^\n]*",
        lambda m: f"- [x] Covered by {gate_line(phase)}",
        text,
    )
    text = re.sub(
        r"- \[x\].*`Phase\d+[^`]*`[^\n]*",
        lambda m: f"- [x] Covered by {gate_line(phase)}",
        text,
    )

    # Script references
    text = re.sub(
        r"`scripts/phase\d+[^`]*`",
        f"`scripts/gate/boot.py --phase {phase}`",
        text,
    )
    text = re.sub(
        r"python scripts/phase\d+_\w+\.py[^\n]*",
        f"python scripts/gate/boot.py --phase {phase} --timeout 180",
        text,
    )
    text = re.sub(
        r"validation_matrix\.py` includes `phase\d+[^`]*`",
        "validation_matrix.py` includes `boot-gate-check`",
        text,
    )

    # Replace ## Validation Commands / ## Validation section
    text = re.sub(
        r"## Validation Commands\n\n```bash\n[\s\S]*?```\n",
        validation_block(phase) + "\n",
        text,
        count=1,
    )
    if "## Validation\n" not in text and "## Validation Commands" not in text:
        text = text.rstrip() + "\n\n" + validation_block(phase)

    path.write_text(text, encoding="utf-8")
    return True


def patch_guide(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    orig = text

    replacements = {
        "Phase175-Epoch7": "AresOS-Gate: name=integrity ok=true",
        "Phase200-Milestone": "AresOS-Gate: name=scheduling ok=true",
        "Phase250-Milestone": "AresOS-Gate: name=hardware ok=true",
        "Phase300-Milestone": "AresOS-Gate: name=federation ok=true",
        "Phase350-Milestone": "AresOS-Gate: name=release ok=true",
        "Phase351-Desktop": "AresOS-Gate: name=desktop_preview ok=true",
        "Phase375-Milestone": "AresOS-Gate: name=desktop ok=true",
        "Phase400-Milestone": "AresOS-Gate: name=functional ok=true",
        "Phase425-Milestone": "AresOS-Gate: name=ci ok=true",
        "Phase450-Milestone": "AresOS-Gate: name=production ok=true",
        "Phase475-Milestone": "AresOS-Gate: name=network ok=true",
        "Phase500-Milestone": "AresOS-SystemGate: ok=true",
        "Phase150-Milestone": "AresOS-BootGate: name=boundary ok=true",
        "Phase149-Epoch5": "AresOS-BootGate: name=scheduler_epoch ok=true",
        "Phase404-Network": "AresOS-BootGate: name=network_compat ok=true",
        "Phase201-VirtioBlk": "AresOS-BootGate: name=virtio_blk ok=true",
        "Phase140-IPC": "AresOS-BootGate: name=build_endpoints ok=true",
        "Phase130-Platform": "AresOS-BootGate: name=platform_brokers ok=true",
        "Phase120-CapCompat": "AresOS-BootGate: name=capabilities ok=true",
        "Phase110-Constitutional": "AresOS-BootGate: name=constitutional ok=true",
        "COMPLETED_PHASE": "SYSTEM_GATE_VERSION / BOOT_GATE_VERSION",
        "phase_catalog.rs": "system_gate.rs / boot_gate.rs",
    }
    for old, new in replacements.items():
        text = text.replace(old, new)

    text = PHASE_SMOKE_LINE_RE.sub(
        "See [VALIDATION_GATES.md](VALIDATION_GATES.md) for gate serial lines.",
        text,
    )
    if text != orig:
        path.write_text(text, encoding="utf-8")
        return True
    return False


def main() -> int:
    checklists = 0
    for p in sorted(DOCS.glob("phase-*-checklist.md")):
        if patch_checklist(p):
            checklists += 1

    guides = 0
    for name in (
        "STORAGE.md",
        "PROGRAMS.md",
        "EXECUTABLE_IMAGES.md",
        "LOAD_PLANS.md",
        "MAPPING_STUBS.md",
        "FRAME_OWNERSHIP.md",
        "FRAME_BACKED_IMAGES.md",
        "DEVICES.md",
        "SECURITY.md",
        "DEMAND_PAGING.md",
        "SHARED_LIBRARIES.md",
        "FILE_DESCRIPTORS.md",
        "USER_SYSCALLS.md",
        "USER_ELF_MVP.md",
        "USER_PAGE_TABLES.md",
        "USER_CONTEXT.md",
        "RING3_TRAMPOLINE.md",
        "SMP.md",
        "ROADMAP_POST100.md",
        "ROADMAP_151_350.md",
        "ROADMAP_351_400.md",
        "ROADMAP_401_500.md",
        "DESIGN_NORTH_STAR.md",
        "RELEASE_SCORECARD_M400.md",
    ):
        p = DOCS / name
        if p.exists() and patch_guide(p):
            guides += 1

    print(f"migrate_phase_docs: {checklists} checklists, {guides} guides")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
