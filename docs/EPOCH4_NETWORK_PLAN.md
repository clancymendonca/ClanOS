# Epoch 4 Planning — Networking

```yaml
status: epoch-scoped: 4
```

Pre-epoch planning commit:

- virtio-net driver (shared virtio framework with epoch 2 blk)
- Compat TCP/UDP + multi-fd `select` — WIRE_SCHEMA_REGISTRY + COMPAT_SUNSET
- Network broker (phase 125) becomes functional
- Network isolation placeholder (broker-filtered caps vs external-routable)

Gate: socket P99 benchmark; compat review.
