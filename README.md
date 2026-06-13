# Clan OS ⚔️

<p align="center">
	<img src="logo.png" alt="Clan OS logo" width="420" />
</p>

**Clan OS** is an experimental operating system written in **Rust**, built from the ground up to explore modern kernel architecture, low-level hardware control, and safe systems programming.

Named for the idea of a **shared craft** — building a system together with intention — the project represents **strength, control, and raw system power**: a developer should fully understand and command the machine they use.

Clan OS is both a **learning platform and a long-term experimental system**, focused on transparency, performance, and deep system knowledge.

---

# Philosophy

Clan OS follows a simple belief:

> The best way to understand a computer is to build the system that runs it.

Modern operating systems hide enormous complexity behind layers of abstraction. Clan OS instead embraces that complexity and exposes how systems truly work.

The project focuses on:

* **Understanding the machine**
* **Writing software close to the hardware**
* **Designing systems intentionally rather than inheriting legacy design**

Rust provides the safety guarantees needed to build such a system without sacrificing performance.

---

# Inspiration

Clan OS draws inspiration from several legendary operating system projects.

One of the strongest influences is TempleOS, created entirely by Terry A. Davis.

TempleOS demonstrated what a single determined developer could achieve by building a complete operating system from scratch. Its bold philosophy and uncompromising approach to system design helped inspire many modern hobby OS projects.

While Clan OS follows a different technical path—using Rust and modern system architecture—it shares the same spirit of **deep curiosity, independence, and exploration of computing at the lowest level**.

Other inspirations include:

* Linux
* Redox OS
* Minix

---

# Goals

Clan OS aims to become a small but powerful experimental operating system that demonstrates:

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

# Validation

Runtime boot validation uses unified **boot** and **system** gates — not per-scope `numbered boot serial` serial lines.

| Gate | Module | Final serial line |
|------|--------|-------------------|
| Boot | `kernel/src/boot_gate.rs` | `ClanOS-BootGate: ok=true` |
| System | `kernel/src/system_gate.rs` | `ClanOS-SystemGate: ok=true` |

Full reference: [`docs/VALIDATION_GATES.md`](docs/VALIDATION_GATES.md)

```bash
python scripts/gate/boot.py --gate boot --timeout 360
python scripts/gate/system.py --gate system --timeout 360
```

---

# Roadmap (historical scope)

Per-scope entries below document **completed implementation scope**. CI and QEMU smokes use the gates above.

### Scope 1 — Boot

* freestanding Rust kernel
* bootloader integration
* basic screen output

Status: ✅ Complete (validated 2026-03-17)

Checklist: `docs/scope-1-checklist.md`

### Scope 2 — Hardware

* interrupt descriptor table
* keyboard driver
* timer interrupts

Status: ✅ Complete (validated 2026-03-17)

Checklist: `docs/scope-2-checklist.md`

### Scope 3 — Memory

* paging implementation
* frame allocator
* heap allocation

Status: ✅ Complete (validated 2026-03-17)

Checklist: `docs/scope-3-checklist.md`

### Scope 4 — Processes

* multitasking scheduler
* context switching
* task management

Status: ✅ Complete (validated 2026-03-17, cooperative async; context switching in `context-lab` mode)

Checklist: `docs/scope-4-checklist.md`

### Scope 5 — Preemptive Scheduling & Process Foundation

* preemptive scheduler mode (`preemption` feature)
* process abstraction + PID allocator
* fairness telemetry and preemption observability

Status: ✅ Complete (validated 2026-05-06)

Checklist: `docs/scope-5-checklist.md`

Scheduler deep dive: `docs/SCHEDULER.md`

### Scope 6 — User Space

* command shell
* system utilities
* basic programs

Status: ✅ Complete (validated 2026-05-06; shell + utilities + syscall/storage baseline)

Checklist: `docs/scope-6-checklist.md`

### Scope 7 — Persistent Storage

* block-device storage boundary
* simple persistent filesystem format
* shell and syscall file operations

