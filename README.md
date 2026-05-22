# AresOS ⚔️

<p align="center">
	<img src="logo.png" alt="AresOS logo" width="420" />
</p>

**AresOS** is an experimental operating system written in **Rust**, built from the ground up to explore modern kernel architecture, low-level hardware control, and safe systems programming.

Named after Ares, the project represents **strength, control, and raw system power** — the philosophy that a developer should fully understand and command the machine they use.

AresOS is both a **learning platform and a long-term experimental system**, focused on transparency, performance, and deep system knowledge.

---

# Philosophy

AresOS follows a simple belief:

> The best way to understand a computer is to build the system that runs it.

Modern operating systems hide enormous complexity behind layers of abstraction. AresOS instead embraces that complexity and exposes how systems truly work.

The project focuses on:

* **Understanding the machine**
* **Writing software close to the hardware**
* **Designing systems intentionally rather than inheriting legacy design**

Rust provides the safety guarantees needed to build such a system without sacrificing performance.

---

# Inspiration

AresOS draws inspiration from several legendary operating system projects.

One of the strongest influences is TempleOS, created entirely by Terry A. Davis.

TempleOS demonstrated what a single determined developer could achieve by building a complete operating system from scratch. Its bold philosophy and uncompromising approach to system design helped inspire many modern hobby OS projects.

While AresOS follows a different technical path—using Rust and modern system architecture—it shares the same spirit of **deep curiosity, independence, and exploration of computing at the lowest level**.

Other inspirations include:

* Linux
* Redox OS
* Minix

---

# Goals

AresOS aims to become a small but powerful experimental operating system that demonstrates:

* modern kernel design
* memory-safe systems programming
* transparent system behavior
* efficient hardware interaction

The project also serves as a **long-term exploration of operating system engineering**.

---

# Planned Features

### Kernel Core

* Rust bare-metal kernel
* interrupt handling
* memory management
* virtual memory and paging

### Hardware Interaction

* keyboard input
* timer interrupts
* device driver framework

### System Architecture

* modular kernel design
* multitasking scheduler
* kernel logging and debugging

### Storage

* filesystem support
* disk drivers
* persistent storage

### User Environment

* terminal shell
* system utilities
* process management tools

---

# Roadmap

### Phase 1 — Boot

* freestanding Rust kernel
* bootloader integration
* basic screen output

Status: ✅ Complete (validated 2026-03-17)

Checklist: `docs/phase-1-checklist.md`

### Phase 2 — Hardware

* interrupt descriptor table
* keyboard driver
* timer interrupts

Status: ✅ Complete (validated 2026-03-17)

Checklist: `docs/phase-2-checklist.md`

### Phase 3 — Memory

* paging implementation
* frame allocator
* heap allocation

Status: ✅ Complete (validated 2026-03-17)

Checklist: `docs/phase-3-checklist.md`

### Phase 4 — Processes

* multitasking scheduler
* context switching
* task management

Status: ✅ Complete (validated 2026-03-17, cooperative async; context switching in `context-lab` mode)

Checklist: `docs/phase-4-checklist.md`

### Phase 5 — Preemptive Scheduling & Process Foundation

* preemptive scheduler mode (`preemption` feature)
* process abstraction + PID allocator
* fairness telemetry and preemption observability

Status: ✅ Complete (validated 2026-05-06)

Checklist: `docs/phase-5-checklist.md`

Scheduler deep dive: `docs/SCHEDULER.md`

### Phase 6 — User Space

* command shell
* system utilities
* basic programs

Status: ✅ Complete (validated 2026-05-06; shell + utilities + syscall/storage baseline)

Checklist: `docs/phase-6-checklist.md`

### Phase 7 — Persistent Storage

* block-device storage boundary
* simple persistent filesystem format
* shell and syscall file operations

Status: ✅ Complete (validated 2026-05-13; remount persistence + QEMU storage smoke)

Checklist: `docs/phase-7-checklist.md`

Storage deep dive: `docs/STORAGE.md`

### Phase 8 — Device & Block Driver Bring-Up

