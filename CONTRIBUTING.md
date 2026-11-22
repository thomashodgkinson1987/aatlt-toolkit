# Contributing

Thanks for taking a look! This is a small tool, but here is how the repository is
organised.

> **AI assistance:** this project is developed with the help of an AI coding agent. All
> changes — human or AI-assisted — go through the same review and testing as described
> below.

## Branching model

- `main` is the default branch and is always releasable.
- Feature work happens on short-lived branches and lands via pull request.
- Tags of the form `vX.Y.Z` trigger the release workflow.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/) so `git-cliff` can
generate the changelog:

```
feat(splash): handle the 5-byte push-immediate site layout
fix(graphics): keep the first ddraw.ini backup on re-install
docs: clarify the untested status of the splash patch
```

## Building and testing

```
cargo build --release
cargo test --all
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

There is an end-to-end test that applies and removes the splash patch against a real
`AmazingAdventures.exe`. Because the game binary is not in the repository, it is skipped
unless you point it at a pristine copy:

```
AAALT_STOCK_EXE=/path/to/AmazingAdventures.exe cargo test --all
```

## What to be careful about

- **Never commit game files.** No `.exe`, `.dll`, `Resource.dll`, `config.txt`,
  `properties/partner.xml`, or extracted payloads. Hashes and offsets belong in the source,
  not binaries.
- **The splash patch offsets are build-specific.** They are validated against a specific
  SHA-256 (`STOCK_EXE_SHA256` in `src/hashes.rs`). If you change the site table, recompute
  the expected `PATCHED_EXE_SHA256` and update the test fixtures.
- **Keep `PATCHED_IMM` non-zero.** The tween builder divides by the duration, so `0.0`
   risks a divide-by-zero. The current value is `0.01f`.
- **Respect the safety rules.** The toolkit writes only inside the game folder you point it
   at, keeps a pristine backup before patching, and refuses to touch an unrecognised EXE.

## License

By contributing, you agree your contributions are licensed under the
[MIT License](./LICENSE).