Status: ✅ Complete (validated 2026-05-13; remount persistence + QEMU storage smoke)

Checklist: `docs/scope-7-checklist.md`

Storage deep dive: `docs/STORAGE.md`

### Scope 8 — Device & Block Driver Bring-Up

* device registry and PCI discovery skeleton
* block-device manager
* QEMU-friendly driver-backed storage path

Status: ✅ Complete (validated 2026-05-13; device/block smoke + storage-through-manager)

Checklist: `docs/scope-8-checklist.md`

Device deep dive: `docs/DEVICES.md`

### Scope 9 — Stored Program Loader

* executable manifest format
* `/bin/*` program discovery
* file-backed launch path for built-in program entries

Status: ✅ Complete (validated 2026-05-13; stored manifests + loader smoke)

Checklist: `docs/scope-9-checklist.md`

Program loader deep dive: `docs/PROGRAMS.md`

### Scope 10 — Permissions & Process Isolation Groundwork

* static users, roles, and credential model
* file owner/mode metadata with checked shell/syscall operations
* executable trust fields and process ownership policy

Status: ✅ Complete (validated 2026-05-13; permission denial + process ownership smoke)

Checklist: `docs/scope-10-checklist.md`

Security deep dive: `docs/SECURITY.md`

### Scope 11 — Executable Image & Address-Space Groundwork

* conservative ELF64 image validation
* descriptor-only address-space and virtual-region model
* image manifest discovery without unsafe binary execution

Status: ✅ Complete (validated 2026-05-13; image validation + unsupported execution smoke)

Checklist: `docs/scope-11-checklist.md`

Executable image deep dive: `docs/EXECUTABLE_IMAGES.md`

### Scope 12 — Executable Load Plans & Mapping Groundwork

* page-aligned executable load plans
* copy and zero-fill action accounting
* frame/page reservation metadata without page-table mutation

Status: ✅ Complete (validated 2026-05-13; load-plan preparation + execution-block smoke)

Checklist: `docs/scope-12-checklist.md`

Load-plan deep dive: `docs/LOAD_PLANS.md`

### Scope 13 — Frame-Backed Mapping Stubs

* deterministic mapping-stub records for prepared load plans
* frame-token, copy-byte, and zero-fill accounting
* mapped-stub process metadata without executable scheduling

Status: ✅ Complete (validated 2026-05-13; mapping-stub smoke + execution-block preservation)

Checklist: `docs/scope-13-checklist.md`

Mapping-stub deep dive: `docs/MAPPING_STUBS.md`

### Scope 14 — Frame Ownership Service

* persistent frame ownership registry
* bounded physical-frame accounting after heap initialization
* frame allocation/release counters for future executable backing

Status: ✅ Complete (validated 2026-05-13; frame ownership smoke)

Checklist: `docs/scope-14-checklist.md`

Frame ownership deep dive: `docs/FRAME_OWNERSHIP.md`

### Scope 15 — Real Backing Frames For Load Plans

* frame-backed image records for mapped executable pages
* owned-frame consumption from the Scope 14 registry
* copy and zero-fill accounting attached to backed pages

Status: ✅ Complete (validated 2026-05-13; frame-backed image smoke)

Checklist: `docs/scope-15-checklist.md`

Frame-backed image deep dive: `docs/FRAME_BACKED_IMAGES.md`

### Scope 16 — Inactive User Page Tables

* inactive user page-table descriptors for frame-backed images
* virtual-to-physical translation validation
* blocked `PageTableReady` process metadata without CR3 switching

Status: ✅ Complete (validated 2026-05-13; inactive page-table smoke)

Checklist: `docs/scope-16-checklist.md`

User page-table deep dive: `docs/USER_PAGE_TABLES.md`

### Scope 17 — User Context And Entry Frames

* GDT user code/data selectors
* initial user entry frame and stack descriptors
* blocked `UserContextReady` process metadata without Ring 3 entry

Status: ✅ Complete (validated 2026-05-13; user-context smoke)

Checklist: `docs/scope-17-checklist.md`