* device registry and PCI discovery skeleton
* block-device manager
* QEMU-friendly driver-backed storage path

Status: ✅ Complete (validated 2026-05-13; device/block smoke + storage-through-manager)

Checklist: `docs/phase-8-checklist.md`

Device deep dive: `docs/DEVICES.md`

### Phase 9 — Stored Program Loader

* executable manifest format
* `/bin/*` program discovery
* file-backed launch path for built-in program entries

Status: ✅ Complete (validated 2026-05-13; stored manifests + loader smoke)

Checklist: `docs/phase-9-checklist.md`

Program loader deep dive: `docs/PROGRAMS.md`

### Phase 10 — Permissions & Process Isolation Groundwork

* static users, roles, and credential model
* file owner/mode metadata with checked shell/syscall operations
* executable trust fields and process ownership policy

Status: ✅ Complete (validated 2026-05-13; permission denial + process ownership smoke)

Checklist: `docs/phase-10-checklist.md`

Security deep dive: `docs/SECURITY.md`

### Phase 11 — Executable Image & Address-Space Groundwork

* conservative ELF64 image validation
* descriptor-only address-space and virtual-region model
* image manifest discovery without unsafe binary execution

Status: ✅ Complete (validated 2026-05-13; image validation + unsupported execution smoke)

Checklist: `docs/phase-11-checklist.md`

Executable image deep dive: `docs/EXECUTABLE_IMAGES.md`

### Phase 12 — Executable Load Plans & Mapping Groundwork

* page-aligned executable load plans
* copy and zero-fill action accounting
* frame/page reservation metadata without page-table mutation

Status: ✅ Complete (validated 2026-05-13; load-plan preparation + execution-block smoke)

Checklist: `docs/phase-12-checklist.md`

Load-plan deep dive: `docs/LOAD_PLANS.md`

### Phase 13 — Frame-Backed Mapping Stubs

* deterministic mapping-stub records for prepared load plans
* frame-token, copy-byte, and zero-fill accounting
* mapped-stub process metadata without executable scheduling

Status: ✅ Complete (validated 2026-05-13; mapping-stub smoke + execution-block preservation)

Checklist: `docs/phase-13-checklist.md`

Mapping-stub deep dive: `docs/MAPPING_STUBS.md`

### Phase 14 — Frame Ownership Service

* persistent frame ownership registry
* bounded physical-frame accounting after heap initialization
* frame allocation/release counters for future executable backing

Status: ✅ Complete (validated 2026-05-13; frame ownership smoke)

Checklist: `docs/phase-14-checklist.md`

Frame ownership deep dive: `docs/FRAME_OWNERSHIP.md`

### Phase 15 — Real Backing Frames For Load Plans

* frame-backed image records for mapped executable pages
* owned-frame consumption from the Phase 14 registry
* copy and zero-fill accounting attached to backed pages

Status: ✅ Complete (validated 2026-05-13; frame-backed image smoke)

Checklist: `docs/phase-15-checklist.md`

Frame-backed image deep dive: `docs/FRAME_BACKED_IMAGES.md`

### Phase 16 — Inactive User Page Tables

* inactive user page-table descriptors for frame-backed images
* virtual-to-physical translation validation
* blocked `PageTableReady` process metadata without CR3 switching

Status: ✅ Complete (validated 2026-05-13; inactive page-table smoke)

Checklist: `docs/phase-16-checklist.md`

User page-table deep dive: `docs/USER_PAGE_TABLES.md`

### Phase 17 — User Context And Entry Frames

* GDT user code/data selectors
* initial user entry frame and stack descriptors
* blocked `UserContextReady` process metadata without Ring 3 entry

Status: ✅ Complete (validated 2026-05-13; user-context smoke)

Checklist: `docs/phase-17-checklist.md`

User context deep dive: `docs/USER_CONTEXT.md`

### Phase 18 — Controlled Ring 3 Trampoline

* controlled user-entry/trap result records
* reserved user trap vector metadata
* blocked `UserTrapped` process metadata

