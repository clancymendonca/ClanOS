# Milestone 400 Release Scorecard — Functional OS



```yaml

status: authoritative

milestone: 400

semantics_version: 1.0.0

```



| Criterion | Target | Gate |

|-----------|--------|------|

| System gate | `SYSTEM_GATE_VERSION = 1.0.0` | `system_gate.rs` |

| Functional OS | desktop + userland + network | `scripts/gate/system.py --gate functional` |

| Desktop | GUI + mouse + WM + shell | `scripts/gate/system.py --gate desktop` |

| Native apps | `/bin/demo-hello` runs | `functional_gate` kernel smoke |

| Network | Loopback ping | `network_stack` smokes |

| Prior release | M350 regression-free | `scripts/gate/system.py --gate release` |

| Boot smoke | `AresOS-Gate: name=functional ok=true` | QEMU serial |