User context deep dive: `docs/USER_CONTEXT.md`

### Scope 18 — Controlled Ring 3 Trampoline

* controlled user-entry/trap result records
* reserved user trap vector metadata
* blocked `UserTrapped` process metadata

Status: ✅ Complete (validated 2026-05-13; controlled Ring 3 trampoline smoke)

Checklist: `docs/scope-18-checklist.md`

Ring 3 trampoline deep dive: `docs/RING3_TRAMPOLINE.md`

### Scope 19 — Syscall Entry And Return ABI

* user syscall register-frame ABI
* syscall dispatch return metadata
* blocked `UserSyscallReturned` process metadata

Status: ✅ Complete (validated 2026-05-13; syscall return smoke)

Checklist: `docs/scope-19-checklist.md`

User syscall deep dive: `docs/USER_SYSCALLS.md`

### Scope 20 — Minimal ELF Execution MVP

* guarded `/bin/hello` ELF execution path
* deterministic output and exit status for `run hello`
* blocked `UserElfExited` process metadata

Status: ✅ Complete (validated 2026-05-13; user ELF smoke)

Checklist: `docs/scope-20-checklist.md`

User ELF MVP deep dive: `docs/USER_ELF_MVP.md`

### Scope 21 — Hardware User Page Tables

* real x86_64 page tables from inactive descriptors
* descriptor vs hardware translation verification
* blocked `HwPageTableReady` process metadata

Checklist: `docs/scope-21-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 22 — Controlled CR3 Activation

* activate and restore user CR3 without execution
* translation verification under switched page tables
* blocked `Cr3Activated` process metadata

Checklist: `docs/scope-22-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 23 — Real iretq User Entry

* CPU Ring 3 entry via `iretq` to a controlled stub
* return through invalid-opcode trap during bring-up
* blocked `UserEnteredHw` process metadata

Checklist: `docs/scope-23-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 24 — Hardware User Trap Return

* IDT vector `0x80` handler for cooperative user return
* blocked `UserHwTrapped` process metadata

Checklist: `docs/scope-24-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 25 — CPU syscall / sysret Path

* `syscall`/`sysret` MSRs and entry stub
* hardware tick-probe syscall path
* blocked `UserHwSyscallReturned` process metadata

Checklist: `docs/scope-25-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 26 — Validated User Copyin

* bounded `copy_from_user` / `copy_to_user`
* copy-probe syscall round-trip

Checklist: `docs/scope-26-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 27 — Static ELF Relocations

* `R_X86_64_RELATIVE` / `R_X86_64_64` for seeded images
* relocation accounting during frame backing

Checklist: `docs/scope-27-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 28 — Hardware Hello Execution

* `run hello` through hardware Ring 3 + syscall path
* blocked `UserHwElfExited` process metadata

Checklist: `docs/scope-28-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 29 — Allowlisted ELF Programs

* allowlisted `hello` and `exit42` ELF programs
* seeded manifests and images

Checklist: `docs/scope-29-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 30 — Per-Process CR3 Switching

* save/restore distinct user CR3 values
* isolation verification across switches

Checklist: `docs/scope-30-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 31 — Scheduler CR3 Binding

* CR3 binding on process records and preemptive context switch
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-31-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

Scheduler deep dive: `docs/SCHEDULER.md`

### Scope 32 — User Trap Frame Persistence

* saved `UserHwFrame` across scheduler yield
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-32-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 33 — Concurrent Allowlisted ELFs

* `hello` and `exit42` under distinct hardware page tables
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-33-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 34 — Exit and Wait Syscalls

* `ExitProcess` / `WaitProcess` syscalls
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-34-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 35 — Hardware Syscall Dispatch Table

* allowlisted hardware syscall IDs
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-35-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

User syscall deep dive: `docs/USER_SYSCALLS.md`

### Scope 36 — Storage Syscalls With Copyin

* storage probe syscalls with validated user copies
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-36-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 37 — Manifest-Discovered ELF Load