Status: ✅ Complete (validated 2026-05-13; controlled Ring 3 trampoline smoke)

Checklist: `docs/phase-18-checklist.md`

Ring 3 trampoline deep dive: `docs/RING3_TRAMPOLINE.md`

### Phase 19 — Syscall Entry And Return ABI

* user syscall register-frame ABI
* syscall dispatch return metadata
* blocked `UserSyscallReturned` process metadata

Status: ✅ Complete (validated 2026-05-13; syscall return smoke)

Checklist: `docs/phase-19-checklist.md`

User syscall deep dive: `docs/USER_SYSCALLS.md`

### Phase 20 — Minimal ELF Execution MVP

* guarded `/bin/hello` ELF execution path
* deterministic output and exit status for `run hello`
* blocked `UserElfExited` process metadata

Status: ✅ Complete (validated 2026-05-13; user ELF smoke)

Checklist: `docs/phase-20-checklist.md`

User ELF MVP deep dive: `docs/USER_ELF_MVP.md`

### Phase 21 — Hardware User Page Tables

* real x86_64 page tables from inactive descriptors
* descriptor vs hardware translation verification
* blocked `HwPageTableReady` process metadata

Checklist: `docs/phase-21-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 22 — Controlled CR3 Activation

* activate and restore user CR3 without execution
* translation verification under switched page tables
* blocked `Cr3Activated` process metadata

Checklist: `docs/phase-22-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 23 — Real iretq User Entry

* CPU Ring 3 entry via `iretq` to a controlled stub
* return through invalid-opcode trap during bring-up
* blocked `UserEnteredHw` process metadata

Checklist: `docs/phase-23-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 24 — Hardware User Trap Return

* IDT vector `0x80` handler for cooperative user return
* blocked `UserHwTrapped` process metadata

Checklist: `docs/phase-24-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 25 — CPU syscall / sysret Path

* `syscall`/`sysret` MSRs and entry stub
* hardware tick-probe syscall path
* blocked `UserHwSyscallReturned` process metadata

Checklist: `docs/phase-25-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 26 — Validated User Copyin

* bounded `copy_from_user` / `copy_to_user`
* copy-probe syscall round-trip

Checklist: `docs/phase-26-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 27 — Static ELF Relocations

* `R_X86_64_RELATIVE` / `R_X86_64_64` for seeded images
* relocation accounting during frame backing

Checklist: `docs/phase-27-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 28 — Hardware Hello Execution

* `run hello` through hardware Ring 3 + syscall path
* blocked `UserHwElfExited` process metadata

Checklist: `docs/phase-28-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 29 — Allowlisted ELF Programs

* allowlisted `hello` and `exit42` ELF programs
* seeded manifests and images

Checklist: `docs/phase-29-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 30 — Per-Process CR3 Switching

* save/restore distinct user CR3 values
* isolation verification across switches

Checklist: `docs/phase-30-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 31 — Scheduler CR3 Binding

* CR3 binding on process records and preemptive context switch
* `Phase31-SchedCr3` boot smoke

Checklist: `docs/phase-31-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

Scheduler deep dive: `docs/SCHEDULER.md`

### Phase 32 — User Trap Frame Persistence

* saved `UserHwFrame` across scheduler yield
* `Phase32-UserFrame` boot smoke

Checklist: `docs/phase-32-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 33 — Concurrent Allowlisted ELFs

* `hello` and `exit42` under distinct hardware page tables
* `Phase33-MultiElf` boot smoke

Checklist: `docs/phase-33-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 34 — Exit and Wait Syscalls

* `ExitProcess` / `WaitProcess` syscalls
* `Phase34-ExitWait` boot smoke

Checklist: `docs/phase-34-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 35 — Hardware Syscall Dispatch Table

* allowlisted hardware syscall IDs
* `Phase35-SyscallTable` boot smoke

Checklist: `docs/phase-35-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

User syscall deep dive: `docs/USER_SYSCALLS.md`

### Phase 36 — Storage Syscalls With Copyin

* storage probe syscalls with validated user copies
* `Phase36-StorageCopyin` boot smoke

Checklist: `docs/phase-36-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 37 — Manifest-Discovered ELF Load

