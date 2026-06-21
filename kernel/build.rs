//! Build ring-3 userland ELFs and embed them for kernel smoke tests.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rustc-cfg=curve25519_dalek_backend=\"serial\"");
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap();

    build_embed_elf(
        repo_root,
        "mendo",
        "mendo",
        "mendo.bin",
        repo_root.join("userland/mendo/src"),
    );
    build_embed_elf(
        repo_root,
        "ring3-io-demo",
        "ring3-io-demo",
        "ring3_io_demo.bin",
        repo_root.join("userland/ring3-io-demo/src"),
    );
    build_embed_elf(
        repo_root,
        "hello-alloc",
        "hello-alloc",
        "hello_alloc.bin",
        repo_root.join("userland/hello-alloc/src"),
    );
    build_embed_elf(
        repo_root,
        "sig-demo",
        "sig-demo",
        "sig_demo.bin",
        repo_root.join("userland/sig-demo/src"),
    );
    rerun_if_changed(repo_root.join("userland/src"));

    println!("cargo:rerun-if-changed=build.rs");

    let corpus = repo_root.join("config/signed_elf_test_corpus");
    for name in ["payload.bin", "manifest.toml", "canonical_body.utf8", "WIRE_FORMAT.txt"] {
        println!("cargo:rerun-if-changed={}", corpus.join(name).display());
    }
    println!(
        "cargo:rerun-if-changed={}",
        repo_root
            .join("config/trust_anchor_epoch450.toml")
            .display()
    );
}

fn build_embed_elf(
    repo_root: &Path,
    package: &str,
    bin_name: &str,
    out_name: &str,
    source_dir: PathBuf,
) {
    let target_elf = repo_root.join(format!(
        "target/x86_64-unknown-none/release/{bin_name}"
    ));
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_bin = out_dir.join(out_name);

    let status = Command::new("cargo")
        .args([
            "build",
            "-p",
            package,
            "--release",
            "--target",
            "x86_64-unknown-none",
        ])
        .current_dir(repo_root)
        .status()
        .unwrap_or_else(|err| panic!("failed to spawn cargo for {package}: {err}"));

    if !status.success() {
        panic!("{package} build failed with status {status}");
    }
    if !target_elf.exists() {
        panic!("{package} ELF missing at {}", target_elf.display());
    }

    std::fs::copy(&target_elf, &out_bin).unwrap_or_else(|err| {
        panic!("copy {out_name}: {err}");
    });

    rerun_if_changed(source_dir);
}

fn rerun_if_changed(dir: PathBuf) {
    if let Ok(read) = std::fs::read_dir(dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                rerun_if_changed(path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
