# Post-400 Gate Design Backlog

```yaml
status: authoritative
phase: 401-500-design
priority_adr: docs/architecture/ADR/ADR-0002-signed-elf-production-gate.md
audit_baseline: docs/GATE_AUDIT_401_500.md
```

Design specs for gates that **do not yet exist** (backend design), distinct from v2.1.0 honesty wiring.

**Build order (principal engineer discretion, 2026-06-20):** Signed ELF → CI gate stub → External network → Production SMP depth. Re-order via ADR if threat model or roadmap shifts.

---

## 1. Signed ELF (priority — ADR-0002)

| Field | Spec |
|-------|------|
| Serial line | `ClanOS-Gate: name=production ok=true` (signed leaf must fail if trust check fails) |
| Replace | `build_integrity::verify_signed_user_elf_corpus()` self-referential digest |
| Must prove | Pinned gate corpus in `config/signed_elf_test_corpus/` verified against **epoch-450 trust anchor** (`config/trust_anchor_epoch450.toml` pubkey embedded in kernel); reject tampered corpus |
| Out of scope | Loader / `/bin/*` signature path — see ADR-0002 "Two trust mechanisms" |
| Do not infer | `production ok=true` ≠ all userland signed — pinned corpus only |
| Host check | `scripts/gate/signed_elf.py` + `scripts/gate/test_signed_elf.py` (negative fixtures in `scripts/gate/fixtures/signed_elf/`) |
| Kernel smoke | `smoke_signed_user_elf()` calls `verify_signed_user_elf_corpus()` with pinned test corpus only |
| Version bump | `VALIDATION_GATE_VERSION` minor when serial semantics change |
| Threat | Update `docs/THREAT_NODES.toml` for forged userland binary |

---

## 2. CI gate (`validation_matrix_smoke`)

| Field | Spec |
|-------|------|
| Serial line | `ClanOS-Gate: name=ci ok=true` |
| Replace | `validation_matrix_smoke()` unconditional `true` |
| Option A | Host-only: remove kernel stub; CI runs matrix separately (document in VALIDATION_GATES) |
| Option B | Kernel emits `ci` only after mailbox/counter set by host orchestrator (heavy) |
| Recommendation | Option A + gate_honesty Part A ban on `validation_matrix_smoke` stub via ADR |

---

## 3. External network

| Field | Spec |
|-------|------|
| Serial line | `ClanOS-Gate: name=network ok=true` |
| Replace | `simulate_external_route()` loopback `external-probe` |
| Must prove | TX/RX on non-loopback path (second QEMU NIC, test server, or virtio peer) |
| Flag | Set `has_external_network = true` only after this gate passes |
| Host check | Extend `scripts/gate/network.py` with external harness contract |

---

## 4. Production SMP depth

| Field | Spec |
|-------|------|
| Serial line | `ClanOS-Gate: name=production ok=true` (AP leaf) |
| Replace | `smoke_ap_scheduler()` tick/enqueue counter only |
| Must prove | Multi-AP runnable progress under load; optional link to `preemption/soak.py` thresholds |
| Host check | AP fairness sample count ≥ N in serial telemetry |

---

## Implementation epoch rules

1. One logical change per commit; ADR before tradeoffs.
2. Classify in `GATE_AUDIT_401_500.md` after each gate lands.
3. No `architecture_state.toml` flag flip without matching gate proof.
