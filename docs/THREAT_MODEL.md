# AresOS Threat Model

```yaml
status: authoritative
semantics_version: 1.0.0
```

Scope: phases 121–150 QEMU-first development. Structured nodes in [`THREAT_NODES.toml`](THREAT_NODES.toml).

See: [`FAULT_ESCALATION.md`](FAULT_ESCALATION.md), [`KERNEL_OBJECT_MODEL.md`](KERNEL_OBJECT_MODEL.md), [`SECURITY.md`](../SECURITY.md).

---

## Attacker goals

Every threat analysis records which **goals** a control addresses. Coverage is reported per goal in `STATUS.md` (not only open/closed node counts).

| Goal id | Description | Example attacks |
|---------|-------------|-----------------|
| `privilege_escalation` | Gain authority beyond granted caps | Cap amplification, broker bypass, shim elevation |
| `information_disclosure` | Leak data across trust boundaries | Audit exfiltration, generation oracle, covert channels |
| `denial_of_service` | Degrade availability without escalation | Cap exhaustion, IPC liveness deadlock, stack overflow |
| `integrity_violation` | Corrupt security state or evidence | Audit tamper, image substitution, wire schema confusion |

Each threat node in `THREAT_NODES.toml` includes `goals = [...]`. Each attacker class maps to one or more goals below.

---

## Attacker classes

| Class | Capability | Goals |
|-------|------------|-------|
| **Unprivileged user process** | Ring 3, compat syscalls, limited caps | `privilege_escalation`, `denial_of_service` |
| **Compromised userland service** | Native caps from loader manifest | `privilege_escalation`, `information_disclosure`, `denial_of_service`, `integrity_violation` |
| **Compromised compat shim** | Compat-internal bridge access | `privilege_escalation`, `integrity_violation` |
| **Malicious IPC peer** | Valid endpoint, hostile message patterns | `denial_of_service`, `privilege_escalation` |
| **Compromised CI runner** | Build pipeline write access | `integrity_violation` |
| **Compromised contributor** | Merge access with review bypass attempt | `integrity_violation`, `privilege_escalation` |
| **Physical attacker** | Hardware access | Out of scope pre-150; `reopen_trigger` in architecture_state |
| **Side-channel observer** | Timing/power observation | Out of scope pre-150 with deferred node |
| **Supply chain attacker** | Malicious crate or toolchain | `integrity_violation` |

---

## Attack surfaces

| Surface | Owner doc |
|---------|-----------|
| Cap table | `KERNEL_OBJECT_MODEL.md` |
| Audit subsystem | `AUDIT_SUBSYSTEM.md` |
| IPC / endpoints | `CAP_TRANSFER_PROTOCOL.md`, `ABI_IPC.md` |
| Compat shim | `COMPAT_ISOLATION.md` |
| Build pipeline | `BUILD_INTEGRITY.md`, `SUPPLY_CHAIN_POLICY.md` |
| Kernel heap / stack | `MEMORY_SAFETY_BOUNDARY.md`, `UNSAFE_AUDIT.md` |

`project_health.py` emits a **threat coverage matrix** (goals × surfaces) each epoch gate.

---

## Insider threat

Proofs verify code as written, not author intent. Mitigation: second reviewer on TCB (`UNSAFE_AUDIT.md`), GPG gate commits, split domain review. Residual risk documented; not eliminated.

---

## Debug introspection

Ad-hoc debug bypassing cap model: **out of scope** pre-150 with open threat node `T-debug-bypass` until policy defined.

---

## Deferred re-evaluation

Threat nodes with `status = deferred` reopen when `architecture_state.toml` flags match. CI blocks epoch gate if flag is true without re-eval commit.

---

## Service identity

Stable service identity for audit correlation across restart: **deferred** post-150 or loader-registered name stub — see `KERNEL_OBJECT_MODEL.md`.
