//! Known-answer SHA-256 hashes and hashing helpers.
//!
//! The splash patch is validated against the exact game build it was derived from, so a
//! wrong or unrecognised executable is refused rather than corrupted.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::Path;

/// The Steam executable that this toolkit patches.
pub const EXE_NAME: &str = "AmazingAdventures.exe";

/// File offset of the embedded game payload's PE header inside `AmazingAdventures.exe`.
///
/// The wrapper embeds `!popcapdrmprotect!` at `0x194000`; the real game PE begins at
/// `0x1941a2`. All splash-patch offsets below are absolute file offsets in the wrapper.
pub const OVERLAY_OFFSET: usize = 0x194_1a2;

/// SHA-256 of the pristine (stock) `AmazingAdventures.exe`.
pub const STOCK_EXE_SHA256: &str =
    "a1936133a75d750c356a1abb61b55ebe04af3770828533c87a604b2fac1c34c4";

/// SHA-256 of `AmazingAdventures.exe` after the splash patch is applied.
///
/// Predicted by zeroing all eight `push 1.25` dwell immediates to `0.01f` (see
/// `splash::SITES`); **not yet verified at runtime**.
pub const PATCHED_EXE_SHA256: &str =
    "7cf9d759869ad468ff8f17aced01e408f991dc6560be494dd6e238a8716c57f7";

/// Hex-encoded SHA-256 of a byte slice.
pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Hex-encoded SHA-256 of a file's contents.
pub fn sha256_file(path: &Path) -> Result<String> {
    let data = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(sha256_bytes(&data))
}
