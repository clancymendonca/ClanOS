#!/usr/bin/env python3
"""Host check: virtio-net TX/RX virtqueue simulation API."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
NET = REPO / "kernel" / "src" / "virtio_net.rs"
STACK = REPO / "kernel" / "src" / "network_stack.rs"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=30)
    _ = ap.parse_args()
    net_text = NET.read_text(encoding="utf-8")
    stack_text = STACK.read_text(encoding="utf-8")
    for needle in ("submit_tx", "poll_rx", "smoke_virtio_net_txrx", "VIRTIO_NET_Q_TX"):
        if needle not in net_text:
            print(f"gate/virtio_net_txrx: missing {needle}", file=sys.stderr)
            return 1
    if "submit_tx" not in stack_text:
        print("gate/virtio_net_txrx: network_stack not wired", file=sys.stderr)
        return 1
    if "send_loopback" not in net_text:
        print("gate/virtio_net_txrx: missing loopback compat", file=sys.stderr)
        return 1
    print("gate/virtio_net_txrx: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
