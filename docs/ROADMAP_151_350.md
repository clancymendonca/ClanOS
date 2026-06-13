# Post-150 Roadmap (Scopes 151–350)

```yaml
status: authoritative
semantics_version: 1.0.0
```

Continues from [ROADMAP_POST100.md](ROADMAP_POST100.md) milestone 150. Living plan: [`.cursor/plans/clanos_build_151_350.plan.md`](../.cursor/plans/clanos_build_151_350.plan.md).

**Pace:** 1–3 scopes/month. Milestone 350 targets 1.0 release gate (~5–8 years post-150).

---

## Epoch map

| Epoch | Scopes | Milestone | Theme |
|-------|--------|-----------|-------|
| 7 | 151–175 | — | Stub graduation: loom, OOM, audit/build, Kani tier B |
| 8 | 176–200 | **200** | SCHEDULING_UNIFIED + meta-semantics + semantic lint CI |
| 9 | 201–225 | — | Native SDK / UX; clan-rt production path |
| 10 | 226–250 | **250** | Language adapters; POSIX depth; QEMU→hardware |
| 11 | 251–275 | — | Userspace drivers; IOMMU/DMA; GPU isolation |
| 12 | 276–300 | **300** | Observability; federation; distributed endpoints |
| 13 | 301–325 | — | Checkpoint security; Tier D; Verus selective |
| 14 | 326–350 | **350** | 1.0 API freeze; compat sunset; public release |

---

## Scopes 151–175 (Epoch 7)

| Scope | Title | Layer |
|------:|-------|-------|
| 151 | Loom ENDPOINT_QUEUES harness | kernel |
| 152 | Loom SESSION_QUEUES harness | kernel |
| 153 | SMP AP bring-up gate | kernel |
| 154 | SCHEDULING_UNIFIED draft | kernel |
| 155 | S-01 executable spec case | kernel |
| 156 | OOM suspend frozen-in-memory | kernel |
| 157 | OOM shed/ack wire format | kernel |
| 158 | MEM_BUDGET full enforcement | kernel |
| 159 | Cap quota on all mint paths | platform |
| 160 | Epoch 7 OOM integration | kernel |
| 161 | Audit tamper policy | kernel |
| 162 | Audit shadow counter | kernel |
| 163 | Dual-build hash CI | platform |
| 164 | Build integrity production path | platform |
| 165 | Epoch 7 audit/build gate | governance |
| 166 | Mandatory Kani CI | governance |
| 167 | Transfer TOCTOU Kani harness | kernel |
| 168 | Fuzz corpus graduation | governance |
| 169 | Proof cache key CI | governance |
| 170 | Epoch 7 evidence gate | governance |
| 171 | Compat review epoch 7 | compat |
| 172 | Benchmark re-baseline | governance |
| 173 | Health dashboard delta | governance |
| 174 | Epoch 7 integration smoke | governance |
| 175 | Epoch 7 signoff | governance |

---

## Scopes 176–200 (Epoch 8 → Milestone 200)

| Scope | Title |
|------:|-------|
| 176–180 | Service-centric scheduler implementation (S-*) |
| 181–185 | Meta-semantics M-* precedence table |
| 186–190 | Semantic lint CI for clan-semantics-v* |
| 191–195 | Full health dashboard |
| 196–199 | Four-layer boundary review II |
| 200 | **Milestone 200** integration gate |

---

## Scopes 201–250 (Epochs 9–10 → Milestone 250)

| Scope | Title |
|------:|-------|
| 201–210 | Native SDK / manifest tooling / UX |
| 211–220 | Language runtime adapters (Rust, C) |
| 221–230 | POSIX compat depth + corpus expansion |
| 231–240 | Real hardware path + architecture_state flags |
| 241–249 | QEMU→hardware transition procedure |
| 250 | **Milestone 250** hardware + SDK gate |

---

## Scopes 251–300 (Epochs 11–12 → Milestone 300)

| Scope | Title |
|------:|-------|
| 251–265 | DRIVER_MODEL implementation; userspace drivers |
| 266–275 | Semantic observability tooling |
| 276–290 | Federation + distributed endpoint protocol |
| 291–299 | Checkpoint reopen_trigger design |
| 300 | **Milestone 300** federation gate |

---

## Scopes 301–350 (Epochs 13–14 → Milestone 350)

| Scope | Title |
|------:|-------|
| 301–310 | Checkpoint/restore security domain |
| 311–320 | FORMAL_MODEL.md + Tier D / Verus |
| 321–330 | Never-stabilize graduation to 1.0 |
| 331–340 | Public SECURITY/CONTRIBUTING; GPG gates |
| 341–349 | Release scorecard + compat sunset target |
| 350 | **Milestone 350** release 1.0 gate |

---

## Boot smokes (post-150)

| Line | Epoch |
|------|-------|
| `ClanOS-Gate: name=integrity ok=true` | 7 |
| `ClanOS-Gate: name=scheduling ok=true` | 8 |
| `ClanOS-Gate: name=hardware ok=true` | 10 |
| `ClanOS-Gate: name=federation ok=true` | 12 |
| `ClanOS-Gate: name=release ok=true` | 14 |

Scripts: `python scripts/gate/system.py --gate integrity --timeout 180`, `python scripts/gate/system.py --gate scheduling --timeout 180`, `python scripts/gate/system.py --gate hardware --timeout 180`, `python scripts/gate/system.py --gate federation --timeout 180`, `python scripts/gate/system.py --gate release --timeout 180`.
