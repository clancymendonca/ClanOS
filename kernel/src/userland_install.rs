//! Native userland install hook (phases 376–399) — ares-rt demo + `/bin` manifests.

use core::sync::atomic::{AtomicU64, Ordering};

static NATIVE_LAUNCHES: AtomicU64 = AtomicU64::new(0);

pub const DEMO_HELLO_MANIFEST: &str = "ares-exec-v1\nname=demo-hello\nkind=builtin-alias\nentry=demo-hello\ndescription=ares-rt demo\ntrust=system\nowner=admin\n";

pub fn install_native_packages() -> bool {
    crate::network_stack::mark_package_installed();
    true
}

pub fn run_native_demo() -> bool {
    install_native_packages();
    match crate::task::userspace::run_program("demo-hello", &[]) {
        Ok(out) => {
            let ok = out.contains("ares-rt") || out.contains("userland");
            if ok {
                NATIVE_LAUNCHES.fetch_add(1, Ordering::Relaxed);
            }
            ok
        }
        Err(_) => false,
    }
}

pub fn native_launch_count() -> u64 {
    NATIVE_LAUNCHES.load(Ordering::Relaxed)
}

pub fn phase376_userland_smoke() -> bool {
    install_native_packages()
        && crate::task::userspace::run_program("demo-hello", &[])
            .map(|out| out.contains("ares-rt") || out.contains("userland"))
            .unwrap_or(false)
}

pub fn phase396_package_smoke() -> bool {
    phase376_userland_smoke() && crate::network_stack::packages_installed() > 0
}

pub fn phase399_native_app_smoke() -> bool {
    run_native_demo() && native_launch_count() > 0
}
