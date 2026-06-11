# Generation Counter

```yaml
status: authoritative
semantics_version: 1.0.0
```

See [`KERNEL_OBJECT_MODEL.md`](KERNEL_OBJECT_MODEL.md).

---

## Type

`u64` everywhere (kernel, audit, wire). Overflow not practical; narrowing requires charter revision.

---

## Increment triggers

Service restart, hard revoke, broker session end, endpoint teardown, object invalidation.

---

## Uniqueness

No two concurrently valid caps share `(object_id, generation)` — Kani at declared bound + proptest.

---

## Cold restart (QEMU-era)

All pre-restart caps structurally invalidated; generation/object_id reuse safe within new session.

---

## Entropy

Sequential and predictable in QEMU era unless charter requires CSPRNG. Observable generation values omitted from unprivileged caller errors.
