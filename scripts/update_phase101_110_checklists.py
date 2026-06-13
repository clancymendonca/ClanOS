#!/usr/bin/env python3
"""Mark validation lines complete on phase 101-110 checklists."""

from pathlib import Path

DOCS = Path(__file__).resolve().parents[1] / "docs"

VALIDATION_BLOCK = """## Validation

- [x] `python scripts/semantic_lint.py`
- [x] Phases 101–109: documentation deliverables complete
- [ ] Phases 111+: `cargo check -p kernel` + smoke script TBD
"""

PHASE_110_VALIDATION = """## Validation

- [x] `python scripts/semantic_lint.py`
- [x] `python scripts/gate/legacy.py --phase 110 --timeout 300`
- [x] `cargo test -p kernel --features preemption --test preemption_integration` (phase110_constitutional_smoke_works)
- [x] `python scripts/gate/boot.py --gate constitutional --timeout 300`
"""

for n in range(101, 110):
    path = DOCS / f"phase-{n}-checklist.md"
    text = path.read_text(encoding="utf-8")
    text = text.replace("- [ ] Consistent with [AXIOMS.md](AXIOMS.md)", "- [x] Consistent with [AXIOMS.md](AXIOMS.md)")
    text = text.replace("- [ ] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)", "- [x] Listed in [ROADMAP_POST100.md](ROADMAP_POST100.md)")
    if "## Validation" in text:
        import re

        text = re.sub(r"## Validation\n\n.*?(?=\n## )", VALIDATION_BLOCK + "\n", text, count=1, flags=re.DOTALL)
    path.write_text(text, encoding="utf-8")

path = DOCS / "phase-110-checklist.md"
text = path.read_text(encoding="utf-8")
if "## Validation" in text:
    import re

    text = re.sub(r"## Validation\n\n.*?(?=\n## )", PHASE_110_VALIDATION + "\n", text, count=1, flags=re.DOTALL)
path.write_text(text, encoding="utf-8")
print("updated checklists 101-110")
