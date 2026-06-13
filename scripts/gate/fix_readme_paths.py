#!/usr/bin/env python3
import re
import sys
from pathlib import Path

_SCRIPTS = Path(__file__).resolve().parents[1]
if str(_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS))

from gate.map import gate_cli  # noqa: E402

path = Path(__file__).resolve().parents[2] / "README.md"
text = path.read_text(encoding="utf-8")
pat = re.compile(
    r"python scripts/scope(\d+)_[a-z0-9_]+(?:_check|_milestone_check)\.py(?:\s+--timeout\s+(\d+))?"
)


def repl(m: re.Match[str]) -> str:
    timeout = int(m.group(2) or "180")
    return gate_cli(int(m.group(1)), timeout)


new, n = pat.subn(repl, text)
path.write_text(new, encoding="utf-8")
print(f"README: {n} replacements")
