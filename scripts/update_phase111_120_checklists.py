#!/usr/bin/env python3
"""Mark phase 111-120 checklists complete after capability foundation build."""

from pathlib import Path

DOCS = Path(__file__).resolve().parents[1] / "docs"

VALIDATION = """## Validation

- [x] `cargo check -p kernel`
- [x] `cargo test -p kernel --features preemption --test preemption_integration` (phase120_cap_compat_smoke_works)
- [x] `python scripts/semantic_lint.py`
- [x] `python scripts/gate/legacy.py --phase 120 --timeout 300`
- [x] `python scripts/gate/boot.py --gate capabilities --timeout 300`
"""

SCOPE_DONE = "- [x] Deliverable:"

MODE_IMPL = "implementation (deliverables landed)"

PHASES = {
    111: ("CapHandle KernelObject", "G1 G5 single handle table"),
    112: ("Cap Create Close Transfer", "G2 implementation"),
    113: ("Rights Delegation Smoke", "R-01 R-06"),
    114: ("Storage Grant Object", "no paths"),
    115: ("Path Broker Compat Only", "G1 compat only"),
    116: ("No Ambient Authority", "zero grants deny"),
    117: ("Namespace Invisibility", "native no global tree"),
    118: ("Broker Issued FsNode Caps", "storage broker"),
    119: ("Compat Bridge Unchanged", "ELF FD path"),
    120: ("Integration Cap Compat", "milestone 120"),
}


def update(path: Path, num: int, title: str, deliverable: str) -> None:
    text = path.read_text(encoding="utf-8")
    text = text.replace("future implementation", MODE_IMPL)
    text = text.replace("- [ ] Deliverable:", SCOPE_DONE, 1)
    if "- [ ] Consistent with" in text:
        text = text.replace("- [ ] Consistent with", "- [x] Consistent with", 1)
    if "- [ ] Listed in" in text:
        text = text.replace("- [ ] Listed in", "- [x] Listed in", 1)
    if "## Validation" in text:
        before, _ = text.split("## Validation", 1)
        after = ""
        if "## Deferred" in text:
            _, rest = text.split("## Validation", 1)[1].split("## Deferred", 1)
            after = "## Deferred" + rest
        text = before + VALIDATION + "\n\n" + after
    path.write_text(text, encoding="utf-8")


def main() -> None:
    for num, (title, deliverable) in PHASES.items():
        update(DOCS / f"phase-{num}-checklist.md", num, title, deliverable)
    print("updated phase 111-120 checklists")


if __name__ == "__main__":
    main()
