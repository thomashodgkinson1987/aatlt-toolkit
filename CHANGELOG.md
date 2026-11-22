# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-11-22

### Changed

- Documented the runtime verification of both features (2026-11-22): the splash patch and
  the cnc-ddraw install were exercised on a live install, and the splash patch is no longer
  described as "untested". No functional code change.

## [0.1.0] - 2026-11-22

### Added

- Initial toolkit for *Amazing Adventures: The Lost Tomb* (Steam AppID 3510):
  - `graphics apply` / `graphics remove` — download and install / remove cnc-ddraw
    (a native DirectDraw reimplementation) in the game folder, for smooth rendering
    under Proton/Wine.
  - `splash apply` / `splash remove` — shrink the eight `push 1.25f` dwell immediates
    in the front-end state machine inside `AmazingAdventures.exe` so the startup
    splashes flash past (reversible, hash-validated against the stock build).
  - `status`, an interactive menu, and `--dir` / `--yes` / `--offline` / `--no-color`
    options.
  - A `.aatlt-toolkit.json` marker recording installs, and a pristine backup
    (`AmazingAdventures.exe.aatlt-orig`) written before patching.
  - Cross-platform CI (fmt / clippy / test / build on Linux, Windows, macOS) and a
    release workflow (Linux musl, Windows MSVC, macOS; `SHA256SUMS` + build-provenance
    attestations).
  - An apply/remove round-trip integration test (`tests/real_exe.rs`, gated on
    `AAALT_STOCK_EXE`).

[Unreleased]: https://github.com/thomashodgkinson1987/aatlt-toolkit/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/thomashodgkinson1987/aatlt-toolkit/releases/tag/v0.1.1
[0.1.0]: https://github.com/thomashodgkinson1987/aatlt-toolkit/releases/tag/v0.1.0