* discover `elf64-image` manifests; gated execution including `tickprobe`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-37-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 38 — Demand-Zero Page Growth

* user `#PF` handler and demand-zero mapping
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-38-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

Demand paging deep dive: `docs/DEMAND_PAGING.md`

### Scope 39 — Dynamic Linking Groundwork

* `DT_NEEDED` detection for CLAN seed ELFs
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-39-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 40 — Integration Milestone

* end-to-end validation of scopes 31–39
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-40-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 41 — Shared Library Mapping

* map `libc_stub` dependency at `0x700000` when `DT_NEEDED` is present
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-41-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

Shared library deep dive: `docs/SHARED_LIBRARIES.md`

### Scope 42 — Dynamic Import Relocations

* `R_X86_64_GLOB_DAT` imports against mapped shared library
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-42-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 43 — Trust-Gated ELF Execution

* run `trust=system` manifests without name allowlist (`systrust` fixture)
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-43-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 44 — User Path Copyin

* `ReadPathProbe` syscall with validated user paths
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-44-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 45 — File Descriptor Table

* `OpenFile` / `CloseFile` syscalls with bring-up FD table
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-45-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

File I/O deep dive: `docs/FILE_DESCRIPTORS.md`

### Scope 46 — FD Read/Write

* `ReadFd` / `WriteFd` with validated user buffers
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-46-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 47 — File-Backed Demand Paging

* demand-map file pages from storage on user `#PF`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-47-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 48 — W^X Mapping Policy

* reject writable+executable user page flags
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-48-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 49 — SMP Groundwork

* CPU count detection, AP accounting, TLB flush hooks
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-49-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

SMP deep dive: `docs/SMP.md`

### Scope 50 — Integration Milestone (41–49)

* end-to-end validation of scopes 41–49
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-50-checklist.md`

Status: ✅ Complete (validated 2026-05-22)

### Scope 51 — Per-Process FD Tables

* FD tables stored on `Process`; `current_process_id` from CR3
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-51-checklist.md`

### Scope 52 — Dup FD and CWD-Relative Open

* `DupFd` syscall; per-process `cwd`; relative `OpenFile`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-52-checklist.md`

### Scope 53 — mprotect and Guard Pages

* `Mprotect` syscall within W^X; stack guard probe
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-53-checklist.md`

### Scope 54 — mmap Bring-Up

* anonymous RW at `0x600000`; read-only file mmap
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-54-checklist.md`

### Scope 55 — User Write Path

* `WritePathProbe` syscall; storage round-trip smoke
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-55-checklist.md`

### Scope 56 — Multiple Shared Libraries

* `/lib/*.elf` search; `libc_stub` + `libaux_stub` mapping
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-56-checklist.md`

### Scope 57 — PLT JUMP_SLOT Relocations

* `R_X86_64_JUMP_SLOT` binding; covered by boot gate `dynamic_runtime`

Checklist: `docs/scope-57-checklist.md`

### Scope 58 — Manifest Digest Trust

* `digest=sha256:` manifest field; SHA-256 verification
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-58-checklist.md`

### Scope 59 — Per-CPU Runqueue Skeleton

* BSP runqueue accounting on preempt; APs remain parked
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-59-checklist.md`

### Scope 60 — Integration Milestone (51–59)

* cumulative validation of scopes 51–59
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-60-checklist.md`

### Scope 61 — chdir and Path Normalization

* `Chdir` syscall; collapse `..` in resolved paths
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-61-checklist.md`

### Scope 62 — munmap

* `Munmap` syscall; TLB shootdown on unmap
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-62-checklist.md`

### Scope 63 — Per-Process VMA Registry

* region list on `Process`; mmap overlap rejection
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-63-checklist.md`

### Scope 64 — Fork-Lite with FD Inheritance

* `ForkLite` syscall; shallow-copy FD table and cwd
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-64-checklist.md`

### Scope 65 — Ring 3 HW Syscall Probes