* discover `elf64-image` manifests; gated execution including `tickprobe`
* `Phase37-ManifestElf` boot smoke

Checklist: `docs/phase-37-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 38 — Demand-Zero Page Growth

* user `#PF` handler and demand-zero mapping
* `Phase38-DemandZero` boot smoke

Checklist: `docs/phase-38-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

Demand paging deep dive: `docs/DEMAND_PAGING.md`

### Phase 39 — Dynamic Linking Groundwork

* `DT_NEEDED` detection for ARES seed ELFs
* `Phase39-Dynamic` boot smoke

Checklist: `docs/phase-39-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 40 — Integration Milestone

* end-to-end validation of phases 31–39
* `Phase40-Integration` boot smoke

Checklist: `docs/phase-40-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 41 — Shared Library Mapping

* map `libc_stub` dependency at `0x700000` when `DT_NEEDED` is present
* `Phase41-SharedLib` boot smoke

Checklist: `docs/phase-41-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

Shared library deep dive: `docs/SHARED_LIBRARIES.md`

### Phase 42 — Dynamic Import Relocations

* `R_X86_64_GLOB_DAT` imports against mapped shared library
* `Phase42-DynReloc` boot smoke

Checklist: `docs/phase-42-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 43 — Trust-Gated ELF Execution

* run `trust=system` manifests without name allowlist (`systrust` fixture)
* `Phase43-TrustExec` boot smoke

Checklist: `docs/phase-43-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 44 — User Path Copyin

* `ReadPathProbe` syscall with validated user paths
* `Phase44-UserPath` boot smoke

Checklist: `docs/phase-44-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 45 — File Descriptor Table

* `OpenFile` / `CloseFile` syscalls with bring-up FD table
* `Phase45-FileFd` boot smoke

Checklist: `docs/phase-45-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

File I/O deep dive: `docs/FILE_DESCRIPTORS.md`

### Phase 46 — FD Read/Write

* `ReadFd` / `WriteFd` with validated user buffers
* `Phase46-FdIO` boot smoke

Checklist: `docs/phase-46-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 47 — File-Backed Demand Paging

* demand-map file pages from storage on user `#PF`
* `Phase47-FileDemand` boot smoke

Checklist: `docs/phase-47-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 48 — W^X Mapping Policy

* reject writable+executable user page flags
* `Phase48-WxPolicy` boot smoke

Checklist: `docs/phase-48-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 49 — SMP Groundwork

* CPU count detection, AP accounting, TLB flush hooks
* `Phase49-Smp` boot smoke

Checklist: `docs/phase-49-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

SMP deep dive: `docs/SMP.md`

### Phase 50 — Integration Milestone (41–49)

* end-to-end validation of phases 41–49
* `Phase50-Integration` boot smoke

Checklist: `docs/phase-50-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Phase 51 — Per-Process FD Tables

* FD tables stored on `Process`; `current_process_id` from CR3
* `Phase51-ProcFd` boot smoke

Checklist: `docs/phase-51-checklist.md`

### Phase 52 — Dup FD and CWD-Relative Open

* `DupFd` syscall; per-process `cwd`; relative `OpenFile`
* `Phase52-FdDup` boot smoke

Checklist: `docs/phase-52-checklist.md`

### Phase 53 — mprotect and Guard Pages

* `Mprotect` syscall within W^X; stack guard probe
* `Phase53-Mprotect` boot smoke

Checklist: `docs/phase-53-checklist.md`

### Phase 54 — mmap Bring-Up

* anonymous RW at `0x600000`; read-only file mmap
* `Phase54-Mmap` boot smoke

Checklist: `docs/phase-54-checklist.md`

### Phase 55 — User Write Path

* `WritePathProbe` syscall; storage round-trip smoke
* `Phase55-WritePath` boot smoke

Checklist: `docs/phase-55-checklist.md`

### Phase 56 — Multiple Shared Libraries

