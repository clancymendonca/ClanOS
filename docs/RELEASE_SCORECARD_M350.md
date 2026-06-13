# Milestone 350 Release Scorecard



```yaml

status: authoritative

milestone: 350

semantics_version: 1.0.0

```



| Criterion | Target | Gate |

|-----------|--------|------|

| System gate | `SYSTEM_GATE_VERSION = 1.0.0` | `system_gate.rs` |

| Boot gate | `BOOT_GATE_VERSION = 1.0.0` | `boot_gate.rs` |

| Gap registry | 0 open | `release_scorecard_check.py` |

| Threat nodes | 0 open | `threat_node_lifecycle_check.py` |

| Covenant CI | green | `covenant_ci.py` |

| Gate smokes | integrity → release | `scripts/gate/host.py` + QEMU |

| Never-stabilize | graduated | `never_stabilize_graduated.toml` |

| Public process | SECURITY + CONTRIBUTING | repo root docs |

| Reviewer registry | populated | `keys/reviewer-registry.toml` |