* hardware `syscall` path for `WritePathProbe` / `Mprotect`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-65-checklist.md`

### Scope 66 — Minimal fcntl Stub

* `F_GETFD` and `F_DUPFD` via `Fcntl` syscall
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-66-checklist.md`

### Scope 67 — Lazy PLT Resolution

* defer `R_X86_64_JUMP_SLOT` until `bind_lazy_plt`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-67-checklist.md`

### Scope 68 — Cross-CPU TLB Shootdown Accounting

* per-CPU shootdown counters on unmap/map flush
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-68-checklist.md`

### Scope 69 — AP Idle Trampoline Accounting

* parked AP idle tick counter under QEMU `-smp 2`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-69-checklist.md`

### Scope 70 — Integration Milestone (61–69)

* cumulative validation of scopes 61–69
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-70-checklist.md`

### Scope 71 — HW `syscall` / `sysret` Return Path

* hardware `syscall` stub with `sysret` back to user
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-71-checklist.md`

### Scope 72 — Ring 3 `chdir` from User

* `Chdir` syscall from Ring 3 HW path with user path pointer
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-72-checklist.md`

### Scope 73 — `munmap` with Length (Partial Unmap)

* `Munmap` `arg1` length; VMA truncate; TLB shootdown on unmap
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-73-checklist.md`

### Scope 74 — `WaitLite` on Fork-Lite Child

* `WaitLite = 78` waits for fork-lite child exit code
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-74-checklist.md`

### Scope 75 — `syscallprobe` User ELF Manifest

* `/bin/syscallprobe` manifest; HW syscall probes
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-75-checklist.md`

### Scope 76 — `fcntl` `F_SETFD` / Close-on-Exec

* per-FD flags with `FD_CLOEXEC`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-76-checklist.md`

### Scope 77 — Ring 3 Lazy PLT First Call

* lazy PLT bind under Ring 3 smoke flag
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-77-checklist.md`

### Scope 78 — IPI TLB Shootdown Stub

* logical IPI counters on `request_tlb_shootdown`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-78-checklist.md`

### Scope 79 — AP Idle Trampoline Entry

* AP `hlt` trampoline entry accounting
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-79-checklist.md`

### Scope 80 — Integration Milestone (71–79)

* cumulative validation of scopes 71–79
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-80-checklist.md`

### Scope 81 — Real HW `syscall` / `sysret`

* `HW_SYSRET_REAL` counter; HW probe via `hw-sysret-probe` feature on QEMU boot
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-81-checklist.md`

### Scope 82 — `getcwd` Syscall

* `GetCwd = 79` copies process cwd to user buffer
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-82-checklist.md`

### Scope 83 — `chdirprobe` User ELF

* `/bin/chdirprobe` manifest; `Chdir` + `GetCwd` smoke
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-83-checklist.md`

### Scope 84 — VMA In-Region Split

* middle `munmap` splits VMA registry (`VMA_SPLITS`)
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-84-checklist.md`

### Scope 85 — Fork-Lite CR3 Duplicate

* shallow `fork_duplicate_cr3` for fork-lite child
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-85-checklist.md`

### Scope 86 — `ExecLite` + Close-on-Exec

* `ExecLite = 81` replaces image; sweeps `FD_CLOEXEC` fds
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-86-checklist.md`

### Scope 87 — `PipeLite` Anonymous Pipe

* `Pipe = 80`; ring buffer; read/write on pipe fds
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-87-checklist.md`

### Scope 88 — Ring 3 PLT Fault Lazy Bind

* `#PF` at PLT slot triggers lazy bind under smoke flag
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-88-checklist.md`

### Scope 89 — LAPIC IPI Send Stub

* `LAPIC_IPI_SEND` on TLB shootdown request
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-89-checklist.md`

### Scope 90 — Integration Milestone (81–89)

* cumulative validation of scopes 81–89 counters (no nested re-run)
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-90-checklist.md`

### Scope 91 — Fork-Lite COW Break

* anon page COW break after `fork_lite`; parent/child write isolation smoke
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-91-checklist.md`

### Scope 92 — `PollLite` Syscall