* `/lib/*.elf` search; `libc_stub` + `libaux_stub` mapping
* `Phase56-MultiShlib` boot smoke

Checklist: `docs/phase-56-checklist.md`

### Phase 57 — PLT JUMP_SLOT Relocations

* `R_X86_64_JUMP_SLOT` binding; `Phase57-PltReloc` boot smoke

Checklist: `docs/phase-57-checklist.md`

### Phase 58 — Manifest Digest Trust

* `digest=sha256:` manifest field; SHA-256 verification
* `Phase58-DigestTrust` boot smoke

Checklist: `docs/phase-58-checklist.md`

### Phase 59 — Per-CPU Runqueue Skeleton

* BSP runqueue accounting on preempt; APs remain parked
* `Phase59-Runqueues` boot smoke

Checklist: `docs/phase-59-checklist.md`

### Phase 60 — Integration Milestone (51–59)

* cumulative validation of phases 51–59
* `Phase60-Integration` boot smoke

Checklist: `docs/phase-60-checklist.md`

---

# Documentation

Full index of checklists, deep-dive guides, and validation commands: [`docs/INDEX.md`](docs/INDEX.md)

---

# Project Structure

```
AresOS
├── Cargo.toml                 workspace manifest
├── docs/                      phase checklists + deep-dive guides (see INDEX.md)
├── scripts/                   QEMU smoke checks + validation_matrix.py
├── kernel/
│   ├── Cargo.toml             kernel crate manifest
│   ├── x86_64-unknown-none.json
│   ├── src/
│   │   ├── main.rs            kernel entry + phase boot smokes
│   │   ├── lib.rs             modules, init (GDT, IDT, SMP)
│   │   ├── storage.rs         simple persistent filesystem
│   │   ├── security.rs        identity + permission policy
│   │   ├── syscall.rs         syscall IDs + dispatch
│   │   ├── device.rs          device registry + PCI skeleton
│   │   ├── block.rs           block-device manager
│   │   ├── exec_image.rs      ELF64 image validation
│   │   ├── elf_reloc.rs       static + GLOB_DAT relocations
│   │   ├── shared_loader.rs   shared library mapping (phase 41)
│   │   ├── load_plan.rs       executable load-plan accounting
│   │   ├── mapping_stub.rs    mapping-stub records
│   │   ├── frame_ownership.rs frame ownership registry
│   │   ├── frame_backing.rs   frame-backed image pages
│   │   ├── user_memory.rs     inactive page-table descriptors
│   │   ├── user_paging.rs     hardware page tables, CR3, W^X
│   │   ├── demand_paging.rs   demand-zero + file-backed #PF
│   │   ├── user_context.rs    user entry-frame descriptors
│   │   ├── user_entry.rs      Ring 3 / iretq / syscall entry
│   │   ├── user_syscall.rs    syscall ABI + copy helpers
│   │   ├── user_syscall_hw.rs hardware syscall/sysret path
│   │   ├── user_copy.rs       validated user copies
│   │   ├── user_path.rs       bounded user path copyin
│   │   ├── fd_table.rs        per-process file descriptor table
│   │   ├── mmap.rs            mmap bring-up (Phase 54)
│   │   ├── image_digest.rs    SHA-256 manifest digests (Phase 58)
│   │   ├── smp.rs             CPU detect, TLB hooks, runqueues
│   │   ├── ring3_trampoline.rs controlled user-entry traps
│   │   ├── task/              scheduler, loader, keyboard shell
│   │   └── performance/       metrics + profiler
│   └── tests/
│       └── preemption_integration.rs
└── .cargo/config.toml         target + runner configuration
```

---

# Building

Install dependencies:

```
rustup component add llvm-tools-preview
cargo install bootimage
rustup component add rust-src
```

Install QEMU (example on Ubuntu/Debian):

```
sudo apt install qemu-system-x86
```

Install QEMU on Windows (winget):

```
winget install --id SoftwareFreedomConservancy.QEMU --accept-package-agreements --accept-source-agreements
```

Build the OS:

```
cargo build -p kernel
```

---

# Running

