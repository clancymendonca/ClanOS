#!/usr/bin/env python3
"""Generate scope-151..350 checklist stubs."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "docs"

# Explicit titles for integration / milestone scopes
EXPLICIT: dict[int, tuple[str, str, str, str]] = {
    151: ("Loom ENDPOINT_QUEUES Harness", "kernel", "native", "loom_harness_registry ENDPOINT_QUEUES"),
    155: ("S-01 Executable Spec Case", "kernel", "native", "SCHEDULING_UNIFIED S-01"),
    160: ("Epoch 7 OOM Integration", "kernel", "governance", "scope147 OOM enforcement"),
    165: ("Epoch 7 Audit Build Gate", "governance", "governance", "audit tamper + dual-build"),
    170: ("Epoch 7 Evidence Gate", "governance", "governance", "Kani mandatory CI"),
    175: ("Epoch 7 Signoff", "governance", "governance", "system gate integrity"),
    200: ("Milestone 200 Gate", "governance", "governance", "system gate scheduling"),
    250: ("Milestone 250 Gate", "governance", "governance", "system gate hardware"),
    300: ("Milestone 300 Gate", "governance", "governance", "system gate federation"),
    350: ("Milestone 350 Release 1.0", "governance", "governance", "system gate release"),
}

BANDS = [
    (151, 175, "Epoch 7 stub graduation", "kernel"),
    (176, 200, "Epoch 8 scheduling", "kernel"),
    (201, 225, "Epoch 9 native SDK", "platform"),
    (226, 250, "Epoch 10 hardware", "platform"),
    (251, 275, "Epoch 11 drivers", "platform"),
    (276, 300, "Epoch 12 federation", "kernel"),
    (301, 325, "Epoch 13 checkpoint formal", "kernel"),
    (326, 350, "Epoch 14 release", "governance"),
]


def title_for(n: int) -> tuple[str, str, str, str]:
    if n in EXPLICIT:
        return EXPLICIT[n]
    for lo, hi, band, layer in BANDS:
        if lo <= n <= hi:
            return (f"{band} scope {n}", layer, "native", f"ROADMAP_151_350 band {lo}-{hi}")
    return (f"Post-150 scope {n}", "kernel", "native", "ROADMAP_151_350")


def main() -> None:
    for n in range(151, 351):
        title, layer, tag, deliv = title_for(n)
        path = ROOT / f"scope-{n}-checklist.md"
        path.write_text(
            f"""> **Historical scope checklist.** Runtime validation uses unified gates — see [VALIDATION_GATES.md](VALIDATION_GATES.md).

# Scope {n} Checklist: {title}

## Layer
{layer}

## Tag
{tag}

## Mode
future implementation

## Scope

- [ ] Deliverable: {deliv}
- [ ] Consistent with [AXIOMS.md](AXIOMS.md)
- [ ] Listed in [ROADMAP_151_350.md](ROADMAP_151_350.md)

## Validation

- [ ] `cargo check -p kernel`
- [ ] Gate check via [VALIDATION_GATES.md](VALIDATION_GATES.md)

## Deferred

- See [ROADMAP_151_350.md](ROADMAP_151_350.md) epoch bands.
""",
            encoding="utf-8",
        )
    print(f"wrote 200 checklists (151-350) to {ROOT}")


if __name__ == "__main__":
    main()
