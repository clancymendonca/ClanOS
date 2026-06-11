# Kani Scope

```yaml
status: authoritative
semantics_version: 1.0.0
```

Bounded model checking policy. See [`PROOF_COVERAGE.md`](PROOF_COVERAGE.md).

---

## Policy

- Bounded ≠ complete — document intractability per function
- **Vacuity assertions** on critical harnesses — path must be reached
- Harness bound changes require `bound_justification` — path depth covered + known misses

---

## Registry

Machine-readable harness registry: function, harness file, bound, `kani_version`, last-verified commit. CI checks registry on epoch gate.

---

## Critical harnesses (epoch 0)

- Cap transfer TOCTOU (`CAP_TRANSFER_PROTOCOL.md`)
- R-cascade-revoke at declared depth
- R-destroy-notify delivery
- Rights algebra composition laws