Run AresOS using QEMU:

```
cargo run -p kernel
```

Run Phase 5 preemption mode:

```
cargo run -p kernel --features preemption
```

Phase 5 integration checks:

```
cargo test -p kernel --test preemption_integration
```

Phase 5 soak check (fairness/progress):

```
./scripts/phase5-soak-check --duration 120 --min-samples 3
```

Phase 5 latency check (<100ms estimated preemption latency):

```
./scripts/phase5-latency-check --duration 120 --min-samples 5 --max-latency-ms 100
```

Phase 6 smoke check:

```
./scripts/phase6-smoke-check
```

Phase 7 persistent storage check:

```
./scripts/phase7-storage-check --timeout 180
```

Phase 8 device/block check:

```
./scripts/phase8-device-check --timeout 180
```

Phase 9 stored program loader check:

```
./scripts/phase9-loader-check --timeout 180
```

Phase 10 security policy check:

```
./scripts/phase10-security-check --timeout 180
```

Phase 11 executable image check:

```
./scripts/phase11-image-check --timeout 180
```

Phase 12 executable load-plan check:

```
./scripts/phase12-load-plan-check --timeout 180
```

Phase 13 mapping-stub check:

```
./scripts/phase13-mapping-stub-check --timeout 180
```

Phase 14 frame ownership check:

```
./scripts/phase14-frame-check --timeout 180
```

Phase 15 frame-backed image check:

```
./scripts/phase15-frame-backing-check --timeout 180
```

Phase 16 inactive page-table check:

```
./scripts/phase16-page-table-check --timeout 180
```

Phase 17 user-context check:

```
./scripts/phase17-user-context-check --timeout 180
```

Phase 18 controlled Ring 3 check:

```
./scripts/phase18-ring3-check --timeout 180
```

Phase 19 syscall return check:

```
./scripts/phase19-syscall-return-check --timeout 180
```

Phase 20 user ELF check:

```
./scripts/phase20-user-elf-check --timeout 180
```

Phase 21 hardware page-table check:

```
python scripts/phase21_hw_page_table_check.py --timeout 180
```

Phase 22 CR3 activation check:

```
python scripts/phase22_cr3_check.py --timeout 180
```

Phase 23 iretq entry check:

```
python scripts/phase23_iretq_check.py --timeout 180
```

Phase 24 user trap check:

```
python scripts/phase24_user_trap_check.py --timeout 180
```

Phase 25 hardware syscall check:

```
python scripts/phase25_syscall_hw_check.py --timeout 180
```

Phase 26 user copyin check:

```
python scripts/phase26_copyin_check.py --timeout 180
```

Phase 27 relocation check:

```
python scripts/phase27_reloc_check.py --timeout 180
```

Phase 28 hardware hello check:

```
python scripts/phase28_hw_hello_check.py --timeout 180
```

Phase 29 allowlist check:

```
python scripts/phase29_allowlist_check.py --timeout 180
```

Phase 30 CR3 switch check:

```
python scripts/phase30_cr3_switch_check.py --timeout 180
```

Phases 31–50 QEMU checks (same pattern; example):

```
python scripts/phase41_shared_lib_check.py --timeout 180
python scripts/phase50_integration_check.py --timeout 180
```

Full validation matrix (QEMU-backed; run alone on Windows, ~2+ hours):

```
python scripts/validation_matrix.py --soak-duration 30 --latency-duration 30 --boot-wait 90 --smoke-timeout 180
```

Resume from a specific check:

```
python scripts/validation_matrix.py --from-check phase41-shared-lib-check --smoke-timeout 180
```

Run tests (unit + integration under QEMU):

```
cargo test -p kernel --features preemption --test preemption_integration
```

Run Phase 4 wrapper-mode preemption soak check:

```
./scripts/phase4-soak-check
```

---

# Vision

AresOS is not intended to replace existing operating systems.

Instead, it exists to answer a question:

**What happens when you build a system entirely on your own terms?**

---

# License

Licensed under the Apache License, Version 2.0.

See [LICENSE](LICENSE) for the full text.



