#!/usr/bin/env python3
import argparse, os, re, signal, subprocess, sys
PHASE_RE = re.compile(r"Phase22-Cr3:\s+activations=(\d+),\s+restores=(\d+),\s+verify_ok=(true|false)")
def cleanup():
    if os.name == "nt":
        subprocess.run(["taskkill", "/IM", "qemu-system-x86_64.exe", "/F"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
def run_kernel(timeout):
    p = subprocess.Popen(["cargo", "run", "-p", "kernel", "--features", "preemption", "--", "-serial", "stdio", "-display", "none", "-no-reboot"], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, env=os.environ.copy())
    try:
        o, _ = p.communicate(timeout=timeout); return p.returncode or 0, o
    except subprocess.TimeoutExpired:
        p.send_signal(signal.SIGTERM)
        try: o, _ = p.communicate(timeout=5)
        except subprocess.TimeoutExpired: p.kill(); o, _ = p.communicate(timeout=5)
        cleanup(); return 124, o
def ok(output):
    for line in output.splitlines():
        m = PHASE_RE.search(line)
        if m:
            a, r, flag = m.groups()
            return int(a) >= 1 and int(r) >= 1 and flag == "true"
    return False
def main():
    parser = argparse.ArgumentParser(description="Run Phase 22 CR3 smoke check.")
    parser.add_argument("--timeout", type=int, default=120)
    args = parser.parse_args()
    cleanup()
    code, output = run_kernel(args.timeout)
    print(output[-4000:])
    if not ok(output):
        print("FAIL: Phase22 smoke")
        return 1
    print("PASS: Phase 22")
    return 0
if __name__ == "__main__": sys.exit(main())
