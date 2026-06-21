# Clan OS Gap Registry Audit

```yaml
status: authoritative
registry: gap_registry.toml
audit_script: scripts/gate/gap_registry_audit.py
baseline_overclaimed: 204
baseline_milestone_150_stub: 202
```

## Headline finding

**204/350 (58%) `addressed` gaps are Overclaimed** — a materially worse ratio than the gate audit (where most serial lines had some real code underneath). Only **11/350 (3%)** meet **Implemented** evidence.

**202/350 (58%) of the entire registry** — not gradual drift — are **`implementing_doc = milestone-150-stub`** from **one** run of [`close_remaining_plan_gaps.py`](../scripts/close_remaining_plan_gaps.py). The headline "0 open / 350 addressed" was, for roughly two-thirds of rows, the output of a **single automated bulk-close**, not 350 independent assessments. Registry `status = addressed` was less trustworthy than gate serial output implied.

## What CI OK means

| Output | Meaning |
|--------|---------|
| `gap_registry_audit: OK` | No **new** overclaimed debt vs baseline (204); existing debt remains |
| `--strict` pass | Zero overclaimed rows (future zero-debt target) |
| `addressed` in STATUS | Lifecycle status only — see class histogram below |

A green `gap_registry_audit: OK` does **not** mean all 350 gaps are substantiated.

## Classification legend

| Class | Meaning |
|-------|---------|
| **DocOnly** | `implementing_doc` or `fix` doc path exists on disk; no verified code artifact |
| **Partial** | Code/script path exists; doc missing or gate not verified |
| **Implemented** | Doc + code paths exist (or `mark_delivered_gaps.py` DELIVERED table match) |
| **Overclaimed** | `status = addressed` but no resolvable doc or code path (e.g. `milestone-150-stub`) |
| **Superseded** | Moot by ADR/decision (`superseded_by_commit` required) |

**Important:** registry `status = addressed` ≠ **Implemented**. Most rows are DocOnly or Overclaimed relative to full delivery.

## Histogram (2026-06-20 baseline)

| Class | Count | Share |
|-------|------:|------:|
| DocOnly | 95 | 27% |
| Partial | 40 | 11% |
| Implemented | 11 | 3% |
| Overclaimed | 204 | 58% |
| **Total** | **350** | 100% |

All 350 rows have `status = addressed`, 0 `open`. Nearly all `addressing_commit = null`.

## Exemplars

| ID | Class | `implementing_doc` | Notes |
|----|-------|-------------------|-------|
| 1 | DocOnly | `docs/THREAT_MODEL.md` | Doc exists; no code smoke |
| 3 | Implemented | `docs/BUILD_INTEGRITY.md` | DELIVERED + `kernel/src/build_integrity.rs` |
| 5 | Partial | `scripts/validation_matrix.py` | Script exists; doc not in `fix` |
| 10 | Overclaimed | `milestone-150-stub` | Placeholder; no on-disk artifact |
| 12 | Implemented | `userland/src/lib.rs` | Doc + code paths exist |

## milestone-150-stub provenance

Single bulk-close event — not gradual drift:

| Fact | Detail |
|------|--------|
| Source | [`scripts/close_remaining_plan_gaps.py`](../scripts/close_remaining_plan_gaps.py) fallback (lines 85–89 before fix) |
| Mechanism | When doc-matching failed, set `status = addressed`, `implementing_doc = "milestone-150-stub"` |
| Row count | **202/350** rows (`baseline_milestone_150_stub: 202`) |
| Share of Overclaimed | ~99% of 204 (remaining 2 = other missing paths / placeholders) |
| Still active? | **Fixed** — default leaves gaps `open`; `--allow-plan-stub` required to recreate stubs |
| Not the source | [`graduate_epoch_gaps.py`](../scripts/graduate_epoch_gaps.py) does not emit this placeholder |

Regenerate histogram:

```bash
python scripts/gate/gap_registry_audit.py --summary
python scripts/gate/gap_registry_audit.py
python scripts/gate/test_gap_registry_audit.py
```

## CI enforcement

| Check | Behavior |
|-------|----------|
| `gap_registry_audit.py` | Fails if overclaimed count **exceeds** baseline (204) |
| `gap_registry_audit.py` | Fails if `milestone-150-stub` rows **exceed** baseline (202) |
| `--strict` | Fails on any overclaimed row (fixtures / future zero-debt target) |
| `test_gap_registry_audit.py` | Fixture overclaimed must fail; production must pass baseline |
| `test_close_remaining_plan_gaps.py` | Default run leaves open gaps open; stub path requires `--allow-plan-stub` |

Lowering baselines requires explicit `gap_registry.toml` review — update constants in [`gap_registry_audit.py`](../scripts/gate/gap_registry_audit.py) and this doc together.

## Remediation backlog

1. Reclassify or reopen Overclaimed IDs (especially `milestone-150-stub` rows) — separate PR batches.
2. Attach `addressing_commit` when status changes.
3. Do not bulk-mark addressed from doc-exists alone without code path review.

## Cross-refs

- Gate substance audit: [`GATE_AUDIT.md`](GATE_AUDIT.md)
- Post-400 gate audit: [`GATE_AUDIT_401_500.md`](GATE_AUDIT_401_500.md)
- Scorecard: [`RELEASE_SCORECARD.md`](RELEASE_SCORECARD.md)