* `Poll = 82` single-fd readiness on pipe fds
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-92-checklist.md`

### Scope 93 — Gap-Aware `mmap` Hint

* `next_anon_hint` fills lowest gap before high-water bump
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-93-checklist.md`

### Scope 94 — `ExecLite` Argv from User

* bounded argv copy from user pointer vector
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-94-checklist.md`

### Scope 95 — `pipeprobe` Ring-3 HW ELF

* `/bin/pipeprobe` seed; HW pipe + `Poll` path under `hw-sysret-probe`
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-95-checklist.md`

### Scope 96 — VMA Adjacent Coalesce

* merge adjacent anon VMAs on munmap boundary
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-96-checklist.md`

### Scope 97 — Work-Stealing Stub

* BSP steals from CPU1 runqueue counter when empty
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-97-checklist.md`

### Scope 98 — AP Runnable Enqueue Stub

* synthetic runnable enqueue on CPU1 without AP scheduler loop
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-98-checklist.md`

### Scope 99 — LAPIC ICR Write Stub

* discard-backed ICR-low write counter (no real MMIO in QEMU tests)
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-99-checklist.md`

### Scope 100 — Integration Milestone (91–99)

* cumulative validation of scopes 91–99 counters (no nested re-run)
* Covered by boot/system gate (see VALIDATION_GATES.md)

Checklist: `docs/scope-100-checklist.md`

### Scopes 101–110 — Constitutional Semantic Foundation (documentation)

Scopes 1–100 answered whether the OS can exist. Scopes 101–110 freeze **semantic constitutionalism**: axioms, rights algebra, temporal visibility, IPC guarantees, governance gates G1–G5, and `clan-semantics-v1` — before native implementation (111+).

Status: complete (validated 2026-05-25 — semantic lint + Scope 110 constitutional smoke)

Checklist index: `docs/scope-101-checklist.md` … `docs/scope-110-checklist.md`

Validation:

```
python scripts/semantic_lint.py
python scripts/gate/boot.py --gate constitutional --timeout 300
```

Key guides:

* [NATIVE_MODEL.md](docs/NATIVE_MODEL.md) — post-Unix capability civilization
* [AXIOMS.md](docs/AXIOMS.md) — constitutional axioms A1–A10 (A7+A10 anti-entropy pair)
* [ROADMAP_POST100.md](docs/ROADMAP_POST100.md) — scopes 101–150 table

### Scopes 111–120 — Capability Foundation (implementation)

Kernel object table, native cap lifecycle (IDs 256+ kernel-only), storage grants, compat path broker, ambient/namespace policy, and cap+compat integration milestone.

Status: complete (validated — `ClanOS-BootGate: name=capabilities ok=true`)

Checklist index: `docs/scope-111-checklist.md` … `docs/scope-120-checklist.md`

Validation:

```
cargo check -p kernel
cargo test -p kernel --features preemption --test preemption_integration
python scripts/gate/boot.py --gate capabilities --timeout 300
```

### Scopes 121–150 — Native platform

Service loaders, brokers, endpoints, service-centric scheduling — see [ROADMAP_POST100.md](docs/ROADMAP_POST100.md).

Status: complete (validated — `ClanOS-BootGate: name=boundary ok=true`)

### Scopes 151–500 — Post-150 roadmap

Epochs 7–20 through milestone **500** (fully operational OS). See:

- [ROADMAP_151_350.md](docs/ROADMAP_151_350.md) — milestones 200–350
- [ROADMAP_351_400.md](docs/ROADMAP_351_400.md) — desktop + userland + network (M400)
- [ROADMAP_401_500.md](docs/ROADMAP_401_500.md) — production SMP, signed ELF, external network (M500)

Status: complete (`SYSTEM_GATE_VERSION = 1.0.0`, unified `system_gate.rs`)

Validation (system gate):

```
python scripts/gate/system.py --gate system --timeout 360
python scripts/gate/system_host.py
python scripts/gate/clan_rt.py
.\scripts\run_desktop.ps1
```

# Documentation

