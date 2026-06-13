#!/usr/bin/env python3
import re
from pathlib import Path

path = Path(__file__).resolve().parents[2] / "README.md"
text = path.read_text(encoding="utf-8")
pat = re.compile(
    r"python scripts/phase(\d+)_[a-z0-9_]+(?:_check|_milestone_check)\.py(?:\s+--timeout\s+(\d+))?"
)


def repl(m: re.Match[str]) -> str:
    timeout = m.group(2) or "180"
    return f"python scripts/gate/legacy.py --phase {m.group(1)} --timeout {timeout}"


new, n = pat.subn(repl, text)
path.write_text(new, encoding="utf-8")
print(f"README: {n} replacements")
