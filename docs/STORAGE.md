# Storage Design (Phase 7)

AresOS Phase 7 introduced a small persistent storage stack on top of a block-device boundary. Phase 8 mounts that filesystem through a managed block backend so the same filesystem API can run on driver-plumbed storage.

## Layers

```mermaid
flowchart TD
Shell[Shell Commands] --> StorageApi[storage API]
Syscalls[Storage Syscalls] --> StorageApi
Userspace[User Utilities] --> Syscalls
StorageApi --> SimpleFs[SimpleFs]
SimpleFs --> BlockDevice[BlockDevice]
BlockDevice --> MemoryBlockDevice[MemoryBlockDevice]
BlockDevice --> ManagedBlockDevice[ManagedBlockDevice]
ManagedBlockDevice --> BlockManager[Block Manager]
```

## Filesystem Format

- Sector size: 512 bytes
- Header sector: magic, version, file count
- Directory table: fixed-size entries
- File data: one sector per file
- Maximum files: 16
- Maximum file size: 512 bytes
- Maximum path length: 48 bytes

Each write updates file data and flushes the directory/header metadata to the backing block device. Remount validation proves data survives unmount/mount cycles on the same device instance.

## Runtime API

Primary kernel APIs live in `kernel/src/storage.rs`:

- `init()`
- `format()`
- `remount()`
- `list_files()`
- `read_file(path)`
- `create_file(path)`
- `write_file(path, contents)`
- `delete_file(path)`
- `info()`
- `phase7_smoke_check()`
- `phase8_smoke_check()`

## Shell Commands

- `ls`
- `cat <path>`
- `touch <path>`
- `write <path> <text>`
- `rm <path>`
- `mount`
- `format`
- `fsinfo`

## Validation

```bash
python scripts/gate/boot.py --phase 7 --timeout 180
python scripts/validation_matrix.py --soak-duration 30 --latency-duration 30 --boot-wait 90 --smoke-timeout 180
```

Boot validation emits `AresOS-BootGate: name=shell_storage ok=true` (see [VALIDATION_GATES.md](VALIDATION_GATES.md)).

## Phase 8 Backend

By default, runtime storage uses `ManagedBlockDevice`, which delegates sector I/O to the active block backend. Phase 8 registers `qemu-sim-block0` through the block manager as a deterministic driver-backed backend for QEMU validation.

`MemoryBlockDevice` remains available for focused filesystem tests.

## Phases 36, 45–47

- Phase 36 — `ReadFileProbe` / `WriteFileProbe` syscalls copy through validated user buffers.
- Phases 45–46 — FD table maps open files to storage indices (`OpenFile`, `CloseFile`, `ReadFd`, `WriteFd`). See [FILE_DESCRIPTORS.md](FILE_DESCRIPTORS.md).
- Phase 47 — file-backed demand paging reads filesystem pages on user `#PF`. See [DEMAND_PAGING.md](DEMAND_PAGING.md).

File owner/mode metadata and checked APIs were introduced in Phase 10 ([SECURITY.md](SECURITY.md)).

## Deferred Work

- Real AHCI/NVMe/virtio block drivers
- FAT/ext-style filesystem compatibility
- Journaling and crash consistency
- Per-process FD namespaces and mmap-style file mapping
