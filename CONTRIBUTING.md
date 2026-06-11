# Contributing to AresOS

## Epoch 0 process

1. Author foundational docs per `prereq_graph.toml` DAG on staging branch
2. Cross-doc review before epoch 0 gate squash
3. Unanimous 3/3 domain sign-offs in `epoch_signoffs/epoch-0.toml`
4. GPG-signed gate commit per `SECURITY.md`

## Phase commits

- One commit per implementation phase: `feat(phase-NNN): ...`
- Phase owner only commits their phase (`phase_checklist_schema.toml`)

## Cross-references

Staging may use `[CROSS-REF: doc §section — TBD]`; must be resolved at gate.

## Milestone 150 deliverables

- **Capability transfer walkthrough:** animated or interactive walkthrough of a capability transfer sequence showing state machine transitions in `KERNEL_OBJECT_MODEL.md` and `CAP_TRANSFER_PROTOCOL.md` — onboarding artifact for contributors and external auditors.

## Ergonomics feedback

Each epoch retrospective (`docs/epoch_retrospectives/TEMPLATE.md`) records process vs implementation time — feeds charter amendments.
