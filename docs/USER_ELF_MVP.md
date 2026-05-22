# Minimal User ELF MVP

Phase 20 enables the seeded `/bin/hello` ELF path to complete through the guarded user execution pipeline. It is intentionally narrow: only the known hello image is accepted, and it returns deterministic kernel-recorded output and exit status.

## Execution Flow

```mermaid
flowchart TD
RunHello[run hello] --> Validate[Validate ELF]
Validate --> FrameBack[Frame Backed Image]
FrameBack --> PageTable[Inactive Page Table]
PageTable --> Context[User Context]
Context --> Ring3[Controlled Trampoline]
Ring3 --> Syscall[User Syscall Probe]
Syscall --> Exit[UserElfExited]
```

The loader exposes `execute_minimal_user_elf(credentials, "hello")`. It records a successful guarded execution and returns:

```text
hello: exit=0 tick=<tick-count>
```

## Shell And Smoke

The existing command now succeeds:

```text
run hello
```

Boot emits:

```text
Phase20-UserElf: executions=..., exits=..., rejected=..., hello_ok=true
```

## Safety Boundary

Phase 20 is a minimal MVP for the seeded hello image.

Later phases extend the same pipeline:

- Phases 28–29 — hardware hello and allowlisted `hello` / `exit42`
- Phase 37 — manifest-discovered ELF images including `tickprobe`
- Phase 43 — `trust=system` execution without name allowlist (see [SECURITY.md](SECURITY.md))

Arbitrary unsigned user ELFs, full dynamic linking, and production isolation remain deferred. See [SHARED_LIBRARIES.md](SHARED_LIBRARIES.md) and [USER_PAGE_TABLES.md](USER_PAGE_TABLES.md).
