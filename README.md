# Amazing Adventures: The Lost Tomb Toolkit

[![CI](https://github.com/thomashodgkinson1987/aatlt-toolkit/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/thomashodgkinson1987/aatlt-toolkit/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/thomashodgkinson1987/aatlt-toolkit)](https://github.com/thomashodgkinson1987/aatlt-toolkit/releases/latest)
[![License: MIT](https://img.shields.io/github/license/thomashodgkinson1987/aatlt-toolkit)](LICENSE)
[![Rust edition 2024](https://img.shields.io/badge/rust-edition%202024-orange)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Platforms](https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macOS-blue)](#downloads)

A small cross-platform terminal helper for the Steam title **Amazing Adventures: The
Lost Tomb** (Steam AppID **3510**). It does two things, each fully reversible:

1. **Graphics improvement** — downloads the latest [cnc-ddraw](https://github.com/FunkyFr3sh/cnc-ddraw)
   release and installs it into the game folder. cnc-ddraw is a native DirectDraw
   reimplementation; it fixes the game feeling like ~30 fps and the flickering mouse
   cursor under Proton/Wine.

2. **Splash-screen skip** — shrinks the dwell/fade immediates inside the game payload that
   is embedded in `AmazingAdventures.exe`, so the PopCap → SpinTop → "Amazing Adventures"
   startup splashes flash past and the game goes straight to the menu.

Run it from inside the game folder and pick options from a menu, or use the
non-interactive subcommands.

## Usage

~~~
aatlt-toolkit                 # interactive menu
aatlt-toolkit status
aatlt-toolkit graphics apply
aatlt-toolkit graphics remove
aatlt-toolkit splash apply
aatlt-toolkit splash remove

Options (also work with the menu):
  --dir <PATH>   game folder (default: the current directory)
  -y, --yes      assume "yes"; never prompt
      --offline  do not contact the network (affects `status` only)
      --no-color disable coloured output
~~~

Example — apply both improvements in the game folder:

```
cd "/path/to/steamapps/common/Amazing Adventures The Lost Tomb"
aatlt-toolkit graphics apply
aatlt-toolkit splash apply
```

## Downloads

Prebuilt binaries are published on the [Releases page](https://github.com/thomashodgkinson1987/aatlt-toolkit/releases/latest), named `aatlt-toolkit-<version>-<target>`:

| Platform | Archive |
| --- | --- |
| Windows (x86_64) | `…-x86_64-pc-windows-msvc.zip` |
| Linux (x86_64) | `…-x86_64-unknown-linux-musl.tar.gz` |
| Linux (aarch64) | `…-aarch64-unknown-linux-musl.tar.gz` |
| macOS (Apple Silicon) | `…-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `…-x86_64-apple-darwin.tar.gz` |

Each release also publishes a `SHA256SUMS` file. The Linux binaries are statically linked
against musl, so they run on any distribution. Build from source with the [instructions
below](#building) if you would rather compile it yourself.

> The binaries are unsigned, so macOS may quarantine the download — clear it with
> `xattr -d com.apple.quarantine ./aatlt-toolkit` — and Windows may show a SmartScreen
> warning.

## The Steam launch option (important)

Under Proton/Wine, cnc-ddraw is only actually used if Wine is told to prefer the local
DLL. In Steam: right-click the game → **Properties → Launch Options** and paste:

```
WINEDLLOVERRIDES="ddraw=n,b" %command%
```

The toolkit prints this reminder whenever it installs cnc-ddraw. On native Windows this is
usually unnecessary — the local `ddraw.dll` is loaded automatically.

## What it touches

Everything happens in the game folder you run it from:

| Action | Files |
| --- | --- |
| `graphics apply` | `ddraw.dll`, `ddraw.ini`, `Shaders/…`, `cnc-ddraw config.exe` (extracted from the release zip). A pre-existing `ddraw.ini` is saved as `ddraw.ini.bak` first. |
| `graphics remove` | Deletes the above and restores `ddraw.ini.bak`, if present. |
| `splash apply` | Writes the patched dwell immediates in `AmazingAdventures.exe` and saves a pristine copy as `AmazingAdventures.exe.aatlt-orig`. |
| `splash remove` | Restores the pristine `AmazingAdventures.exe`. |
| any change | Updates a small `.aatlt-toolkit.json` marker recording versions/hashes. |

Notes:

- The splash patch is validated against the exact game build (SHA-256 `a1936133…c34c4`
  stock / `7cf9d759…c57f7` patched); an unrecognised executable is refused rather than
  modified.

- **Steam "Verify integrity of game files" will undo the splash patch** (the EXE is part of
  the Steam manifest). Just run `splash apply` again afterwards.

- The splash patch is **verified at runtime (2026-11-22)**: applied to a live install, the
  splashes flash past and the game lands on a working main menu; `splash remove` restores
  the pristine executable. See the
  [wiki](https://github.com/thomashodgkinson1987/aatlt-wiki) for the full analysis and the
  test plan.

- cnc-ddraw could not be simpler to keep up to date: run `graphics apply` again to fetch the
  newest release.

## Building

Requires a recent stable Rust toolchain (edition 2024).

```
cargo build --release          # native binary
cargo test --all
```

The Windows binary cross-compiles from Linux with the GNU toolchain:

```
rustup target add x86_64-pc-windows-gnu
# e.g. on Arch: sudo pacman -S mingw-w64-gcc
cargo build --release --target x86_64-pc-windows-gnu
```

CI builds and tests Linux, Windows and macOS (see [`.github/workflows/ci.yml`](./.github/workflows/ci.yml)). macOS is currently best-effort: the code is platform-neutral but cnc-ddraw itself is a Windows DLL, so it only makes sense there under Wine/CrossOver.

## How it works

The shipped `AmazingAdventures.exe` is a PopCap DRM/Steam wrapper, not the game: it extracts
the real game (`popcapgame1.exe`) from a PE overlay on every launch and runs it as a child
process whose working directory is the game folder. That is why a local `ddraw.dll` is
picked up, and why the only durable place to patch the splash constants is inside the
wrapper's overlay. The overlay is stored verbatim, so the patch edits the wrapper directly
(see `src/hashes.rs` for the overlay offset and `src/splash.rs` for the site table).

With thanks to [cnc-ddraw](https://github.com/FunkyFr3sh/cnc-ddraw) by FunkyFr3sh.

## AI disclosure

This project is developed with the assistance of an AI coding agent. All changes are
reviewed, built and tested by the maintainer (`cargo test --all`, plus the end-to-end EXE
round-trip), and the maintainer takes full responsibility for the code and its behaviour.

## License

[MIT](./LICENSE)