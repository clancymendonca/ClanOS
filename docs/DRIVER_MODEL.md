# Driver Model

```yaml
status: authoritative
semantics_version: 1.0.0
```

Epoch 2 planning stub. See `DECISION_LOG.md#driver_isolation_model`.

---

## Alternatives

| Model | Description |
|-------|-------------|
| Kernel TCB | Driver in kernel — smallest userspace surface |
| Process + device caps | Userspace driver with gated device caps |
| Hybrid | Minimal kernel shim + userspace protocol |

**Decision:** TBD before virtio-blk epoch 2.
