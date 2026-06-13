# Epoch 10 Graduation (M250 path)

- Native SDK: `sdk_smoke_check.py` (userland + ABI_CLAN_RT)
- Hardware transition: `hardware_smoke_check.py` (architecture_state + ARCHITECTURE_TARGETS)
- Boot attestation gap deferred with documented trigger in ARCHITECTURE_TARGETS
- `AUDIT_REQUIRED=1` enables mandatory cargo-audit in CI