Full index: [`docs/INDEX.md`](docs/INDEX.md) · Gate reference: [`docs/VALIDATION_GATES.md`](docs/VALIDATION_GATES.md)

---

# Project Structure

```
Clan OS
├── Cargo.toml                 workspace manifest
├── docs/                      validation gates, guides, historical checklists (INDEX.md)
├── scripts/                   boot/system gate checks + validation_matrix.py
├── kernel/
│   ├── Cargo.toml             kernel crate manifest
│   ├── x86_64-unknown-none.json
│   ├── src/
│   │   ├── main.rs            kernel entry + boot gate smokes
│   │   ├── lib.rs             modules, init (GDT, IDT, SMP)
│   │   ├── storage.rs         simple persistent filesystem
│   │   ├── security.rs        identity + permission policy
│   │   ├── syscall.rs         syscall IDs + dispatch
│   │   ├── device.rs          device registry + PCI skeleton
│   │   ├── block.rs           block-device manager
│   │   ├── exec_image.rs      ELF64 image validation
│   │   ├── elf_reloc.rs       static + GLOB_DAT relocations
│   │   ├── shared_loader.rs   shared library mapping (scope 41)
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
│   │   ├── mmap.rs            mmap bring-up (Scope 54)
│   │   ├── image_digest.rs    SHA-256 manifest digests (Scope 58)
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

## CI

GitHub Actions ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs `cargo fmt --check` and the full validation matrix on every push and pull request to `main`/`master` (Ubuntu, QEMU; expect roughly 4–6 hours):

```
python scripts/validation_matrix.py --soak-duration 30 --latency-duration 30 --boot-wait 90 --smoke-timeout 180
```

Run Clan OS using QEMU:

```
cargo run -p kernel
```

Run Clan OS in QEMU (interactive shell + desktop):

```
cargo run -p kernel
```

Preemption lab (CI fairness/latency telemetry — does not reach the shell):

```
cargo run -p kernel --features preemption
```

Serial telemetry: `ClanOS-Preemption: name=fairness …` and `name=latency …`

Integration tests:

```
cargo test -p kernel --features preemption --test preemption_integration
```

Preemption validation:

```
python scripts/preemption/soak.py --duration 120 --min-samples 3 --boot-wait 90
python scripts/preemption/latency.py --duration 120 --min-samples 5 --max-latency-ms 100 --boot-wait 90
```

Boot and system gates:

```
python scripts/gate/boot.py --gate boot --timeout 180
python scripts/gate/system.py --gate system --timeout 180
```

Full validation matrix:

```
python scripts/validation_matrix.py --soak-duration 30 --latency-duration 30 --boot-wait 90 --smoke-timeout 180
```

Resume from a specific check:

```
python scripts/validation_matrix.py --from-check boot-gate-check --smoke-timeout 180
```

See [`docs/VALIDATION_GATES.md`](docs/VALIDATION_GATES.md) for boot/system subsystem `--gate` names and scope-index routing.

---

# Vision

Clan OS is an experimental **post-Unix capability system** with **semantic constitutionalism** — not “Linux but smaller.”

Scopes 1–100 built kernel mechanics (paging, ELF, syscalls, SMP groundwork). The long-term challenge is **preserving semantic coherence across decades**, not only shipping features.

**Preserving semantic coherence is harder than building the kernel.**

* **Native:** capabilities, async endpoints, no ambient paths, service-centric design — see [NATIVE_MODEL.md](docs/NATIVE_MODEL.md)
* **Compat:** ELF, FDs, paths, POSIX (future shim) — substrate, not architectural truth
* **Governance:** [AXIOMS.md](docs/AXIOMS.md) (especially A7 semantic laws override convenience, A10 minimization), gates G1–G5, [SEMANTIC_SPECS.md](docs/SEMANTIC_SPECS.md)

**What happens when you build a civilization on the OS on your own terms — and write the laws before the code?**

---

# License

Licensed under the Apache License, Version 2.0.

See [LICENSE](LICENSE) for the full text.



