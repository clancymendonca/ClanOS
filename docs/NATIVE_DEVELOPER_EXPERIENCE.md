# Native Developer Experience

**UX is part of architecture.** No ambient paths is correct technically but **ecosystem-hostile** if tooling is weak — developers will retreat to compat.

Outline phase **110**; SDK work phases **161–170** ([ROADMAP_POST100.md](ROADMAP_POST100.md)).

See: [AXIOMS.md](AXIOMS.md), [ABI_SECURITY.md](ABI_SECURITY.md), [NATIVE_MODEL.md](NATIVE_MODEL.md).

---

## Problem

Developers are trained on:

```text
open("/path/to/file")
```

Native model uses **capability grants** and broker-issued handles. Without ergonomics, native development stalls in compat mode.

---

## Goals

| Goal | Approach |
|------|----------|
| Grants feel like “open document” | SDK high-level API backed by storage broker |
| Debuggable | Law + spec case ID in errors ([SEMANTIC_OBSERVABILITY.md](SEMANTIC_OBSERVABILITY.md)) |
| Shell usable | Broker-mediated names as **labels**, not kernel authority |
| IDE-friendly | Stable grant URIs / manifest paths for editor integration |

---

## Non-goals (initial SDK)

- Exposing raw `ObjectId` / generation to app developers in release builds
- Restoring ambient `/` for native “convenience”

---

## Shell / debug (draft)

| Feature | Behavior |
|---------|----------|
| `grant list` | Show active FsNode caps with manifest scope labels |
| `open@docs` | Resolve developer label via broker — not kernel path walk |
| Denied access | Message cites manifest clause + A2 |

---

## Prevent compat retreat

Track metric (future): % native launches vs compat ELF — target growth as SDK matures.

Phase 128+ mandatory `ares-native-v1` for native tier binaries.

---

## Relation to jurisdiction

Runtime SDK lives in **runtime layer** — cannot define new cross-cutting laws ([SEMANTIC_JURISDICTION.md](SEMANTIC_JURISDICTION.md)).
