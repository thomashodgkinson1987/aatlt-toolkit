//! End-to-end apply/remove round-trip against a real (stock) game executable.
//!
//! This needs the actual `AmazingAdventures.exe`, which is not in the repository, so the
//! test is skipped unless `AAALT_STOCK_EXE` points at a pristine copy — for example a copy
//! saved from a Steam install or from *Verify integrity of game files* (stock SHA-256
//! `a1936133…c34c4`).

use std::path::PathBuf;
use std::process::Command;

use aatlt_toolkit::hashes::{PATCHED_EXE_SHA256, STOCK_EXE_SHA256, sha256_bytes};

fn stock_exe() -> Option<PathBuf> {
    std::env::var_os("AAALT_STOCK_EXE").map(PathBuf::from)
}

fn run_tool(dir: &std::path::Path, args: &[&str]) {
    let status = Command::new(env!("CARGO_BIN_EXE_aatlt-toolkit"))
        .arg("--dir")
        .arg(dir)
        .arg("--yes")
        .args(args)
        .status()
        .expect("running the toolkit binary");
    assert!(
        status.success(),
        "toolkit exited with {status} for {args:?}"
    );
}

#[test]
fn apply_then_remove_roundtrip() {
    let Some(stock) = stock_exe() else {
        eprintln!("skipping: set AAALT_STOCK_EXE to a pristine AmazingAdventures.exe");
        return;
    };
    assert!(stock.is_file(), "{} is not a file", stock.display());

    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join("AmazingAdventures.exe");
    std::fs::copy(&stock, &exe).unwrap();
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        STOCK_EXE_SHA256
    );

    // Apply.
    run_tool(dir.path(), &["splash", "apply"]);
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        PATCHED_EXE_SHA256
    );
    assert!(
        dir.path()
            .join("AmazingAdventures.exe.aatlt-orig")
            .is_file()
    );

    // Applying again is idempotent.
    run_tool(dir.path(), &["splash", "apply"]);
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        PATCHED_EXE_SHA256
    );

    // Remove restores the pristine bytes.
    run_tool(dir.path(), &["splash", "remove"]);
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        STOCK_EXE_SHA256
    );

    // Removing again is a no-op.
    run_tool(dir.path(), &["splash", "remove"]);
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        STOCK_EXE_SHA256
    );
}

#[test]
fn remove_without_backup_reconstructs_pristine() {
    let Some(stock) = stock_exe() else {
        eprintln!("skipping: set AAALT_STOCK_EXE to a pristine AmazingAdventures.exe");
        return;
    };

    let dir = tempfile::tempdir().unwrap();
    let exe = dir.path().join("AmazingAdventures.exe");
    std::fs::copy(&stock, &exe).unwrap();

    run_tool(dir.path(), &["splash", "apply"]);
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        PATCHED_EXE_SHA256
    );

    // Delete the backup so removal has to reconstruct the pristine bytes.
    std::fs::remove_file(dir.path().join("AmazingAdventures.exe.aatlt-orig")).unwrap();

    run_tool(dir.path(), &["splash", "remove"]);
    assert_eq!(
        sha256_bytes(&std::fs::read(&exe).unwrap()),
        STOCK_EXE_SHA256
    );
}
